import React, { useCallback, useEffect, useMemo, useRef } from 'react';
import { Layout, Model, TabNode, IJsonModel, Actions, DockLocation } from 'flexlayout-react';
import { useEditorStore } from '../state/editorStore';
import { dockBus, DockEvent, DockComponent, notifyDockChanged } from '../state/dockBus';
import 'flexlayout-react/style/dark.css';
import { FileTreePanel } from './panels/FileTreePanel';
import { EditorPanel } from './panels/EditorPanel';
import { EmulatorPanel } from './panels/EmulatorPanel';
import { DebugPanel } from './panels/DebugPanel';
import { ErrorsPanel } from './panels/ErrorsPanel';

// Bumped to v2 to force layout refresh including new 'Errors' tab for users with persisted v1 layout
const STORAGE_KEY = 'vpy_dock_model_v2';

const defaultJson = {
  global: { 
    tabEnableClose: true,       // ahora se pueden cerrar (salvo que un tab específico lo deshabilite)
    tabEnableDrag: true,
    tabSetEnableDrop: true
  },
  layout: {
    type: 'row',
    weight: 100,
    children: [
      { type: 'tabset', weight: 20, children: [ { type: 'tab', name: 'Files', component: 'files' } ] },
  { type: 'tabset', weight: 60, children: [ { type: 'tab', name: 'Editor', component: 'editor', enableClose: true } ] },
  { type: 'tabset', weight: 20, children: [ { type: 'tab', name: 'Emulator', component: 'emulator' } ] },
  { type: 'tabset', weight: 30, children: [ { type: 'tab', name: 'Debug', component: 'debug' }, { type: 'tab', name: 'Errors', component: 'errors' } ], location: 'bottom' }
    ]
  }
};

