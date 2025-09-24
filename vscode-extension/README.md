# VPy Vectrex Tools

Extensión VS Code para archivos `.vpy` (pseudo-Python para Vectrex).

## Funciones actuales
- Resaltado de sintaxis (gramática TextMate `vpy.tmLanguage.json`).
- Cliente LSP que lanza el binario Rust `vpy_lsp` (diagnósticos y autocompletado básico).
- Comando: `VPy: Compile Current File` (`vpy.compileCurrent`).

## No implementado todavía
Hover, ir a definición, referencias, semantic tokens, format on save.

## Requisitos
- Node.js (>=18 recomendado)
- Rust toolchain (para compilar el servidor LSP) `cargo build --bin vpy_lsp`

## Desarrollo (modo F5)
1. Abre la carpeta `vscode-extension` en VS Code (o el mono-repo completo).
2. `npm install`
3. F5 (Extension Development Host) y abre un `.vpy`.

## Empaquetar (.vsix)
Genera un archivo instalable local.

```powershell
cd vscode-extension
npm install
# Construye antes (el script prepublish también lo hará)
npm run build
# Empaquetar con vsce (una sola vez puedes usar npx)
npx vsce package
# Salida: vpy-vectrex-tools-0.0.1.vsix
```

Instalar:
1. En VS Code abre la vista de extensiones.
2. Menú de tres puntos ▸ "Install from VSIX...".
3. Selecciona el `.vsix` generado.
4. Recarga la ventana si te lo pide.

## Actualizar versión
Edita `package.json` campo `version` (semver) antes de volver a empaquetar.

## Publicar (opcional futuro)
1. Crea un publisher en https://aka.ms/vscode-create-publisher
2. `npm install -g @vscode/vsce`
3. `vsce login <publisher>`
4. `vsce publish minor` (o patch / major).

## .vscodeignore
Se usa para reducir el peso del paquete (ya incluido) excluyendo artefactos que no hacen falta.

## Troubleshooting
| Problema | Solución |
|----------|----------|
| No arranca LSP | Asegúrate de haber compilado `cargo build --bin vpy_lsp` y que el binario exista en `target/debug/` |
| Sin coloreado | Verifica que la extensión esté activada (abre un `.vpy`) |
| Autocompletado vacío | Escribe una letra tras invocar Ctrl+Espacio; lista son keywords estáticas |

## Próximos pasos sugeridos
- Hover con documentación de comandos vectoriales.
- Go To Definition para `vectorlist` y símbolos locales.
- Semantic tokens para coloreado más rico.
- Formateador básico (indentación bloques).
