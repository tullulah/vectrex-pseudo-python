/**
 * Persistent storage system for IDE state
 * Stores data in Electron userData directory as JSON files
 * Safer than localStorage which can be cleared by browser/dev tools
 */

import { app } from 'electron';
import { join } from 'path';
import { promises as fs } from 'fs';

let _storageDir: string | null = null;
function getStorageDir(): string {
  if (!_storageDir) {
    _storageDir = join(app.getPath('userData'), 'storage');
  }
  return _storageDir;
}

// Ensure storage directory exists
async function ensureStorageDir() {
  try {
    await fs.mkdir(getStorageDir(), { recursive: true });
  } catch (error) {
    console.error('[Storage] Failed to create storage directory:', error);
  }
}

/**
 * Storage keys - centralized to avoid typos
 */
export const StorageKeys = {
  // AI Assistant
  PYPILOT_CONVERSATION: 'pypilot_conversation.json',
  PYPILOT_PROVIDER: 'pypilot_provider.json',
  PYPILOT_CONCISE: 'pypilot_concise.json',
  PYPILOT_CONFIG_PREFIX: 'pypilot_config_', // + provider type + .json
  
  // Editor state
  EDITOR_PERSISTENCE: 'editor_state.json',
  DOCK_LAYOUT: 'dock_layout.json',
  DOCK_HIDDEN_PANELS: 'dock_hidden.json',
  DOCK_PINNED_PANELS: 'dock_pinned.json',
  
  // Emulator
  EMU_BACKEND: 'emu_backend.json',
  
  // Logging
  LOG_CONFIG: 'log_config.json',
  
  // Recent files (already implemented)
  RECENTS: 'recent.json',
} as const;

/**
 * Read data from persistent storage
 */
export async function storageGet<T = any>(key: string): Promise<T | null> {
  await ensureStorageDir();
  
  try {
    const filePath = join(getStorageDir(), key);
    const data = await fs.readFile(filePath, 'utf8');
    return JSON.parse(data);
  } catch (error) {
    // File doesn't exist or parse error - return null
    if ((error as any).code === 'ENOENT') {
      return null;
    }
    console.error(`[Storage] Error reading ${key}:`, error);
    return null;
  }
}

/**
 * Write data to persistent storage
 */
export async function storageSet(key: string, value: any): Promise<boolean> {
  await ensureStorageDir();
  
  try {
    const filePath = join(getStorageDir(), key);
    await fs.writeFile(filePath, JSON.stringify(value, null, 2), 'utf8');
    return true;
  } catch (error) {
    console.error(`[Storage] Error writing ${key}:`, error);
    return false;
  }
}

/**
 * Delete data from persistent storage
 */
export async function storageDelete(key: string): Promise<boolean> {
  await ensureStorageDir();
  
  try {
    const filePath = join(getStorageDir(), key);
    await fs.unlink(filePath);
    return true;
  } catch (error) {
    if ((error as any).code === 'ENOENT') {
      return true; // Already doesn't exist
    }
    console.error(`[Storage] Error deleting ${key}:`, error);
    return false;
  }
}

/**
 * List all storage keys
 */
export async function storageKeys(): Promise<string[]> {
  await ensureStorageDir();
  
  try {
    const files = await fs.readdir(getStorageDir());
    return files.filter(f => f.endsWith('.json'));
  } catch (error) {
    console.error('[Storage] Error listing keys:', error);
    return [];
  }
}

/**
 * Clear all storage (for debugging/reset)
 */
export async function storageClear(): Promise<boolean> {
  try {
    const keys = await storageKeys();
    await Promise.all(keys.map(key => storageDelete(key)));
    return true;
  } catch (error) {
    console.error('[Storage] Error clearing storage:', error);
    return false;
  }
}

/**
 * Get storage directory path (for debugging)
 */
export function getStoragePath(): string {
  return getStorageDir();
}

/**
 * Migrate data from localStorage to persistent storage
 * Call this once on app startup to migrate existing data
 */
export async function migrateFromLocalStorage(localStorageData: Record<string, string>): Promise<void> {
  console.log('[Storage] Migrating from localStorage...');
  
  const migrations: Record<string, string> = {
    'pypilot_conversation': StorageKeys.PYPILOT_CONVERSATION,
    'pypilot_provider': StorageKeys.PYPILOT_PROVIDER,
    'pypilot_concise': StorageKeys.PYPILOT_CONCISE,
    'vpy_editor_docs': StorageKeys.EDITOR_PERSISTENCE,
    'vpy_dock_layout': StorageKeys.DOCK_LAYOUT,
    'vpy_dock_hidden': StorageKeys.DOCK_HIDDEN_PANELS,
    'vpy_pinned_panels_v1': StorageKeys.DOCK_PINNED_PANELS,
    'emu_backend': StorageKeys.EMU_BACKEND,
    'vpy_log_config': StorageKeys.LOG_CONFIG,
  };
  
  let migrated = 0;
  
  for (const [oldKey, newKey] of Object.entries(migrations)) {
    if (localStorageData[oldKey]) {
      try {
        const value = JSON.parse(localStorageData[oldKey]);
        await storageSet(newKey, value);
        migrated++;
        console.log(`[Storage] Migrated: ${oldKey} → ${newKey}`);
      } catch (error) {
        console.error(`[Storage] Failed to migrate ${oldKey}:`, error);
      }
    }
  }
  
  // Migrate provider-specific configs
  for (const [key, value] of Object.entries(localStorageData)) {
    if (key.startsWith('pypilot_config_')) {
      const provider = key.replace('pypilot_config_', '');
      const newKey = `${StorageKeys.PYPILOT_CONFIG_PREFIX}${provider}.json`;
      try {
        const parsed = JSON.parse(value);
        await storageSet(newKey, parsed);
        migrated++;
        console.log(`[Storage] Migrated: ${key} → ${newKey}`);
      } catch (error) {
        console.error(`[Storage] Failed to migrate ${key}:`, error);
      }
    }
  }
  
  console.log(`[Storage] Migration complete: ${migrated} items migrated`);
}
