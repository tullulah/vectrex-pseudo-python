# Backup Directory

Esta carpeta es para **archivos temporales de trabajo** que NO deben estar en git.

## ⚠️ Nota Importante

**NO necesitas hacer backup de archivos del proyecto** - todo está versionado en git:
- ✅ `bios.bin` - Ya está en git (`ide/frontend/src/assets/bios.bin`)
- ✅ Código fuente - Todo en git
- ✅ Configuraciones - package.json, Cargo.toml, etc. en git

## Uso Legítimo

Esta carpeta es útil para:

### Archivos de Trabajo Temporal
- Compilaciones experimentales (.bin, .asm)
- Debug outputs (.pdb con datos sensibles)
- Snapshots de estado durante debugging
- ROMs de prueba personales

### Ejemplo
```bash
# Guardar output experimental:
Copy-Item build\experiment.bin backup\experiment_nov15.bin

# Comparar con versión anterior:
fc /b backup\experiment_nov14.bin backup\experiment_nov15.bin
```

## ⚠️ Nota Importante

Esta carpeta está en `.gitignore` para evitar subir archivos binarios grandes al repositorio.

Los archivos aquí son **locales y temporales** - no se sincronizan con GitHub.
