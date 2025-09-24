import React, { useEffect, useState } from 'react';
import { useEditorStore } from '../state/editorStore';

interface RecentEntry { path: string; mtime?: number; }

export const WelcomeView: React.FC = () => {
  return (
    <div className="vpy-welcome-root">
      <Branding />
      <QuickActions />
      <RecentList />
    </div>
  );
};

const Branding: React.FC = () => {
  return (
    <div className="vpy-welcome-branding">
      <div className="title">Pseudo Python</div>
      <div className="subtitle">Bienvenido. Abre un archivo, carpeta o crea uno nuevo.</div>
    </div>
  );
};

const QuickActions: React.FC = () => {
  const newFile = () => {
    try {
      const st = (useEditorStore as any).getState();
      let idx = 1; let uri: string;
      while (true) { uri = `inmemory://untitled-${idx}.vpy`; if (!st.documents.some((d:any)=>d.uri===uri)) break; idx++; }
      st.openDocument({ uri, language:'vpy', content:'', dirty:false, diagnostics:[], lastSavedContent:'' });
    } catch {}
  };
  return (
    <div className="vpy-welcome-actions">
      <button className="vpy-btn" onClick={() => { try { (window as any).api?.files?.openFile?.(); } catch {} }}>Abrir archivo...</button>
      <button className="vpy-btn" onClick={newFile}>Nuevo archivo</button>
      <button className="vpy-btn" onClick={() => { try { (window as any).api?.files?.openFolder?.(); } catch {} }}>Abrir carpeta...</button>
    </div>
  );
};

const RecentList: React.FC = () => {
  const [recents, setRecents] = useState<RecentEntry[]>([]);
  useEffect(() => {
    let mounted = true;
    (async () => {
      try {
        const arr = await (window as any).api?.recents?.load?.();
        if (mounted && Array.isArray(arr)) setRecents(arr.slice(0, 10));
      } catch {}
    })();
    return () => { mounted = false; };
  }, []);
  if (!recents.length) return null;
  return (
    <div className="vpy-welcome-recents">
      <div className="heading">Recientes</div>
      <div className="list">
        {recents.map(r => {
          const parts = r.path.split(/[/\\]/);
          const file = parts.pop() || r.path;
          const parent = parts.slice(-1)[0] || '';
          return (
            <button key={r.path} className="vpy-recent-item" title={r.path} onClick={() => {
              try { (window as any).api?.files?.openFilePath?.(r.path); } catch {}
            }}>
              <span className="file">{file}</span>
              {parent && <span className="parent">{parent}</span>}
            </button>
          );
        })}
      </div>
    </div>
  );
};

// Local styles (scoped via class names)
// You can migrate to a CSS/SCSS file later if preferred.
export const welcomeStyles = `
.vpy-welcome-root { display:flex; flex-direction:column; align-items:center; justify-content:center; height:100%; gap:26px; color:#bbb; font-size:14px; }
.vpy-welcome-branding { text-align:center; }
.vpy-welcome-branding .title { font-size:48px; font-weight:300; letter-spacing:1px; color:#4fc1ff; margin-bottom:6px; }
.vpy-welcome-branding .subtitle { font-size:16px; color:#aaa; }
.vpy-welcome-actions { display:flex; gap:14px; }
.vpy-welcome-recents { margin-top:4px; max-width:520px; width:100%; }
.vpy-welcome-recents .heading { text-transform:uppercase; font-size:11px; letter-spacing:1px; color:#666; margin-bottom:6px; }
.vpy-welcome-recents .list { display:flex; flex-direction:column; gap:4px; }
.vpy-recent-item { text-align:left; background:#171717; border:1px solid #262626; color:#d0d0d0; padding:8px 12px; border-radius:4px; cursor:pointer; font-size:13px; display:flex; justify-content:space-between; align-items:center; }
.vpy-recent-item:hover { background:#1f1f1f; border-color:#333; }
.vpy-recent-item .file { font-weight:500; }
.vpy-recent-item .parent { font-size:11px; color:#666; margin-left:12px; }
`;
