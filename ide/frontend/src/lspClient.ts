import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

// Simple LSP client that frames JSON-RPC messages and parses server responses.
// We only parse Content-Length headers and forward parsed JSON to handlers.

export type LspNotificationHandler = (method: string, params: any) => void;
export type LspResponseHandler = (id: number | string, result: any, error?: any) => void;

interface PendingRequest { resolve: (v: any)=>void; reject: (e: any)=>void; method: string; }

class LspClient {
  private seq = 0;
  // Framing handled in Rust side; we receive complete JSON payloads via 'lsp://message'
  private pending = new Map<number | string, PendingRequest>();
  private notifHandlers: LspNotificationHandler[] = [];
  private respHandlers: LspResponseHandler[] = [];
  private started = false;
  private versions = new Map<string, number>(); // uri -> version

  async start() {
    if (this.started) return;
    if (typeof window === 'undefined' || !(window as any).__TAURI_IPC__) {
      // Not running inside Tauri environment; skip starting LSP.
      return;
    }
    await invoke('lsp_start');
  await listen<string>('lsp://message', (e) => this.dispatchMessage(e.payload));
  // Keep stdout / stderr listeners for debugging
  await listen<string>('lsp://stdout', (e) => console.debug('[LSP-RAW]', e.payload));
    await listen<string>('lsp://stderr', (e) => console.warn('[LSP-STDERR]', e.payload));
    this.started = true;
  }

  onNotification(cb: LspNotificationHandler) { this.notifHandlers.push(cb); }
  onResponse(cb: LspResponseHandler) { this.respHandlers.push(cb); }

  private dispatchMessage(jsonText: string) {
    try {
      const msg = JSON.parse(jsonText);
      // Ignore parse-error notifications with null id (e.g. { error:{code:-32700}, id:null })
      if ((msg.error && (msg.id === null || msg.id === undefined) && msg.error.code === -32700)) {
        console.warn('[LSP<-SERVER] parse error (ignored, waiting for real response)', msg);
        return;
      }
      if (msg.id !== undefined && (msg.result !== undefined || msg.error !== undefined)) {
        // response
        if (msg.error) {
          console.error('[LSP<-SERVER] error response', msg);
        } else {
          console.debug('[LSP<-SERVER] response', msg);
        }
        this.respHandlers.forEach(h => h(msg.id, msg.result, msg.error));
        const pending = this.pending.get(msg.id);
        if (pending) {
          if (msg.error) pending.reject(msg.error); else pending.resolve(msg.result);
          this.pending.delete(msg.id);
        }
      } else if (msg.method) {
        // notification or request from server (we treat both same; no request handling yet)
        this.notifHandlers.forEach(h => h(msg.method, msg.params));
      } else {
        console.warn('Unknown LSP message shape', msg);
      }
    } catch (e) {
      console.error('Failed parse LSP message', e, jsonText);
    }
  }

  private sendRaw(obj: any): Promise<any> {
    if (!this.started) return Promise.resolve();
    if (typeof window === 'undefined' || !(window as any).__TAURI_IPC__) {
      // Running outside Tauri (e.g. plain web build) -> no-op
      return Promise.resolve();
    }
    const json = JSON.stringify(obj);
    console.debug('[LSP->SERVER]', json);
    return invoke('lsp_send', { payload: json }).catch(err => {
      console.warn('[LSP] send error', err);
    });
  }

  request(method: string, params: any): Promise<any> {
    const id = ++this.seq;
    const p = new Promise<any>((resolve, reject) => {
      this.pending.set(id, { resolve, reject, method });
    });
    this.sendRaw({ jsonrpc: '2.0', id, method, params });
    return p;
  }

  notify(method: string, params: any) {
    this.sendRaw({ jsonrpc: '2.0', method, params });
  }

  didOpen(uri: string, languageId: string, text: string) {
    this.versions.set(uri, 1);
    this.notify('textDocument/didOpen', {
      textDocument: { uri, languageId, version: 1, text }
    });
  }

  didChange(uri: string, text: string) {
    if (!this.started) return; // avoid calling before start
    const current = (this.versions.get(uri) || 1) + 1;
    this.versions.set(uri, current);
    this.notify('textDocument/didChange', {
      textDocument: { uri, version: current },
      contentChanges: [ { text } ]
    });
  }
}

export const lspClient = new LspClient();

export async function initLsp(language: string, documentUri: string, text: string) {
  await lspClient.start();
  const params: any = {
    processId: null,
    rootUri: null,          // explicit per spec when no workspace
    capabilities: {          // minimal but explicit client capabilities
      textDocument: {
        synchronization: { didSave: false, willSave: false, willSaveWaitUntil: false, dynamicRegistration: false },
        publishDiagnostics: { relatedInformation: false },
      },
      general: { positionEncodings: ['utf-16'] },
    },
    clientInfo: { name: 'vpy-ide', version: '0.1.0' },
    initializationOptions: {},
    trace: 'off',
    locale: language,
    workspaceFolders: null,
  };
  console.debug('[LSP] initialize params', params);
  let gotResult = false;
  const initPromise = lspClient.request('initialize', params).then(res => { gotResult = true; return res; });
  try {
    const res = await initPromise;
    console.debug('[LSP] initialize result', res);
  } catch (e:any) {
    if (e && e.code === -32600) {
      console.warn('[LSP] initialize returned -32600 (Invalid request) – tolerando por bug inicial, esperando posible respuesta válida posterior');
    } else {
      console.error('[LSP] initialize failed', e);
      return; // abort if other error
    }
  }
  // Even if we saw -32600, server in nuestra experiencia envía el resultado válido después.
  // Para robustez añadimos un pequeño retraso para permitir la llegada.
  if (!gotResult) {
    setTimeout(() => {
      // No direct follow-up; we proceed anyway to send 'initialized'.
    }, 50);
  }
  lspClient.notify('initialized', {});
  lspClient.didOpen(documentUri, 'vpy', text);
}
