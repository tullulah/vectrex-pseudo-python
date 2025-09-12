import React, { useCallback, useEffect, useMemo, useRef } from 'react';
import { Layout, Model, TabNode, IJsonModel } from 'flexlayout-react';
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
    // Could inspect actions to detect resize; for now rely on automaticLayout.
    return action;
  }, []);

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
