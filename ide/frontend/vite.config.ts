import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Expose monaco-editor "vs" folder at /vs so @monaco-editor/react does not attempt CDN loader.js.
// We point alias 'vs' to the monaco-editor ESM/min directory. MonacoEnvironment.baseUrl will be '/'.
const relax = process.env.VPY_IDE_RELAX_CSP === '1';
// El plugin react siempre incluye React Refresh en dev; si no queremos inline preamble, montamos un plugin custom que elimina el transform Refresh.
const reactPlugin = react();
if(!relax){
  // Monkey patch simple: borrar los hooks relacionados a React Refresh (transform "import RefreshRuntime" etc.)
  const originalTransform = (reactPlugin as any).transform?.bind(reactPlugin);
  if(originalTransform){
    (reactPlugin as any).transform = function(code: string, id: string, ...rest: any[]) {
      // Ejecutar transformación normal
      const res = originalTransform(code, id, ...rest);
      // Si devuelve promesa, encadenamos; si objeto, procesamos directo
      const strip = (r: any) => {
        if(r && typeof r.code === 'string' && r.code.includes('react-refresh-runtime')){
          // Quitar cualquier import/usage refresh runtime y preamble injection
            r.code = r.code
              .replace(/import\s+['\"]react-refresh\/runtime['\"];?/g, '')
              .replace(/\nif\s*\(import\.meta\.hot\)[\s\S]*?\n}\n?/g, '\n');
        }
        return r;
      };
      return res instanceof Promise ? res.then(strip) : strip(res);
    };
  }
}

// Plugin para eliminar los bloques inline (script y style) que inserta React Refresh
// cuando NO estamos en modo relajado. Evita romper CSP sin permitir 'unsafe-inline'.
const stripRefreshPlugin = () => ({
  name: 'strip-react-refresh-inline',
  apply: 'serve' as const,
  transformIndexHtml(html: string) {
    if (relax) return html; // mantenemos refresh normal
    // El preámbulo inline suele contener 'react-refresh' y 'import.meta.hot'
    let out = html.replace(/<script[^>]*>[^<]*react-refresh[\s\S]*?<\/script>/g, '');
    // El overlay de estilos incluye selectores con 'react-refresh'
    out = out.replace(/<style[^>]*>[^<]*react-refresh[\s\S]*?<\/style>/g, '');
    return out;
  }
});

export default defineConfig({
  plugins: [reactPlugin, stripRefreshPlugin()],
  // Use relative paths for assets so Electron can load them from file://
  base: './',
  resolve: {
    alias: {
  'vs': path.resolve(__dirname, 'node_modules/monaco-editor/min/vs')
    }
  },
  assetsInclude: ['**/*.bin'],
  optimizeDeps: {
    // Ensure monaco-editor pieces are pre-bundled, reducing dynamic injections
    include: ['monaco-editor/esm/vs/editor/editor.api']
  },
  server: {
    port: 5184
  }
});
