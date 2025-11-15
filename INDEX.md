# Índice de Documentación - vectrex-pseudo-python

**Guía completa de navegación de la documentación del proyecto**

---

## 🚀 Empezando

### Para Nuevos Usuarios
1. **[SETUP.md](SETUP.md)** - 📦 **Configuración desde cero**
   - Instalación de herramientas (Rust, Node.js, WASM)
   - Compilación de componentes
   - Verificación del entorno
   - Troubleshooting común
   - **🎯 EMPIEZA AQUÍ si es tu primera vez**

2. **[README.md](README.md)** - 📘 Introducción general
   - Quick start
   - Estado actual del proyecto
   - Características principales
   - Ejecución de la IDE

### Para Desarrolladores
3. **[COMPILER_STATUS.md](COMPILER_STATUS.md)** - 🔧 Estado del compilador
   - Instrucciones MC6809 implementadas (63+ opcodes)
   - Arquitectura del ensamblador nativo (PRE-PASS/PASS1/PASS2)
   - Roadmap de implementación
   - Backlog priorizado
   - **Changelog actualizado (Nov 15, 2025)**

4. **[SUPER_SUMMARY.md](SUPER_SUMMARY.md)** - 📚 Documentación técnica completa
   - Arquitectura detallada del emulador
   - Memory map y dispositivos
   - CPU 6809 implementación
   - Sistema de vectores e integrador
   - Timing y ciclos
   - **Referencia técnica definitiva**

---

## 📂 Documentación por Categoría

### Compilador y Lenguaje VPy
- **[COMPILER_STATUS.md](COMPILER_STATUS.md)** - Estado completo del compilador
  - Lexer, parser, AST
  - Pipeline de optimización
  - Backend M6809
  - Ensamblador nativo (arquitectura de 3 fases)
  - 23+ instrucciones implementadas en Nov 2025
  
- **[VPY_AUTHORSHIP.md](VPY_AUTHORSHIP.md)** - Autoría del lenguaje VPy
- **[VPY_RESERVED_WORDS.md](VPY_RESERVED_WORDS.md)** - Palabras reservadas
- **[SYNTAX_UNIFICATION_COMPLETE.md](SYNTAX_UNIFICATION_COMPLETE.md)** - Unificación sintáctica

### Emulador
- **[SUPER_SUMMARY.md](SUPER_SUMMARY.md)** - Documentación técnica del emulador
  - Secciones 1-32: Arquitectura completa
  - CPU 6809, memory map, VIA 6522
  - Sistema de vectores e integrador
  - Instrucciones ilegales y edge cases
  
- **[docs/TIMING.md](docs/TIMING.md)** - Modelo de timing determinista
  - `cycle_frame` (autoridad)
  - `bios_frame` (observacional)
  - Acumulación de ciclos
  - Sincronización timers VIA
  
- **[docs/VECTOR_MODEL.md](docs/VECTOR_MODEL.md)** - Backend de vectores
  - Integrador analógico simplificado
  - Fusión de segmentos
  - Auto-drain
  - Métricas expuestas

### WASM y Migración
- **[MIGRATION_WASM.md](MIGRATION_WASM.md)** - Migración a emulador WASM
  - Retirada del emulador TypeScript
  - API WASM actual
  - Estado histórico (completado)

### Setup y Desarrollo
- **[SETUP.md](SETUP.md)** - 📦 **Setup completo desde cero**
  - Requisitos del sistema
  - Instalación paso a paso
  - Compilación de componentes
  - Verificación
  - Troubleshooting detallado
  - Comandos de referencia

- **[MIGRATION_CHECKLIST.md](MIGRATION_CHECKLIST.md)** - 🔄 **Checklist de migración**
  - Guía paso a paso para cambio de máquina
  - Backup de archivos críticos (BIOS)
  - Verificación post-migración
  - Troubleshooting específico de migración
  - **Úsalo cuando cambies de equipo**

### Progreso y Planificación
- **[CHANGELOG.md](CHANGELOG.md)** - Historial de cambios
- **TODO List** (ver sección en README.md) - Tareas pendientes

---

## 🎯 Flujos de Trabajo Comunes

