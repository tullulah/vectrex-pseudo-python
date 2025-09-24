# DEPRECATED: Tauri implementation

La versión Tauri de la IDE ha sido sustituida por la shell Electron situada en `ide/electron`.

Motivos del cambio:
- Problemas con comportamiento de tabs / WebView2.
- Necesidad de un control más directo sobre integración y debugging.
- Simplicidad para integrar APIs adicionales por IPC (Electron).

Acciones tomadas:
- `run-ide.ps1` ahora lanza Electron + Vite y no arranca Tauri.
- El código permanece temporalmente para referencia y posible comparación de rendimiento.

Plan de retirada:
1. Confirmar que no faltan features exclusivas de Tauri.
2. Migrar cualquier lógica Rust específica (si existiera) al backend general.
3. Eliminar el directorio tras dos ciclos de verificación.

Si necesitas reactivar Tauri manualmente, aún puedes ejecutar desde `ide/ide-app`:
```
cargo run --manifest-path src-tauri/Cargo.toml
```
(Siempre que las dependencias estén presentes.)
