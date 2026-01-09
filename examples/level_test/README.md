# Level System Test

Proyecto de prueba para el sistema de levels con API encapsulada.

## Características Probadas

### Builtins del Sistema de Levels
- ✅ **LOAD_LEVEL(name)** - Carga un level desde assets/playground/*.vplay
- ✅ **SHOW_LEVEL()** - Dibuja automáticamente todos los objetos de todas las capas (bg, gameplay, fg)
- ✅ **UPDATE_LEVEL()** - Actualiza estado del level (placeholder por ahora)
- ✅ **GET_LEVEL_BOUNDS()** - Obtiene límites del mundo (xMin, xMax, yMin, yMax)

### Assets Incluidos
- **Vectores**: mountain.vec, bubble_large.vec, coin.vec
- **Level**: test_level.vplay con 3 objetos (1 background, 2 gameplay)

## Compilación

```bash
# Desde el directorio raíz del proyecto
./target/debug/vectrexc build examples/level_test/src/main.vpy --bin

# Resultado esperado
✓ Phase 6 SUCCESS: Binary generation complete
✓ Assembler generated: 2223 bytes
```

## Estructura del Proyecto

```
level_test/
├── level_test.vpyproj      # Configuración del proyecto
├── src/
│   └── main.vpy            # Código fuente (25 líneas)
└── assets/
    ├── vectors/            # Sprites vectoriales
    │   ├── mountain.vec    # Triángulo de fondo
    │   ├── bubble_large.vec # Octágono de enemigo
    │   └── coin.vec        # Octágono de coleccionable
    └── playground/         # Definiciones de levels
        └── test_level.vplay # Level de prueba con 3 objetos
```

## API Encapsulada - Ejemplo de Uso

```python
def main():
    SET_INTENSITY(127)
    LOAD_LEVEL("test_level")  # Cargar una vez al inicio

def loop():
    WAIT_RECAL()
    
    # Título
    PRINT_TEXT(-90, 100, "LEVEL SYSTEM TEST")
    
    # ¡Todo el level se dibuja automáticamente!
    SHOW_LEVEL()              # Dibuja bg + gameplay + fg
    
    # Actualizar estado (física, animaciones)
    UPDATE_LEVEL()            # Placeholder por ahora
```

## Ventajas del Sistema

1. **Encapsulación completa**: Un solo call (`SHOW_LEVEL()`) dibuja todo
2. **Cero overhead manual**: No necesitas iterar objetos ni llamar BIOS
3. **Multi-capa automática**: Dibuja background, gameplay y foreground en orden
4. **Data-driven**: Levels definidos en JSON, fáciles de editar sin recompilar

## Datos Técnicos

- **Tamaño binario**: 2223 bytes (2.2 KB)
- **Código VPy**: 25 líneas (vs 42 líneas con drawing manual)
- **RAM usada**: LEVEL_PTR, LEVEL_BG_PTR, LEVEL_GAMEPLAY_PTR, LEVEL_FG_PTR (8 bytes)
- **Función runtime**: SHOW_LEVEL_RUNTIME (~70 líneas ASM, emitida automáticamente)

## Próximos Pasos

- [ ] Implementar UPDATE_LEVEL con física/animaciones
- [ ] Añadir capa foreground (actualmente solo bg + gameplay)
- [ ] Soportar propiedades personalizadas de objetos
- [ ] Colisiones y eventos de gameplay
