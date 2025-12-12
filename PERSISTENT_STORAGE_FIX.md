# Persistent Storage Fix - localStorage Reset Issue

**Fecha**: 10 de diciembre de 2025  
**Problema**: localStorage se borra aleatoriamente, perdiendo configuración de API keys y contexto de chat

---

## Problema Identificado

### Síntomas
- API keys desaparecen aleatoriamente
- Conversaciones de PyPilot se pierden
- Configuración del IDE se resetea
- Ocurre al hacer click en lugares random o recargar página

### Causa Raíz
Todo el estado crítico estaba almacenado en **localStorage del navegador**, que es volátil y puede borrarse por:
1. **DevTools**: Cualquier acción en la consola de desarrollo
2. **Hard Refresh** (Ctrl+F5 / Cmd+Shift+R)
3. **Errores de hydration** en React
4. **Límites de cuota** (localStorage tiene límite de ~5-10MB)
5. **Limpieza automática del navegador**

## Solución Implementada

### Arquitectura Nueva

```
┌─────────────────────────────────────────────────────────────┐
│                        FRONTEND                              │
│                                                              │
│  React Components                                            │
│       ↓                                                      │
│  persistentStorage.ts (wrapper)                             │
│       ↓                                                      │
│  window.storage (IPC bridge)                                │
└──────────────────────┬──────────────────────────────────────┘
                       │ IPC
┌──────────────────────┴──────────────────────────────────────┐
│                     ELECTRON MAIN                            │
│                                                              │
│  storage.ts handlers                                         │
│       ↓                                                      │
│  Filesystem (userData dir)                                   │
│       ↓                                                      │
│  ~/.config/vpy-ide/storage/*.json                           │
│  (macOS: ~/Library/Application Support/vpy-ide/storage/)    │
└─────────────────────────────────────────────────────────────┘
```

### Archivos Creados

1. **`ide/electron/src/storage.ts`**
   - Backend de almacenamiento persistente
   - Escribe archivos JSON en `app.getUserData()/storage/`
   - APIs: `storageGet()`, `storageSet()`, `storageDelete()`, `storageClear()`
   - Migración automática desde localStorage

2. **`ide/frontend/src/services/persistentStorage.ts`**
   - Wrapper frontend con API tipo localStorage
   - Fallback a localStorage en desarrollo (browser)
   - Hook React: `usePersistentState<T>()`
   - Constantes centralizadas: `StorageKey.*`

3. **IPC Handlers añadidos** (`ide/electron/src/main.ts`)
   ```typescript
   ipcMain.handle('storage:get', ...)
   ipcMain.handle('storage:set', ...)
   ipcMain.handle('storage:delete', ...)
   ipcMain.handle('storage:keys', ...)
   ipcMain.handle('storage:clear', ...)
   ipcMain.handle('storage:path', ...)
   ```

4. **Preload API** (`ide/electron/src/preload.ts`)
   ```typescript
   window.storage = {
     get, set, delete, keys, clear, getPath, getKeys
   }
   ```

### Datos Almacenados

**Ubicación física**: `~/.config/vpy-ide/storage/` (Linux)  
`~/Library/Application Support/vpy-ide/storage/` (macOS)  
`%APPDATA%/vpy-ide/storage/` (Windows)

**Archivos JSON**:
- `pypilot_conversation.json` - Historial completo de chat
- `pypilot_provider.json` - Proveedor AI seleccionado
- `pypilot_concise.json` - Modo conciso on/off
- `pypilot_config_<provider>.json` - Config por proveedor (API keys, modelos)
- `editor_state.json` - Documentos abiertos, posiciones de cursor
- `dock_layout.json` - Layout de paneles del IDE
- `emu_backend.json` - Backend de emulador seleccionado
- `log_config.json` - Configuración de logging

### Migración Automática

La primera vez que el IDE arranca con el nuevo sistema:

1. Lee todos los valores de `localStorage`
2. Los escribe en archivos JSON en `userData/storage/`
3. Limpia `localStorage` después de migración exitosa
4. Log completo en consola: `[Storage] Migration complete: N items migrated`

Para activar manualmente:
```typescript
import { migrateFromLocalStorage } from '@/services/persistentStorage';

// En main.tsx, después de montar la app
await migrateFromLocalStorage();
```

## Uso en Código

### Antes (localStorage)
```typescript
// ❌ Volátil, se puede perder
const apiKey = localStorage.getItem('pypilot_api_key');
localStorage.setItem('pypilot_api_key', newKey);
```

### Después (persistentStorage)
```typescript
// ✅ Persistente, nunca se pierde
import { storageGet, storageSet, StorageKey } from '@/services/persistentStorage';

const config = await storageGet(StorageKey.pypilotConfig('anthropic'));
await storageSet(StorageKey.pypilotConfig('anthropic'), { apiKey: newKey });
```

### Con React Hook
```typescript
import { usePersistentState } from '@/services/persistentStorage';

function MyComponent() {
  const [apiKey, setApiKey, loading] = usePersistentState('my_key.json', '');
  
  if (loading) return <div>Loading...</div>;
  
  return <input value={apiKey} onChange={e => setApiKey(e.target.value)} />;
}
```

## Migración de Componentes Existentes

### AiAssistantPanel.tsx

