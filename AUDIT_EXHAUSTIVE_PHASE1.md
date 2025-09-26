# AUDITORÍA EXHAUSTIVA 1:1 - FASE 1: MEMORY OPERATIONS

## 🚨 OBJETIVO: VERIFICACIÓN LÍNEA POR LÍNEA SIN EXCEPCIONES

**Fecha**: 2025-09-26  
**Status**: INICIANDO AUDITORÍA COMPLETA  
**Metodología**: Comparación exhaustiva Vectrexy vs emulator_v2

---

## 📋 CATEGORÍA 1: MEMORY OPERATIONS - NEG

### 🔍 **NEG (0x00, 0x40, 0x50, 0x60, 0x70)**

#### **VECTREXY ORIGINAL** (`Cpu.cpp` líneas 467-470):
```cpp
void OpNEG(uint8_t& value) {
    // Negating is 0 - value
    value = SubtractImpl(0, value, 0, CC);
}
```

#### **VERIFICACIÓN emulator_v2**:
PENDIENTE - Necesito localizar implementación exacta

---

## 📋 CATEGORÍA 1: MEMORY OPERATIONS - COM

### 🔍 **COM (0x03, 0x43, 0x53, 0x63, 0x73)**

#### **VECTREXY ORIGINAL** (`Cpu.cpp` líneas 590-596):
```cpp
void OpCOM(uint8_t& value) {
    value = ~value;
    CC.Negative = CalcNegative(value);
    CC.Zero = CalcZero(value);
    CC.Overflow = 0;
    CC.Carry = 1;
}
```

#### **VERIFICACIÓN emulator_v2**:
PENDIENTE - Necesito localizar implementación exacta

---

## 🚨 METODOLOGÍA SISTEMÁTICA

### Paso 1: Extraer TODAS las implementaciones Vectrexy
### Paso 2: Localizar TODAS las implementaciones emulator_v2  
### Paso 3: Comparación línea por línea SIN excepciones
### Paso 4: Documentar CADA discrepancia

---

*INICIANDO EXTRACCIÓN SISTEMÁTICA...*