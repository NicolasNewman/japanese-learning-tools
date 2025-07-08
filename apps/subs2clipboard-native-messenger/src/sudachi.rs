use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

// Daemon communication structures
#[derive(Debug, Serialize)]
pub struct SudachiRequest {
    pub text: String,
    #[serde(default)]
    pub debug: bool,
}

#[derive(Debug, Deserialize)]
pub struct SudachiResponse {
    pub status: String,
    pub result: Option<String>,
    pub message: Option<String>,
}

// Sudachi daemon manager
pub struct SudachiDaemon {
    child: Child,
    stdin: BufWriter<std::process::ChildStdin>,
    stdout: BufReader<std::process::ChildStdout>,
}

impl SudachiDaemon {
    pub fn new() -> Result<Self> {
        info!("Starting Sudachi daemon...");

        let mut child = Command::new("gd-sudachi")
            .arg("--daemon")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to start gd-sudachi daemon")?;

        let stdin = BufWriter::new(child.stdin.take().unwrap());
        let stdout = BufReader::new(child.stdout.take().unwrap());

        info!("Sudachi daemon started successfully");

        Ok(SudachiDaemon {
            child,
            stdin,
            stdout,
        })
    }

    pub fn process_text(&mut self, text: &str) -> Result<String> {
        // Send request as JSON
        let request = SudachiRequest {
            text: text.to_string(),
            debug: false,
        };

        let request_json = serde_json::to_string(&request)?;
        writeln!(self.stdin, "{}", request_json)?;
        self.stdin.flush()?;

        // Read response
        let mut response_line = String::new();
        self.stdout.read_line(&mut response_line)?;

        // Parse JSON response
        let response: SudachiResponse = serde_json::from_str(&response_line.trim())?;

        match response.status.as_str() {
            "success" => Ok(response.result.unwrap_or_default()),
            "error" => {
                let error_msg = response
                    .message
                    .unwrap_or_else(|| "Unknown error".to_string());
                Err(anyhow!("Sudachi error: {}", error_msg))
            }
            _ => Err(anyhow!("Invalid response format")),
        }
    }
}

impl Drop for SudachiDaemon {
    fn drop(&mut self) {
        info!("Shutting down Sudachi daemon...");
        if let Err(e) = self.child.kill() {
            error!("Failed to kill Sudachi daemon: {}", e);
        }
        if let Err(e) = self.child.wait() {
            error!("Failed to wait for Sudachi daemon: {}", e);
        }
    }
}

// Global daemon instance
lazy_static! {
    static ref SUDACHI_DAEMON: Mutex<Option<SudachiDaemon>> = Mutex::new(None);
}

pub fn get_or_create_daemon() -> Result<()> {
    let mut daemon = SUDACHI_DAEMON.lock().unwrap();
    if daemon.is_none() {
        *daemon = Some(SudachiDaemon::new()?);
    }
    Ok(())
}

pub fn process_text_with_daemon(text: &str) -> Result<String> {
    // Ensure daemon is running
    get_or_create_daemon()?;

    // Process text using the daemon
    let output_text = {
        let mut daemon = SUDACHI_DAEMON.lock().unwrap();
        if let Some(ref mut daemon) = daemon.as_mut() {
            daemon.process_text(text)?
        } else {
            return Err(anyhow!("Daemon not available"));
        }
    };

    info!("Sudachi processed text successfully");
    Ok(output_text)
}
