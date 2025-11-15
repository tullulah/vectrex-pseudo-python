# Backup Directory

Esta carpeta se utiliza para almacenar copias de seguridad de archivos críticos que **NO están en git**.

## Propósito

Mantener el proyecto **autocontenido** - todos los backups dentro del workspace, no en rutas externas (Desktop, HOME, etc.).

## Archivos Típicos

### bios.bin (8192 bytes)
```bash
# Backup antes de migración:
Copy-Item ide\frontend\dist\bios.bin backup\bios.bin

# Restaurar después de migración:
Copy-Item backup\bios.bin ide\frontend\dist\bios.bin
Copy-Item backup\bios.bin ide\frontend\src\assets\bios.bin
```

### Otros archivos temporales
- Compilaciones de prueba (.bin, .pdb)
- Configuraciones personales
- Snapshots de estado

## ⚠️ Nota Importante

Esta carpeta está en `.gitignore` para evitar subir archivos binarios grandes al repositorio.

Los archivos aquí son **locales y temporales** - no se sincronizan con GitHub.