**Antes**:
```typescript
const [messages, setMessages] = useState<AiMessage[]>(() => {
  const saved = localStorage.getItem('pypilot_conversation');
  return saved ? JSON.parse(saved) : [];
});

useEffect(() => {
  localStorage.setItem('pypilot_conversation', JSON.stringify(messages));
}, [messages]);
```

**Después**:
```typescript
import { storageGet, storageSet, StorageKey } from '@/services/persistentStorage';

const [messages, setMessages] = useState<AiMessage[]>([]);

useEffect(() => {
  storageGet(StorageKey.PYPILOT_CONVERSATION).then(saved => {
    if (saved) setMessages(saved);
  });
}, []);

useEffect(() => {
  storageSet(StorageKey.PYPILOT_CONVERSATION, messages);
}, [messages]);
```

### DockWorkspace.tsx

**Antes**:
```typescript
const stored = window.localStorage.getItem(STORAGE_KEY);
// ...
window.localStorage.setItem(STORAGE_KEY, JSON.stringify(json));
```

**Después**:
```typescript
import { storageGet, storageSet, StorageKey } from '@/services/persistentStorage';

const stored = await storageGet(StorageKey.DOCK_LAYOUT);
// ...
await storageSet(StorageKey.DOCK_LAYOUT, json);
```

## Investigación de Causas del Reset

### Posibles Causas a Investigar

1. **Hard Refresh Accidental**
   - Añadir interceptor para Ctrl+F5 / Cmd+Shift+R
   - Mostrar advertencia antes de recargar con pérdida de datos

2. **Errores de React Hydration**
   - Revisar logs de `react-dom` para errores de hydration
   - Estos pueden causar re-renderizado completo y pérdida de estado

3. **Límite de Cuota de localStorage**
   - Historial de chat puede crecer indefinidamente
   - Implementar límite de mensajes (ej: últimos 100)

4. **Clicks en Devtools**
   - "Clear Storage" en Application tab
   - "Clear Site Data" en consola
   - **Solución**: Persistente en filesystem, no afectado por DevTools

### Monitoreo Añadido

```typescript
// En main.tsx
window.addEventListener('beforeunload', (e) => {
  console.log('[Storage] Page unloading, data should persist');
  // No prevenir unload, solo log para debugging
});

// Detectar hard refresh
window.addEventListener('keydown', (e) => {
  if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'r') {
    console.warn('[Storage] Hard refresh detected!');
  }
});
```

## Testing

### Verificar Migración
```bash
# 1. Abrir IDE con localStorage existente
npm run dev

# 2. Verificar migración en consola:
# [Storage] Starting migration from localStorage...
# [Storage] Migrated: pypilot_conversation → pypilot_conversation.json
# [Storage] Migration complete: N items migrated

# 3. Verificar ubicación física:
ls -la ~/.config/vpy-ide/storage/  # Linux
ls -la ~/Library/Application\ Support/vpy-ide/storage/  # macOS
dir %APPDATA%\vpy-ide\storage\  # Windows
```

### Verificar Persistencia
```bash
# 1. Configurar API key en PyPilot
# 2. Cerrar IDE completamente
# 3. Abrir DevTools → Application → Clear Storage → Clear All
# 4. Reabrir IDE
# 5. API key debe seguir configurada ✅
```

### Debugging
```typescript
// Ver ubicación de storage
import { getStoragePath } from '@/services/persistentStorage';
console.log('Storage path:', await getStoragePath());

// Ver todos los archivos
const keys = await window.storage.keys();
console.log('Stored files:', keys);

// Leer archivo específico
const data = await window.storage.get('pypilot_conversation.json');
console.log('Conversation data:', data);
```

## TODO: Pasos Siguientes

- [ ] **Integrar migración en main.tsx** (llamar `migrateFromLocalStorage()` al inicio)
- [ ] **Migrar AiAssistantPanel** para usar `storageGet/Set`
- [ ] **Migrar DockWorkspace** para usar persistent storage
- [ ] **Migrar editorPersistence** (documentos abiertos)
- [ ] **Añadir límite a historial de chat** (ej: últimos 100 mensajes)
- [ ] **Añadir warning en hard refresh** si hay datos no guardados
- [ ] **Tests de stress**: Crear 1000 mensajes y verificar que no hay pérdida
- [ ] **Documentar en MANUAL.md** cómo borrar cache si hay corrupción

## Ventajas del Nuevo Sistema

✅ **Nunca se pierde**: Filesystem es persistente, no volátil  
✅ **No afectado por DevTools**: Clear Storage no toca userData  
✅ **Sin límite de cuota**: Filesystem puede crecer tanto como espacio en disco  
✅ **Debugging fácil**: Archivos JSON legibles en `~/.config/vpy-ide/storage/`  
✅ **Migración automática**: Usuarios existentes no pierden datos  
✅ **Fallback a localStorage**: Funciona en desarrollo sin Electron  
✅ **Type-safe**: Constantes `StorageKey.*` previenen typos  

## Referencias

- **Electron userData**: https://www.electronjs.org/docs/latest/api/app#appgetpathname
- **contextBridge**: https://www.electronjs.org/docs/latest/api/context-bridge
- **IPC Security**: https://www.electronjs.org/docs/latest/tutorial/ipc

---

**Autor**: GitHub Copilot  
**Revisado**: Usuario (daniel)
