#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use std::{process::{Child, Command, Stdio}, sync::Mutex, io::{Write, BufRead, BufReader}};
use tauri::{AppHandle, Manager};

struct LspProcess {
    child: Child,
    stdin: Mutex<std::process::ChildStdin>,
}

static LSP: Lazy<Mutex<Option<LspProcess>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
fn lsp_start(app: AppHandle) -> Result<(), String> {
    let mut guard = LSP.lock().unwrap();
    if guard.is_some() { return Ok(()); }

    // Determine executable name (debug build path). We rely on PATH resolution within workspace target/debug.
    // In packaged builds, binary will live next to the main exe; we attempt relative lookup.
    let exe_name = if cfg!(windows) { "vpy_lsp.exe" } else { "vpy_lsp" };
    let mut cmd = Command::new(exe_name);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("spawn lsp failed: {e}"))?;

    let stdin = child.stdin.take().ok_or("no stdin")?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let app_handle = app.clone();

    // Read stdout lines and forward raw content
    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line { let _ = app_handle.emit_all("lsp://stdout", line); }
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
