# Build Resources

Este directorio contiene recursos para el empaquetado de la aplicación.

## Iconos necesarios

Para generar instaladores con iconos personalizados, añade:

- `icon.icns` - Icono para macOS (puede generarse desde PNG de 1024x1024)
- `icon.ico` - Icono para Windows (múltiples resoluciones: 16, 32, 48, 64, 128, 256)
- `icons/` - Directorio con PNGs para Linux (256x256.png, 512x512.png, etc.)

## Herramientas útiles

- [electron-icon-builder](https://www.npmjs.com/package/electron-icon-builder) - Genera todos los formatos desde un PNG
- [iconutil](https://developer.apple.com/library/archive/documentation/GraphicsAnimation/Conceptual/HighResolutionOSX/Optimizing/Optimizing.html) - Herramienta nativa de macOS

## Ejemplo de generación

```bash
npm install -g electron-icon-builder
electron-icon-builder --input=./icon-source.png --output=./build
```
