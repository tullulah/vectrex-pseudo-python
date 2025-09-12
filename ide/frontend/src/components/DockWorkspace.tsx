import React, { useCallback, useEffect, useMemo, useRef } from 'react';
import { Layout, Model, TabNode, IJsonModel } from 'flexlayout-react';
import { dockBus, DockEvent, DockComponent, notifyDockChanged } from '../state/dockBus';
import 'flexlayout-react/style/dark.css';
import { FileTreePanel } from './panels/FileTreePanel';
import { EditorPanel } from './panels/EditorPanel';
import { EmulatorPanel } from './panels/EmulatorPanel';
import { DebugPanel } from './panels/DebugPanel';
import { ErrorsPanel } from './panels/ErrorsPanel';

const STORAGE_KEY = 'vpy_dock_model_v1';

const defaultJson = {
  global: { 
    tabEnableClose: false,
    tabEnableDrag: true,        // explicitly allow tab dragging (sometimes implicit default)
    tabSetEnableDrop: true      // ensure tabsets accept drops
  },
  layout: {
    type: 'row',
    weight: 100,
    children: [
      { type: 'tabset', weight: 20, children: [ { type: 'tab', name: 'Files', component: 'files' } ] },
      { type: 'tabset', weight: 60, children: [ { type: 'tab', name: 'Editor', component: 'editor', enableClose: false } ] },
  { type: 'tabset', weight: 20, children: [ { type: 'tab', name: 'Emulator', component: 'emulator' } ] },
  { type: 'tabset', weight: 30, children: [ { type: 'tab', name: 'Debug', component: 'debug' }, { type: 'tab', name: 'Errors', component: 'errors' } ], location: 'bottom' }
    ]
  }
};

