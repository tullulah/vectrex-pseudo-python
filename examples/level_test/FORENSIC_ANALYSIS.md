# ANÁLISIS FORENSE: DRAW_VECTOR vs SHOW_LEVEL
## Comparación paso a paso de ejecución

---

## TEST 1: DRAW_VECTOR (Control - Funciona)

### Código VPy:
```python
def loop():
    PRINT_TEXT(-90, 100, "DIRECT VECTOR")
    DRAW_VECTOR("mountain", 0, 0)
```

### Análisis de código ASM generado:

