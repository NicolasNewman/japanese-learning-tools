use anyhow::{Context, Result};
use arboard::Clipboard;
use chrono::Local;
use env_logger::Builder;
use log::{error, info, LevelFilter};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufWriter, Read, stdin, stdout, Write};
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct IncomingMessage {
    text: String,
}

#[derive(Serialize, Debug)]
struct OutgoingMessage {
    success: bool,
    message: String,
}

fn read_message() -> Result<IncomingMessage> {
    // Native messaging protocol uses a 4-byte length prefix (u32, native endian)
    let mut length_bytes = [0u8; 4];
    stdin().read_exact(&mut length_bytes)
        .context("Failed to read message length")?;
    
    let length = u32::from_ne_bytes(length_bytes) as usize;
    
    // Read the JSON message of the specified length
    let mut buffer = vec![0u8; length];
    stdin().read_exact(&mut buffer)
        .context("Failed to read message content")?;
    
    let message: IncomingMessage = serde_json::from_slice(&buffer)
        .context("Failed to parse message as JSON")?;
    
    Ok(message)
}

fn write_message(message: &OutgoingMessage) -> Result<()> {
    let json = serde_json::to_vec(message)
        .context("Failed to serialize response")?;
    
    // Write message length as 4-byte prefix
    let length_bytes = (json.len() as u32).to_ne_bytes();
    stdout().write_all(&length_bytes)
        .context("Failed to write message length")?;
    
    // Write the JSON message
    stdout().write_all(&json)
        .context("Failed to write message content")?;
    
    // Flush to ensure message is sent immediately
    stdout().flush()
        .context("Failed to flush stdout")?;
    
    Ok(())
}

fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .context("Failed to initialize clipboard")?;
    
    clipboard.set_text(text)
        .context("Failed to set clipboard text")?;
    
    Ok(())
}

fn get_log_file_path() -> Result<PathBuf> {
    let log_filename = "browser_native_messenger.log";
    
    // Get the temp directory (platform specific)
    let temp_dir = std::env::temp_dir();
    let log_path = temp_dir.join(log_filename);
    
    // Make sure parent directory exists
    if let Some(parent) = log_path.parent() {
        std::fs::create_dir_all(parent)
            .context("Failed to create parent directories for log file")?;
    }
    
    Ok(log_path)
}

fn process_message(message: IncomingMessage) -> Result<()> {
    info!("Received message: {}", message.text);
    
    // Copy the text to clipboard
    copy_to_clipboard(&message.text)?;
    
    // Send success response
    let response = OutgoingMessage {
        success: true,
        message: format!("Copied {} characters to clipboard", message.text.len()),
    };
    
    write_message(&response)?;
    
    Ok(())
}

fn main() -> Result<()> {
    // Initialize logger to write to a file
    let log_path = get_log_file_path()?;
    
    // Create a file logger that overwrites the file each time
    let file = File::create(&log_path).context("Failed to create log file")?;
    let buf_writer = BufWriter::new(file);
    
    // Build a custom logger
    let mut builder = Builder::new();
    builder.target(env_logger::Target::Pipe(Box::new(buf_writer)))
           .filter_level(LevelFilter::Info)
           .format(|buf, record| {
               writeln!(buf, 
                       "{} [{}] - {}", 
                       Local::now().format("%Y-%m-%d %H:%M:%S"),
                       record.level(),
                       record.args())
           })
           .init();
    
    info!("Native messaging host started (logging to {})", log_path.display());
    
    // Process messages in a loop
    loop {
        match read_message() {
            Ok(message) => {
                if let Err(e) = process_message(message) {
                    error!("Error processing message: {:#}", e);
                    
                    // Send error response
                    let error_response = OutgoingMessage {
                        success: false,
                        message: format!("Error: {}", e),
                    };
                    
                    if let Err(e) = write_message(&error_response) {
                        error!("Failed to send error response: {:#}", e);
                    }
                }
            },
            Err(e) => {
                // If stdin is closed, this likely means the browser disconnected
                if e.root_cause().downcast_ref::<io::Error>()
                   .map_or(false, |io_err| io_err.kind() == io::ErrorKind::UnexpectedEof) {
                    info!("Browser disconnected, exiting");
                    break;
                }
                
                error!("Error reading message: {:#}", e);
                
                // Try to send error response
                let error_response = OutgoingMessage {
                    success: false,
                    message: format!("Error: {}", e),
                };
                
                if let Err(e) = write_message(&error_response) {
                    error!("Failed to send error response: {:#}", e);
                }
            }
        }
    }
    
    Ok(())
}