export const DockWorkspace: React.FC = () => {
  const stored = typeof window !== 'undefined' ? window.localStorage.getItem(STORAGE_KEY) : null;
  const model = useMemo(() => Model.fromJson((stored ? JSON.parse(stored) : defaultJson) as IJsonModel), [stored]);
  const layoutRef = useRef<Layout | null>(null);
  const dragStateRef = useRef<{ active: boolean; tabId?: string; startX: number; startY: number; currentIndex?: number; targetIndex?: number; tabsetId?: string; marker?: HTMLDivElement; container?: HTMLElement; overlay?: HTMLDivElement; targetTabsetId?: string } | null>(null);
  // Expose globally so static handlers can reach
  (window as any).__vpyDragStateRef = dragStateRef;
  (window as any).__vpyDockModel = model;

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

  // Persist changes
  const onModelChange = useCallback(() => {
    const json = model.toJson();
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(json));
  }, [model]);

  // Force relayout of Monaco when editor tab becomes visible
  const onAction = useCallback((action: any) => {
    // After any internal flexlayout action, notify change (debounced naturally by React loop)
    setTimeout(() => notifyDockChanged(), 0);
    return action;
  }, []);

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
    // Decide target tabset based on comp
    let target: string | undefined;
    model.visitNodes((n) => {
      // pick first tabset roughly matching typical region
      if (n.getType && n.getType() === 'tabset' && !target) {
        target = n.getId();
      }
    });
    const nameMap: Record<DockComponent,string> = {
      files: 'Files', editor: 'Editor', emulator: 'Emulator', debug: 'Debug', errors: 'Errors'
    };
    if (target) {
      model.doAction({
        type: 'FlexLayout_AddNode',
        json: { type: 'tab', component: comp, name: nameMap[comp] },
        to: target,
        // position -1 append
        index: -1
      } as any);
    }
  }, [hasComponent, model]);

  const removeComponent = useCallback((comp: DockComponent) => {
    const toRemove: string[] = [];
    model.visitNodes((n) => {
      // @ts-ignore
      if (n._attributes?.component === comp) {
        // @ts-ignore
        toRemove.push(n.getId());
      }
    });
    toRemove.forEach(id => {
      model.doAction({ type: 'FlexLayout_DeleteTab', node: id } as any);
    });
  }, [model]);

  useEffect(() => {
    const unsub = dockBus.on((ev: DockEvent) => {
      if (ev.type === 'toggle') {
        if (hasComponent(ev.component)) removeComponent(ev.component); else addComponent(ev.component);
        notifyDockChanged();
      } else if (ev.type === 'reset') {
        const fresh = Model.fromJson(defaultJson as IJsonModel);
        // Replace model contents
        // @ts-ignore internal API to swap, fallback recreation if needed
        model._root = fresh._root;
        notifyDockChanged();
      }
    });
    return () => { unsub(); };
  }, [addComponent, hasComponent, model, removeComponent]);

  useEffect(() => {
    // Example: add future dynamic tabs via layoutRef.current?.addTabWithDragAndDrop
  }, []);

  // Debug: log dragstart events in Tauri to help diagnose disabled drag interactions
  // Removed custom drag listeners to let flexlayout manage DnD natively. Reintroduce if needed.
  useEffect(() => {
    if (!(window as any).__TAURI_IPC__) return;
    const isTabButton = (el: EventTarget | null) => {
      return !!(el instanceof HTMLElement && el.classList.contains('flexlayout__tab_button'));
    };
    const onDragStart = (e: DragEvent) => {
      if (isTabButton(e.target)) {
        try {
          e.dataTransfer?.setData('text/plain', 'tab');
          if (e.dataTransfer) {
            e.dataTransfer.effectAllowed = 'move';
            e.dataTransfer.dropEffect = 'move';
          }
        } catch {}
        // console.debug('[Dock] dragstart forced');
      }
    };
    const onDragOver = (e: DragEvent) => {
      // Allow dropping anywhere inside layout/tabset borders
      const t = e.target as HTMLElement | null;
      if (t && (t.closest('.flexlayout__tabset') || t.closest('.flexlayout__border') || t.classList.contains('flexlayout__layout'))) {
        e.preventDefault();
        if (e.dataTransfer) e.dataTransfer.dropEffect = 'move';
      }
    };
    document.addEventListener('dragstart', onDragStart, true);
    document.addEventListener('dragover', onDragOver, true);
    const onDragEnter = (e: DragEvent) => {
      const t = e.target as HTMLElement | null;
      if (t && (t.closest('.flexlayout__tabset') || t.closest('.flexlayout__border'))) {
        e.preventDefault();
      }
    };
    const onDrop = (e: DragEvent) => {
      const t = e.target as HTMLElement | null;
      if (t && (t.closest('.flexlayout__tabset') || t.closest('.flexlayout__border'))) {
        e.preventDefault();
      }
    };
    document.addEventListener('dragenter', onDragEnter, true);
    document.addEventListener('drop', onDrop, true);
    // Force WebView2 to acknowledge draggable items (sometimes needed after dynamic creation)
    requestAnimationFrame(() => {
      document.querySelectorAll('.flexlayout__tab_button').forEach(el => {
        (el as HTMLElement).setAttribute('draggable','true');
      });
    });
    return () => {
      document.removeEventListener('dragstart', onDragStart, true);
      document.removeEventListener('dragover', onDragOver, true);
      document.removeEventListener('dragenter', onDragEnter, true);
      document.removeEventListener('drop', onDrop, true);
    };
  }, []);

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
        ref={(r: Layout | null) => (layoutRef.current = r)}
        model={model}
        factory={factory}
        onModelChange={onModelChange}
        onAction={onAction}
      />
    </div>
  );
};

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
    model.doAction({ type: 'FlexLayout_MoveNode', node: tabId, to: destTabset, index: targetIndex } as any);
    console.debug('[Dock] Moved tab', tabId, 'to tabset', destTabset, 'index', targetIndex);
  } catch (err) {
    console.warn('[Dock] move failed, fallback not applied', err);
  }
}

function deriveTabsetId(bar: HTMLElement): string | undefined {
  // flexlayout-react doesn't expose id on DOM elements by default; we approximate: hash of tab button texts
  const texts = Array.from(bar.querySelectorAll('.flexlayout__tab_button')).map(b=>b.textContent?.trim()||'').join('|');
  if (!texts) return undefined;
  // simple hash
  let h = 0; for (let i=0;i<texts.length;i++){ h = (h*31 + texts.charCodeAt(i))|0; }
  return 'ts_'+Math.abs(h);
}
