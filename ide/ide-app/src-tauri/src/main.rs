#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Child, Command, Stdio},
    sync::Mutex,
};
use tauri::{AppHandle, Manager};

struct LspProcess {
    child: Child,
    stdin: Mutex<std::process::ChildStdin>,
}

impl Drop for LspProcess {
    fn drop(&mut self) {
        // Attempt graceful terminate; on Windows Child::kill sends a terminate signal.
        if let Err(e) = self.child.kill() {
            eprintln!("[LSP] Failed to kill child on drop: {e}");
        }
    }
}

static LSP: Lazy<Mutex<Option<LspProcess>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
fn lsp_start(app: AppHandle) -> Result<(), String> {
    let mut guard = LSP.lock().unwrap();
    if guard.is_some() { return Ok(()); }

    // Determine LSP executable path: attempt sibling of current exe (target/debug) first; fallback to bare name.
    let exe_name = if cfg!(windows) { "vpy_lsp.exe" } else { "vpy_lsp" };
    let lsp_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join(exe_name)))
        .filter(|p| p.exists())
        .unwrap_or_else(|| std::path::PathBuf::from(exe_name));

    if !lsp_path.exists() {
        let msg = format!("LSP binary not found: {:?}", lsp_path);
        let _ = app.emit_all("lsp://stderr", msg.clone());
        return Err(msg);
    }

    let _ = app.emit_all("lsp://stderr", format!("[spawn] LSP path: {:?}", lsp_path));
    let mut cmd = Command::new(&lsp_path);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| {
        let msg = format!("spawn lsp failed: {e}");
        let _ = app.emit_all("lsp://stderr", msg.clone());
        msg
    })?;

    let stdin = child.stdin.take().ok_or("no stdin")?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let app_handle = app.clone();

    // Read stdout with proper LSP framing (Content-Length) and emit complete JSON bodies
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        loop {
            // Parse headers
            let mut content_length: Option<usize> = None;
            loop {
                let mut header_line = String::new();
                match reader.read_line(&mut header_line) {
                    Ok(0) => return,              // EOF
                    Ok(_) => {},
                    Err(_) => return,             // I/O error -> terminate thread
                }
                let trimmed = header_line.trim_end();
                if trimmed.is_empty() { // End of headers
                    break;
                }
                if let Some(rest) = trimmed.strip_prefix("Content-Length:") {
                    content_length = rest.trim().parse::<usize>().ok();
                }
                // Ignore other headers (Content-Type, etc.)
            }

            let len = match content_length { Some(l) => l, None => continue };
            let mut body = vec![0u8; len];
            if reader.read_exact(&mut body).is_err() { return; }
            match String::from_utf8(body) {
                Ok(json) => {
                    let _ = app_handle.emit_all("lsp://message", json.clone());
                    // Also emit raw for debugging (optional)
                    let _ = app_handle.emit_all("lsp://stdout", json);
                }
                Err(_) => {
                    let _ = app_handle.emit_all("lsp://stderr", "<non-utf8 LSP body>".to_string());
                }
            }
        }
    });

    // Read stderr lines
    let app_handle_err = app.clone();
    std::thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line { let _ = app_handle_err.emit_all("lsp://stderr", line); }
        }
    });

    *guard = Some(LspProcess { child, stdin: Mutex::new(stdin) });
    Ok(())
}

#[tauri::command]
fn lsp_send(_app: AppHandle, payload: String) -> Result<(), String> {
    let guard = LSP.lock().unwrap();
    if let Some(proc) = guard.as_ref() {
        let mut handle = proc.stdin.lock().unwrap();
        // Write LSP framing: Content-Length header + CRLF CRLF + payload
        let bytes = payload.as_bytes();
        // If this looks like initialize (method":"initialize) and id == 1, emit hexdump for debugging once.
        if payload.contains("\"method\":\"initialize\"") && payload.contains("\"id\":1") {
            let hex: String = bytes.iter().map(|b| format!("{b:02X} ")).collect();
            println!("[LSP DEBUG] initialize payload len={} hex: {hex}", bytes.len());
        }
        if let Err(e) = write!(handle, "Content-Length: {}\r\n\r\n", bytes.len()) {
            return Err(format!("write header failed: {e}"));
        }
        if let Err(e) = handle.write_all(bytes) { return Err(format!("write body failed: {e}")); }
        if let Err(e) = handle.flush() { return Err(format!("flush failed: {e}")); }
        return Ok(());
    }
    println!("[LSP-SEND] dropped message (LSP not started)");
    Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![lsp_start, lsp_send])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
