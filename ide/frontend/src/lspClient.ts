import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Simple LSP client that frames JSON-RPC messages and parses server responses.
// We only parse Content-Length headers and forward parsed JSON to handlers.

export type LspNotificationHandler = (method: string, params: any) => void;
export type LspResponseHandler = (id: number | string, result: any, error?: any) => void;

interface PendingRequest { resolve: (v: any)=>void; reject: (e: any)=>void; method: string; }

class LspClient {
  private seq = 0;
  private buffer = '';
  private contentLength: number | null = null;
  private pending = new Map<number | string, PendingRequest>();
  private notifHandlers: LspNotificationHandler[] = [];
  private respHandlers: LspResponseHandler[] = [];
  private started = false;

  async start() {
    if (this.started) return;
    await invoke('lsp_start');
    await listen<string>('lsp://stdout', (e) => this.onStdoutChunk(e.payload));
    await listen<string>('lsp://stderr', (e) => console.warn('[LSP-STDERR]', e.payload));
    this.started = true;
  }

  onNotification(cb: LspNotificationHandler) { this.notifHandlers.push(cb); }
  onResponse(cb: LspResponseHandler) { this.respHandlers.push(cb); }

  private onStdoutChunk(chunk: string) {
    // chunk may be a partial header or body. Accumulate.
    this.buffer += chunk + '\n'; // side reading is line-based
    while (true) {
      if (this.contentLength == null) {
        const headerEnd = this.buffer.indexOf('\r\n\r\n');
        if (headerEnd === -1) return; // wait more
        const header = this.buffer.slice(0, headerEnd);
        const match = /Content-Length: (\d+)/i.exec(header);
        if (!match) {
          console.error('Invalid LSP header', header);
          // drop until after headerEnd
          this.buffer = this.buffer.slice(headerEnd + 4);
          continue;
        }
        this.contentLength = parseInt(match[1], 10);
        this.buffer = this.buffer.slice(headerEnd + 4);
      }
      if (this.contentLength != null) {
        if (this.buffer.length < this.contentLength) return; // wait body
        const body = this.buffer.slice(0, this.contentLength);
        this.buffer = this.buffer.slice(this.contentLength);
        this.contentLength = null;
        this.dispatchMessage(body);
        // loop to see if another message already buffered
      }
    }
  }

  private dispatchMessage(jsonText: string) {
    try {
      const msg = JSON.parse(jsonText);
      if (msg.id !== undefined && (msg.result !== undefined || msg.error !== undefined)) {
        // response
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
    const json = JSON.stringify(obj);
    return invoke('lsp_send', { payload: json });
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
}

export const lspClient = new LspClient();

export async function initLsp(language: string, documentUri: string, text: string) {
  await lspClient.start();
  await lspClient.request('initialize', {
    processId: null,
    rootUri: null,
    capabilities: {},
    locale: language,
  });
  lspClient.notify('initialized', {});
  lspClient.notify('textDocument/didOpen', {
    textDocument: { uri: documentUri, languageId: 'vpy', version: 1, text }
  });
}