export const DockWorkspace: React.FC = () => {
  const documents = useEditorStore((s:any)=>s.documents);
  const setActive = useEditorStore((s:any)=>s.setActive);
  const stored = typeof window !== 'undefined' ? window.localStorage.getItem(STORAGE_KEY) : null;
  const model = useMemo(() => Model.fromJson((stored ? JSON.parse(stored) : defaultJson) as IJsonModel), [stored]);
  const layoutRef = useRef<Layout | null>(null);
  const dragStateRef = useRef<{ active: boolean; tabId?: string; startX: number; startY: number; currentIndex?: number; targetIndex?: number; tabsetId?: string; marker?: HTMLDivElement; container?: HTMLElement; overlay?: HTMLDivElement; targetTabsetId?: string } | null>(null);
  // Expose globally so static handlers can reach
  (window as any).__vpyDragStateRef = dragStateRef;
  (window as any).__vpyDockModel = model;
  //

  const factory = useCallback((node: TabNode) => {
    const comp = node.getComponent();
    switch (comp) {
      case 'files': return <FileTreePanel />;
      case 'editor': return <EditorPanel />;
      case 'emulator': return <EmulatorPanel />;
  case 'debug': return <DebugPanel />;
  case 'errors': return <ErrorsPanel />;
      default: return <div>Unknown: {comp}</div>;
    }
  }, []);

  // Renderizador custom de tabs para añadir botón de cierre confiable incluso si flexlayout oculta el suyo en WebView2
  const onRenderTab = useCallback((node: TabNode, renderValues: any) => {
    const comp = (node as any)?._attributes?.component;
    const canClose = true; // toutes las tabs cerrables ahora
    if (canClose) {
      // Asegurar array buttons existe
      renderValues.buttons = renderValues.buttons || [];
      // Evitar duplicados si el renderizador se invoca múltiples veces: filtrar previos con nuestra marca
      renderValues.buttons = renderValues.buttons.filter((b:any) => !(b?.key && (""+b.key).startsWith('close-')));
      renderValues.buttons.push(
        <button
          key={`close-${node.getId()}`}
          className="vpy-tab-close"
          title="Close"
          onClick={(e) => {
            e.stopPropagation();
            try {
              model.doAction(Actions.deleteTab(node.getId()));
            } catch (err) {
              console.warn('[Dock] close failed', err);
            }
          }}
          style={{
            background:'transparent', border:'none', color:'#aaa', cursor:'pointer', padding:0,
            fontSize:12, lineHeight:1, width:16, height:16, display:'flex', alignItems:'center', justifyContent:'center'
          }}
        >×</button>
      );
    }
  }, [model]);

  // Persist changes
  const onModelChange = useCallback(() => {
    const json = model.toJson();
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(json));
    // schedule DOM tagging refresh shortly after React commit
    requestAnimationFrame(() => tagTabsetsWithIds(model));
  }, [model]);

  // Force relayout of Monaco when editor tab becomes visible
  const onAction = useCallback((action: any) => {
    if (action.type === 'FlexLayout_DeleteTab') {
      // Find node being deleted
      const nodeId = action?.data?.node || action.node; // compat
      let targetNode: any = undefined;
      model.visitNodes((n: any) => { if (n.getId && n.getId() === nodeId) targetNode = n; });
      const comp = targetNode?._attributes?.component;
      if (comp === 'editor') {
  const dirty = documents.some((d:any)=>d.dirty);
        if (dirty) {
          const confirmClose = window.confirm('Hay cambios sin guardar. ¿Cerrar de todos modos?');
          if (!confirmClose) {
            return undefined; // cancelar acción
          }
        }
      }
      // If closing current active doc, clear active (simple behavior)
      if (comp === 'editor') {
        setActive('');
      }
    }
    setTimeout(() => notifyDockChanged(), 0);
    return action;
  }, [documents, model, setActive]);

  // Helper to find if a component tab exists
  const hasComponent = useCallback((comp: DockComponent) => {
    let found = false;
    model.visitNodes((n) => {
      // @ts-ignore private API but fine for now
      if (n._attributes?.component === comp) found = true;
    });
    return found;
  }, [model]);

  const addComponent = useCallback((comp: DockComponent) => {
    if (hasComponent(comp)) return;
    console.debug('[Dock] addComponent', comp);

    // Map internal component name to display title
    const nameMap: Record<DockComponent,string> = {
      files: 'Files', editor: 'Editor', emulator: 'Emulator', debug: 'Debug', errors: 'Errors'
    };

    // Find anchor tabset (editor) for spatial placement
    let editorTabset: string | undefined;
    model.visitNodes(n => {
      if ((n as any).getType && (n as any).getType() === 'tabset') {
        const children: any[] = (n as any).getChildren?.() || [];
        if (children.some(c => c?._attributes?.component === 'editor')) {
          editorTabset = (n as any).getId?.();
        }
      }
    });

    // Fallback: first tabset id if editor not found
    if (!editorTabset) {
      model.visitNodes(n => { if (!editorTabset && (n as any).getType && (n as any).getType()==='tabset') editorTabset = (n as any).getId?.(); });
    }

    // Decide desired docking relative to editor for each component
    let location: typeof DockLocation.CENTER = DockLocation.CENTER; // default center (same tabset)
    if (comp === 'files') location = DockLocation.LEFT; else if (comp === 'emulator') location = DockLocation.RIGHT; else if (comp === 'debug' || comp === 'errors') location = DockLocation.BOTTOM;

    try {
      if (editorTabset) {
        model.doAction(
          Actions.addNode({ type: 'tab', component: comp, name: nameMap[comp] } as any, editorTabset, location, -1)
        );
      }
      else {
        // Ultimate fallback: append to any (center) if no tabset found
        console.warn('[Dock] editorTabset not found; appending component in first tabset');
        let first: string | undefined; model.visitNodes(n=>{ if (!first && (n as any).getType && (n as any).getType()==='tabset') first = (n as any).getId?.(); });
        if (first) model.doAction(Actions.addNode({ type: 'tab', component: comp, name: nameMap[comp] } as any, first, DockLocation.CENTER, -1));
      }
    } catch (e) {
      console.warn('[Dock] addComponent failed', e);
    }
  }, [hasComponent, model]);

  const removeComponent = useCallback((comp: DockComponent) => {
    console.debug('[Dock] removeComponent', comp);
    const toRemove: string[] = [];
    model.visitNodes((n) => {
      // @ts-ignore
      if (n._attributes?.component === comp) {
        // @ts-ignore
        toRemove.push(n.getId());
      }
    });
    toRemove.forEach(id => {
      try { model.doAction(Actions.deleteTab(id)); } catch(e) { console.warn('[Dock] deleteTab failed', e); }
    });
  }, [model]);

  useEffect(() => {
    const unsub = dockBus.on((ev: DockEvent) => {
      if (ev.type === 'toggle') {
        if (hasComponent(ev.component)) removeComponent(ev.component); else addComponent(ev.component);
        notifyDockChanged();
        requestAnimationFrame(()=>tagTabsetsWithIds(model));
      } else if (ev.type === 'reset') {
        try {
          const fresh = Model.fromJson(defaultJson as IJsonModel);
          // Replace model contents (internal API) then persist
          // @ts-ignore internal API to swap, fallback recreation if needed
          model._root = fresh._root;
          onModelChange();
          notifyDockChanged();
          console.info('[Dock] Layout reset to defaults');
          requestAnimationFrame(()=>tagTabsetsWithIds(model));
        } catch (e) {
          console.warn('[Dock] reset failed', e);
        }
      }
    });
    return () => { unsub(); };
  }, [addComponent, hasComponent, model, removeComponent, onModelChange]);

  // Migration: if layout persisted antes de existir 'Errors', añadir la pestaña automáticamente
  useEffect(() => {
    // slight defer until model stable
    setTimeout(() => {
      if (!hasComponent('errors')) {
        let debugTabset: string | undefined;
        model.visitNodes((n) => {
          // @ts-ignore inspect children for debug component
          if (n.getType && n.getType() === 'tabset') {
            const children: any[] = (n as any).getChildren?.() || [];
            if (children.some(c => c?._attributes?.component === 'debug')) {
              debugTabset = (n as any).getId?.();
            }
          }
        });
        if (debugTabset) {
          try {
            model.doAction(Actions.addNode({ type: 'tab', component: 'errors', name: 'Errors' } as any, debugTabset, DockLocation.CENTER, -1));
            notifyDockChanged();
            console.info('[Dock] Migrated layout: added missing Errors tab');
          } catch (e) {
            console.warn('[Dock] Failed to auto-add Errors tab', e);
          }
        }
      }
    }, 50);
  }, [hasComponent, model]);

  useEffect(() => {
    // Example: add future dynamic tabs via layoutRef.current?.addTabWithDragAndDrop
    tagTabsetsWithIds(model);
  }, []);

  // Tauri-specific drag workaround removed (runtime is Electron + standard web now).

  return (
    <div style={{position:'absolute', inset:0}} onMouseDown={(e) => {
      const target = e.target as HTMLElement;
      if (!target.classList.contains('flexlayout__tab_button')) return;
      // Identify tab id by traversing React-resolved data attributes (fallback to text)
      // flexlayout-react stores an internal id on the tab button element's parent sometimes via dataset. We fallback to node name match.
      let tabName = target.textContent?.trim() || '';
      if (!tabName) return;
      // Find tab node by name (unique enough for demo) and its tabset
      let foundNode: TabNode | undefined; let tabsetId: string | undefined; let indexInSet: number | undefined;
      model.visitNodes((n) => {
        if ((n as any).getName && (n as any).getName() === tabName) {
          foundNode = n as TabNode;
          const parent: any = (n as any).getParent && (n as any).getParent();
          if (parent && parent.getId) {
            tabsetId = parent.getId();
            const children = parent.getChildren();
            indexInSet = children.indexOf(n);
          }
        }
      });
      if (!foundNode || tabsetId===undefined || indexInSet===undefined) return;
      const container = target.closest('.flexlayout__tabset_tabbar_outer') as HTMLElement | null;
      dragStateRef.current = { active: true, tabId: (foundNode as any).getId(), startX: e.clientX, startY: e.clientY, currentIndex: indexInSet, targetIndex: indexInSet, tabsetId, container: container || undefined };
      // Create marker
      const marker = document.createElement('div');
      marker.style.position='absolute';
      marker.style.top='0';
      marker.style.width='2px';
      marker.style.background='#4FC1FF';
      marker.style.zIndex='999';
      marker.style.pointerEvents='none';
      dragStateRef.current.marker = marker;
      document.body.appendChild(marker);
      // Create overlay highlight (for cross-tabset)
      const overlay = document.createElement('div');
      overlay.style.position='absolute';
      overlay.style.border='2px dashed #4FC1FF';
      overlay.style.pointerEvents='none';
      overlay.style.zIndex='998';
      overlay.style.display='none';
      dragStateRef.current.overlay = overlay;
      document.body.appendChild(overlay);
      document.addEventListener('mousemove', handleDragMove, true);
      document.addEventListener('mouseup', handleDragEnd, true);
    }}>
      <Layout
        ref={r => { layoutRef.current = r; }}
        model={model}
        factory={factory}
        onRenderTab={onRenderTab}
        onModelChange={onModelChange}
        onAction={onAction}
      />
    </div>
  );
};

