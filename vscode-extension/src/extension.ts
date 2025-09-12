import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext) {
  // Command: compile current .vpy using existing cargo binary
  const compileCmd = vscode.commands.registerCommand('vpy.compileCurrent', async () => {
    const doc = vscode.window.activeTextEditor?.document;
    if (!doc || doc.languageId !== 'vpy') { return; }
    await doc.save();
    const terminal = vscode.window.createTerminal({ name: 'VPy Build' });
    terminal.show(true);
    terminal.sendText(`cargo run --quiet --bin vectrex_lang -- build "${doc.fileName}" --bin`);
  });
  context.subscriptions.push(compileCmd);

  // Simple inline diagnostics via regex while full LSP server is WIP
  const diagCollection = vscode.languages.createDiagnosticCollection('vpy');
  context.subscriptions.push(diagCollection);

  const recomputeDiagnostics = (doc: vscode.TextDocument) => {
    if (doc.languageId !== 'vpy') return;
    const diags: vscode.Diagnostic[] = [];
    for (let i = 0; i < doc.lineCount; i++) {
      const line = doc.lineAt(i).text;
      if (/\bPOLYGON\s+2\b/.test(line)) {
        diags.push(new vscode.Diagnostic(new vscode.Range(i,0,i,line.length), 'POLYGON count 2 genera lista degenerada (usa >=3 o un RECT delgado).', vscode.DiagnosticSeverity.Warning));
      }
    }
    diagCollection.set(doc.uri, diags);
  };
  if (vscode.window.activeTextEditor) {
    recomputeDiagnostics(vscode.window.activeTextEditor.document);
  }
  context.subscriptions.push(vscode.workspace.onDidChangeTextDocument(e => recomputeDiagnostics(e.document)));
  context.subscriptions.push(vscode.workspace.onDidOpenTextDocument(doc => recomputeDiagnostics(doc)));

  // Placeholder LSP startup (external server to be implemented in Rust later)
  const serverModule = context.asAbsolutePath('server-placeholder.js');
  const serverOptions: ServerOptions = { run: { module: serverModule, transport: TransportKind.ipc }, debug: { module: serverModule, transport: TransportKind.ipc, options: { execArgv: ['--nolazy','--inspect=6009'] } } };
  const clientOptions: LanguageClientOptions = { documentSelector: [{ language: 'vpy' }], synchronize: { fileEvents: vscode.workspace.createFileSystemWatcher('**/*.vpy') } };
  client = new LanguageClient('vpyLanguageServer','VPy Language Server (Placeholder)', serverOptions, clientOptions);
  // Do not start yet until real server exists – comment out next line when implementing.
  // client.start();
}

export function deactivate(): Thenable<void> | undefined {
  return client?.stop();
}
