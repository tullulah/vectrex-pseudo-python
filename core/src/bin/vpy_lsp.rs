use vectrex_lang::lsp::run_stdio_server;

#[tokio::main]
async fn main() {
    run_stdio_server().await;
}