//

function handleDragMove(e: MouseEvent) {
  const ref = (window as any).__vpyDragStateRef as React.MutableRefObject<any> | undefined;
  if (!ref) return;
  const st = ref.current; if (!st || !st.active) return;
  // Detect potential tabset under cursor
  const elUnder = document.elementFromPoint(e.clientX, e.clientY) as HTMLElement | null;
  let tabsetBar = elUnder?.closest('.flexlayout__tabset_tabbar_outer') as HTMLElement | null;
  if (!tabsetBar) {
    // Hide marker & overlay if outside any tab bar
    if (st.marker) st.marker.style.display='none';
    if (st.overlay) st.overlay.style.display='none';
    return;
  }
  const model = (window as any).__vpyDockModel as Model | undefined;
  let targetTabsetId: string | undefined = undefined;
  if (model) {
    // Heuristic: match tabset by comparing child tab button count & positions; fallback: reuse origin tabset if same bar
    // We can embed data-layout-path attribute; if not present rely on identity of DOM node references.
    // For now, store text of first tab as key to locate node set; approximate.
  }
  const buttons = Array.from(tabsetBar.querySelectorAll('.flexlayout__tab_button')) as HTMLElement[];
  if (!buttons.length) return;
  // Compute index inside this tabset
  const x = e.clientX;
  let targetIndex = buttons.length - 1;
  for (let i=0;i<buttons.length;i++) {
    const r = buttons[i].getBoundingClientRect();
    const center = r.left + r.width/2;
    if (x < center) { targetIndex = i; break; }
  }
  st.targetIndex = targetIndex;
  st.targetTabsetId = deriveTabsetId(tabsetBar);
  if (st.marker) {
    const refBtn = buttons[targetIndex];
    const r = refBtn.getBoundingClientRect();
    st.marker.style.display='block';
    st.marker.style.left = `${r.left - 1}px`;
    st.marker.style.height = `${r.height}px`;
    st.marker.style.top = `${r.top + window.scrollY}px`;
  }
  if (st.overlay) {
    const barRect = tabsetBar.getBoundingClientRect();
    st.overlay.style.display='block';
    st.overlay.style.left = `${barRect.left}px`;
    st.overlay.style.top = `${barRect.top + window.scrollY}px`;
    st.overlay.style.width = `${barRect.width}px`;
    st.overlay.style.height = `${barRect.height}px`;
  }
}

