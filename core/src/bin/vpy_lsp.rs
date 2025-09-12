use vectrex_lang::lsp::run_stdio_server;

#[tokio::main]
async fn main() {
    eprintln!("[vpy_lsp] starting LSP server...");
    run_stdio_server().await;
}
