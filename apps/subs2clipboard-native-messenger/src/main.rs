use anyhow::{anyhow, Context, Result};
use arboard::Clipboard;
use chrono::Local;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::str;

#[derive(Debug, Deserialize)]
struct BrowserMessage {
    event: String,
    text: String,
    #[serde(default)]
    id: String,
}

#[derive(Debug, Serialize)]
struct ResponseMessage {
    #[serde(rename = "type")]
    response_type: String,
    text: String,
    id: String,
}

fn setup_logger() -> Result<()> {
    let log_file = std::env::temp_dir().join("subs2clipboard-log");
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_file)
        .context("Failed to open log file")?;

    env_logger::builder()
        .target(env_logger::Target::Pipe(Box::new(file)))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    info!("Logger initialized");
    Ok(())
}

fn read_message_from_stdin() -> Result<BrowserMessage> {
    // Native messaging protocol: first 4 bytes indicate message length
    let mut length_bytes = [0u8; 4];
    io::stdin()
        .read_exact(&mut length_bytes)
        .context("Failed to read message length")?;
    
    let message_length = u32::from_ne_bytes(length_bytes) as usize;
    
    // Read the actual message
    let mut input = vec![0; message_length];
    io::stdin()
        .read_exact(&mut input)
        .context("Failed to read message")?;
    
    // Parse the JSON message
    let message: BrowserMessage = serde_json::from_slice(&input)
        .context("Failed to parse JSON message")?;
    
    Ok(message)
}

fn write_message_to_stdout(message: &ResponseMessage) -> Result<()> {
    let json = serde_json::to_vec(message)
        .context("Failed to serialize response to JSON")?;
    
    let message_length = json.len() as u32;
    let length_bytes = message_length.to_ne_bytes();
    
    // Write the message length followed by the message
    io::stdout().write_all(&length_bytes).context("Failed to write message length")?;
    io::stdout().write_all(&json).context("Failed to write response")?;
    io::stdout().flush().context("Failed to flush stdout")?;
    
    Ok(())
}

fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .context("Failed to initialize clipboard")?;
    
    clipboard
        .set_text(text)
        .context("Failed to set text to clipboard")?;
    
    info!("Copied text to clipboard: {}", text);
    Ok(())
}

fn run_sudachi(text: &str, id: &str) -> Result<()> {
    // Run the gd-sudachi command with the provided text
    let output = Command::new("gd-sudachi")
        .arg(text)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute gd-sudachi command")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("gd-sudachi command failed: {}", stderr);
        return Err(anyhow!("gd-sudachi command failed: {}", stderr));
    }
    
    let output_text = String::from_utf8_lossy(&output.stdout).to_string();
    info!("Sudachi output: {}", output_text);
    
    // Send the output back to the browser
    let response = ResponseMessage {
        response_type: "SUDACHI".to_string(),
        text: output_text,
        id: id.to_string(),
    };
    
    write_message_to_stdout(&response)?;
    
    Ok(())
}

fn main() -> Result<()> {
    // Initialize the logger
    if let Err(e) = setup_logger() {
        eprintln!("Failed to set up logger: {}", e);
    }
    
    info!("Native messenger started");
    
    loop {
        match read_message_from_stdin() {
            Ok(message) => {
                info!("Received message: {:?}", message);
                
                match message.event.as_str() {
                    "COPY_TO_CLIPBOARD" => {
                        if let Err(e) = copy_to_clipboard(&message.text) {
                            error!("Failed to copy to clipboard: {}", e);
                        }
                    },
                    "SUDACHI" => {
                        if let Err(e) = run_sudachi(&message.text, &message.id) {
                            error!("Failed to run sudachi: {}", e);
                        }
                    },
                    _ => {
                        error!("Unknown event type: {}", message.event);
                    }
                }
            },
            Err(e) => {
                // If we can't read from stdin (e.g., browser closed connection), exit
                error!("Error reading message: {}", e);
                break;
            }
        }
    }
    
    info!("Native messenger shutting down");
    Ok(())
}