function handleDragEnd(_e: MouseEvent) {
  const ref = (window as any).__vpyDragStateRef as React.MutableRefObject<any> | undefined;
  if (!ref) return;
  const st = ref.current; if (!st || !st.active) return;
  if (st.marker && st.marker.parentElement) st.marker.parentElement.removeChild(st.marker);
  if (st.overlay && st.overlay.parentElement) st.overlay.parentElement.removeChild(st.overlay);
  const { tabId, currentIndex, targetIndex, tabsetId, targetTabsetId } = st;
  ref.current = null;
  document.removeEventListener('mousemove', handleDragMove, true);
  document.removeEventListener('mouseup', handleDragEnd, true);
  if (!tabId || targetIndex===undefined) return;
  const destTabset = targetTabsetId || tabsetId;
  if (!destTabset) return;
  const model = (window as any).__vpyDockModel as Model | undefined;
  if (!model) return;
  try {
    // Use official moveNode API (DockLocation.CENTER keeps in same tabset region)
    model.doAction(Actions.moveNode(tabId, destTabset, DockLocation.CENTER, targetIndex));
    console.debug('[Dock] Moved tab', tabId, 'to tabset', destTabset, 'index', targetIndex);
  } catch (err) {
    console.warn('[Dock] move failed, fallback not applied', err);
  }
}

function deriveTabsetId(bar: HTMLElement): string | undefined {
  return bar.getAttribute('data-tabsetid') || undefined;
}

function tagTabsetsWithIds(model: Model) {
  try {
    const tabsetBars = document.querySelectorAll('.flexlayout__tabset_tabbar_outer');
    // Build mapping of first tab button text -> tabset id; but better: iterate model
    const idByFirstName: Record<string,string> = {};
    model.visitNodes((n:any) => {
      if (n.getType && n.getType()==='tabset') {
        const children = n.getChildren?.() || [];
        if (children.length>0) {
          const first = children[0];
          const name = first?.getName?.();
          if (name) idByFirstName[name] = n.getId();
        }
      }
    });
    tabsetBars.forEach(bar => {
      const firstBtn = bar.querySelector('.flexlayout__tab_button');
      const label = firstBtn?.textContent?.trim();
      if (label && idByFirstName[label]) {
        (bar as HTMLElement).setAttribute('data-tabsetid', idByFirstName[label]);
      }
    });
  } catch (e) {
    console.warn('[Dock] tagTabsetsWithIds failed', e);
  }
}