### 1. Setup Inicial (Nueva Máquina)
```
SETUP.md → README.md → Compilar y probar
```

### 2. Desarrollo del Compilador
```
COMPILER_STATUS.md → SUPER_SUMMARY.md (Sec. 24-26) → core/src/backend/
```

### 3. Desarrollo del Emulador
```
SUPER_SUMMARY.md → docs/TIMING.md → docs/VECTOR_MODEL.md → emulator/src/
```

### 4. Implementar Nueva Instrucción M6809
```
COMPILER_STATUS.md (ver pendientes) → core/src/backend/asm_to_binary.rs → 
core/src/backend/m6809_binary_emitter.rs → cargo test
```

### 5. Troubleshooting
```
SETUP.md (Troubleshooting) → README.md → SUPER_SUMMARY.md (sección relevante)
```

---

## 📊 Estado de Documentación

| Documento | Estado | Última Actualización | Completitud |
|-----------|--------|---------------------|-------------|
| SETUP.md | ✅ Completo | Nov 15, 2025 | 100% |
| README.md | ✅ Actualizado | Nov 15, 2025 | 95% |
| COMPILER_STATUS.md | ✅ Actualizado | Nov 15, 2025 | 95% |
| SUPER_SUMMARY.md | ✅ Completo | Sept 2025 | 98% |
| docs/TIMING.md | ✅ Completo | Sept 2025 | 100% |
| docs/VECTOR_MODEL.md | ✅ Completo | Sept 2025 | 100% |
| MIGRATION_WASM.md | ✅ Histórico | Sept 2025 | 100% |
| VPY_*.md | ⚠️ Pendiente revisión | 2025 | 80% |

---

## 🔍 Búsqueda Rápida

### ¿Cómo hacer...?

**Compilar un programa VPy:**
```bash
# Ver: SETUP.md sección "Verificación del Setup"
cargo build --bin vectrexc
./target/debug/vectrexc build --bin programa.vpy
```

**Añadir nueva instrucción MC6809:**
```
1. Ver COMPILER_STATUS.md sección 13 (instrucciones pendientes)
2. Editar core/src/backend/asm_to_binary.rs (dispatch + emit_xxx)
3. Editar core/src/backend/m6809_binary_emitter.rs (xxx_immediate, xxx_extended, etc.)
4. cargo test
```

**Entender timing del emulador:**
```
docs/TIMING.md → SUPER_SUMMARY.md sección 8-9
```

**Modificar integrador de vectores:**
```
docs/VECTOR_MODEL.md → SUPER_SUMMARY.md sección 15-16
```

**Debuggear problema de BIOS:**
```
SUPER_SUMMARY.md sección 4-6 (Memory Map, BIOS loading)
```

---

## 📝 Contribuir a la Documentación

### Actualizar Documentación Existente
1. Añadir entrada al Changelog del documento
2. Actualizar fecha "Última Actualización" en este INDEX.md
3. Incrementar versión si aplica

### Crear Nueva Documentación
1. Seguir formato Markdown estándar
2. Añadir entrada a este INDEX.md
3. Crear PR con etiqueta `documentation`

### Convenciones
- **Títulos en español** para docs generales
- **Términos técnicos en inglés** (opcodes, keywords)
- **Código en bloques con syntax highlighting**
- **Emojis para categorización** (📦 setup, 🔧 técnico, 📚 referencia)

---

## 🆘 Ayuda y Soporte

### Recursos Internos
- **SETUP.md**: Troubleshooting común
- **COMPILER_STATUS.md**: Limitaciones conocidas
- **SUPER_SUMMARY.md**: Decisiones de diseño técnicas

### Recursos Externos
- **MC6809 Reference**: [6809.pdf](http://www.maddes.net/m6809pm/sections.htm)
- **Vectrex Wiki**: [vectrex.wikia.com](https://vectrex.fandom.com/)
- **GitHub Issues**: [Reportar problemas](https://github.com/tullulah/vectrex-pseudo-python/issues)

---

## 📅 Última Actualización de Este Índice

**Fecha:** Noviembre 15, 2025  
**Versión:** 1.0  
**Autor:** Sistema de documentación vectrex-pseudo-python

**Próxima revisión programada:** Diciembre 1, 2025
