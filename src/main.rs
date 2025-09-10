mod lexer;    // Lexical analysis
mod ast;      // AST definitions
mod parser;   // Parsing logic
mod codegen;  // Optimization + backend dispatch
mod target;   // Target info & selection
mod backend { pub mod string_literals; pub mod m6809; pub mod arm; pub mod cortexm; } // Backend modules (shared string literal utils)

use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "vectrexc", about = "Pseudo-Python multi-target assembler compiler (prototype)")]
struct Cli { #[command(subcommand)] command: Commands }

#[derive(Subcommand)]
enum Commands {
    Build { input: PathBuf, #[arg(short, long)] out: Option<PathBuf>, #[arg(long, default_value="vectrex")] target: target::Target, #[arg(long, default_value="UNTITLED")] title: String },
    Lex { input: PathBuf },
    Ast { input: PathBuf },
}

// main: parse CLI and dispatch subcommands.
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { input, out, target, title } => build_cmd(&input, out.as_ref(), target, &title),
        Commands::Lex { input } => lex_cmd(&input),
        Commands::Ast { input } => ast_cmd(&input),
    }
}

// read_source: utility to load file contents.
fn read_source(path: &PathBuf) -> Result<String> { Ok(fs::read_to_string(path)?) }

// lex_cmd: dump tokens for a source file.
fn lex_cmd(path: &PathBuf) -> Result<()> { let src = read_source(path)?; let tokens = lexer::lex(&src)?; for t in tokens { println!("{:?}", t); } Ok(()) }

// ast_cmd: pretty-print the parsed AST.
fn ast_cmd(path: &PathBuf) -> Result<()> { let src = read_source(path)?; let tokens = lexer::lex(&src)?; let module = parser::parse(&tokens)?; println!("{:#?}", module); Ok(()) }

// build_cmd: run full pipeline (lex/parse/opt/codegen) and write assembly.
fn build_cmd(path: &PathBuf, out: Option<&PathBuf>, tgt: target::Target, title: &str) -> Result<()> {
    let src = read_source(path)?;
    let tokens = lexer::lex(&src)?;
    let module = parser::parse(&tokens)?;
    if tgt == target::Target::All {
        for ct in target::concrete_targets() {
            let asm = codegen::emit_asm(&module, *ct, &codegen::CodegenOptions { title: title.to_string() });
            let base = path.file_stem().unwrap().to_string_lossy();
            let out_path = out.cloned().unwrap_or_else(|| path.with_file_name(format!("{}-{}.asm", base, ct)));
            fs::write(&out_path, &asm)?;
            eprintln!("Generated: {} (target={})", out_path.display(), ct);
        }
        Ok(())
    } else {
        let asm = codegen::emit_asm(&module, tgt, &codegen::CodegenOptions { title: title.to_string() });
        let out_path = out.cloned().unwrap_or_else(|| path.with_extension("asm"));
        fs::write(&out_path, asm)?;
        eprintln!("Generated: {} (target={})", out_path.display(), tgt);
        Ok(())
    }
}
