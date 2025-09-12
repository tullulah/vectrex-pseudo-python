import React, { useCallback, useEffect, useMemo, useRef } from 'react';
import { Layout, Model, TabNode, IJsonModel } from 'flexlayout-react';
import { dockBus, DockEvent, DockComponent, notifyDockChanged } from '../state/dockBus';
import 'flexlayout-react/style/dark.css';
import { FileTreePanel } from './panels/FileTreePanel';
import { EditorPanel } from './panels/EditorPanel';
import { EmulatorPanel } from './panels/EmulatorPanel';
import { DebugPanel } from './panels/DebugPanel';

const STORAGE_KEY = 'vpy_dock_model_v1';

const defaultJson = {
  global: { tabEnableClose: false },
  layout: {
    type: 'row',
    weight: 100,
    children: [
      { type: 'tabset', weight: 20, children: [ { type: 'tab', name: 'Files', component: 'files' } ] },
      { type: 'tabset', weight: 60, children: [ { type: 'tab', name: 'Editor', component: 'editor', enableClose: false } ] },
      { type: 'tabset', weight: 20, children: [ { type: 'tab', name: 'Emulator', component: 'emulator' } ] },
      { type: 'tabset', weight: 25, children: [ { type: 'tab', name: 'Debug', component: 'debug' } ], location: 'bottom' }
    ]
  }
};

export const DockWorkspace: React.FC = () => {
  const stored = typeof window !== 'undefined' ? window.localStorage.getItem(STORAGE_KEY) : null;
  const model = useMemo(() => Model.fromJson((stored ? JSON.parse(stored) : defaultJson) as IJsonModel), [stored]);
  const layoutRef = useRef<Layout | null>(null);

  const factory = useCallback((node: TabNode) => {
    const comp = node.getComponent();
    switch (comp) {
      case 'files': return <FileTreePanel />;
      case 'editor': return <EditorPanel />;
      case 'emulator': return <EmulatorPanel />;
      case 'debug': return <DebugPanel />;
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
      files: 'Files', editor: 'Editor', emulator: 'Emulator', debug: 'Debug'
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

  return (
    <div style={{position:'absolute', inset:0}}>
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
