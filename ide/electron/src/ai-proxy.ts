// AI Provider Proxy - Handles CORS-blocked APIs (Anthropic, DeepSeek)
import { ipcMain } from 'electron';
import https from 'https';
import http from 'http';

interface ProxyRequest {
  provider: 'anthropic' | 'deepseek';
  apiKey: string;
  endpoint: string;
  method: string;
  body: any;
  headers?: Record<string, string>;
}

interface ProxyResponse {
  success: boolean;
  data?: any;
  error?: string;
  status?: number;
}

const PROVIDER_CONFIG = {
  anthropic: {
    baseUrl: 'https://api.anthropic.com',
    defaultHeaders: {
      'anthropic-version': '2023-06-01'
    }
  },
  deepseek: {
    baseUrl: 'https://api.deepseek.com',
    defaultHeaders: {}
  }
};

/**
 * Makes an HTTPS request to the AI provider API
 */
function makeRequest(url: string, options: https.RequestOptions, body: string): Promise<{ status: number; data: string }> {
  return new Promise((resolve, reject) => {
    const protocol = url.startsWith('https') ? https : http;
    
    const req = protocol.request(url, options, (res) => {
      let data = '';
      
      res.on('data', (chunk) => {
        data += chunk;
      });
      
      res.on('end', () => {
        resolve({
          status: res.statusCode || 500,
          data
        });
      });
    });
    
    req.on('error', (error) => {
      reject(error);
    });
    
    if (body) {
      req.write(body);
    }
    
    req.end();
  });
}

/**
 * Proxy handler for AI API requests
 */
async function handleProxyRequest(request: ProxyRequest): Promise<ProxyResponse> {
  try {
    const config = PROVIDER_CONFIG[request.provider];
    if (!config) {
      return {
        success: false,
        error: `Unknown provider: ${request.provider}`
      };
    }

    const url = `${config.baseUrl}${request.endpoint}`;
    const body = JSON.stringify(request.body);

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      'Content-Length': Buffer.byteLength(body).toString(),
      ...config.defaultHeaders,
      ...(request.headers || {})
    };

    // Add authentication header
    if (request.provider === 'anthropic') {
      headers['x-api-key'] = request.apiKey;
    } else if (request.provider === 'deepseek') {
      headers['Authorization'] = `Bearer ${request.apiKey}`;
    }

    const options: https.RequestOptions = {
      method: request.method,
      headers
    };

    console.log(`[AI Proxy] ${request.method} ${url}`);
    
    const response = await makeRequest(url, options, body);
    
    console.log(`[AI Proxy] Response status: ${response.status}`);

    if (response.status >= 200 && response.status < 300) {
      return {
        success: true,
        data: JSON.parse(response.data),
        status: response.status
      };
    } else {
      return {
        success: false,
        error: response.data,
        status: response.status
      };
    }
  } catch (error: any) {
    console.error('[AI Proxy] Error:', error);
    return {
      success: false,
      error: error.message || 'Unknown error'
    };
  }
}

/**
 * Register IPC handlers for AI proxy
 */
export function registerAIProxyHandlers() {
  ipcMain.handle('ai-proxy-request', async (_event, request: ProxyRequest) => {
    return handleProxyRequest(request);
  });

  console.log('[AI Proxy] Handlers registered');
}
