// Thin binary that delegates to the full-featured LSP implementation in `lsp.rs`.
// This ensures hover, semantic tokens, definition, rename, signature help, etc. are available.
#[tokio::main]
async fn main() {
    eprintln!("[vpy_lsp] launching full LSP server");
    if let Err(e) = vectrex_lang::lsp::run().await {
        eprintln!("[vpy_lsp][fatal] {}", e);
    }
}
