// Sistema de logging centralizado para el IDE
// Permite control granular de qué se muestra en consola

export type LogLevel = 'error' | 'warn' | 'info' | 'debug' | 'verbose';
export type LogCategory = 'LSP' | 'Build' | 'File' | 'Save' | 'Compilation' | 'App' | 'HMR' | 'Dock' | 'Project' | 'AI';

interface LogConfig {
  level: LogLevel;
  categories: Set<LogCategory>;
  enabled: boolean;
}

// Configuración por defecto - solo errores y warnings importantes
const defaultConfig: LogConfig = {
  level: 'warn',
  categories: new Set(['Build', 'File', 'Save', 'App', 'HMR']),
  enabled: true
};

let config = { ...defaultConfig };

// Niveles de prioridad para filtrado
const levelPriority: Record<LogLevel, number> = {
  error: 0,
  warn: 1,
  info: 2,
  debug: 3,
  verbose: 4
};

// Configuración desde localStorage o defaults
export function initLogging() {
  try {
    const stored = localStorage.getItem('vpy_log_config');
    if (stored) {
      const parsed = JSON.parse(stored);
      config = {
        level: parsed.level || defaultConfig.level,
        categories: new Set(parsed.categories || Array.from(defaultConfig.categories)),
        enabled: parsed.enabled !== undefined ? parsed.enabled : defaultConfig.enabled
      };
    }
  } catch (e) {
    console.warn('[Logger] Failed to load config, using defaults');
  }
}

// Guardar configuración
export function saveLogConfig() {
  try {
    localStorage.setItem('vpy_log_config', JSON.stringify({
      level: config.level,
      categories: Array.from(config.categories),
      enabled: config.enabled
    }));
  } catch (e) {
    console.warn('[Logger] Failed to save config');
  }
}

// Cambiar configuración dinámicamente
export function setLogLevel(level: LogLevel) {
  config.level = level;
  saveLogConfig();
}

export function toggleCategory(category: LogCategory, enabled?: boolean) {
  if (enabled === undefined) {
    enabled = !config.categories.has(category);
  }
  
  if (enabled) {
    config.categories.add(category);
  } else {
    config.categories.delete(category);
  }
  saveLogConfig();
}

export function setLoggingEnabled(enabled: boolean) {
  config.enabled = enabled;
  saveLogConfig();
}

// Función principal de logging
function log(level: LogLevel, category: LogCategory, message: string, ...args: any[]) {
  if (!config.enabled) return;
  if (!config.categories.has(category)) return;
  if (levelPriority[level] > levelPriority[config.level]) return;

  const prefix = `[${category}]`;
  const method = level === 'error' ? console.error : 
                level === 'warn' ? console.warn : 
                console.log;

  method(prefix, message, ...args);
}

// Funciones de conveniencia por nivel
export const logger = {
  error: (category: LogCategory, message: string, ...args: any[]) => log('error', category, message, ...args),
  warn: (category: LogCategory, message: string, ...args: any[]) => log('warn', category, message, ...args),
  info: (category: LogCategory, message: string, ...args: any[]) => log('info', category, message, ...args),
  debug: (category: LogCategory, message: string, ...args: any[]) => log('debug', category, message, ...args),
  verbose: (category: LogCategory, message: string, ...args: any[]) => log('verbose', category, message, ...args),
};

// Detección de refreshes y HMR
let lastRefreshTime = 0;
let refreshCount = 0;

export function detectRefresh(source: string = 'unknown') {
  const now = Date.now();
  const timeSinceLastRefresh = now - lastRefreshTime;
  
  refreshCount++;
  lastRefreshTime = now;
  
  // Detectar si es un refresh rápido (posible HMR failure)
  if (timeSinceLastRefresh < 5000 && refreshCount > 1) {
    logger.warn('HMR', `🔄 Page refresh #${refreshCount} by ${source} (${timeSinceLastRefresh}ms since last)`);
    
    // Capturar stack trace para saber quién causó el refresh
    const stack = new Error().stack;
    logger.debug('HMR', 'Refresh stack trace:', stack);
  } else {
    logger.info('HMR', `🔄 App refresh by ${source}`);
  }
}

// Interceptar reloads del window
function interceptReloads() {
  const originalReload = window.location.reload;
  window.location.reload = function(...args) {
    detectRefresh('window.location.reload');
    return originalReload.apply(this, args);
  };
  
  // Interceptar navigation
  const originalAssign = window.location.assign;
  window.location.assign = function(url) {
    if (url === window.location.href) {
      detectRefresh('window.location.assign');
    }
    return originalAssign.call(this, url);
  };
}

// Detectar HMR events de Vite
function interceptHMR() {
  if (import.meta.hot) {
    import.meta.hot.on('vite:beforeUpdate', (payload) => {
      logger.debug('HMR', '⚡ HMR update:', payload.updates.map(u => u.path).join(', '));
    });
    
    import.meta.hot.on('vite:error', (err) => {
      logger.error('HMR', '❌ HMR error:', err);
    });
    
    import.meta.hot.on('vite:invalidate', (payload) => {
      logger.warn('HMR', '🔄 HMR invalidate - full refresh needed:', payload.path);
      detectRefresh('HMR invalidate: ' + payload.path);
    });
  }
}

// Inicializar todo el sistema
export function initLoggerWithRefreshDetection() {
  initLogging();
  interceptReloads();
  interceptHMR();
  
  // Log inicial
  logger.info('App', '🚀 VPy IDE initialized');
  
  // Exponer controles globales para debugging
  (window as any).__vpyLogger = {
    setLevel: setLogLevel,
    toggleCategory,
    setEnabled: setLoggingEnabled,
    config: () => ({ ...config, categories: Array.from(config.categories) }),
    detectRefresh
  };
}