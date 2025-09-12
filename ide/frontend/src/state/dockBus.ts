// Simple event bus for docking actions
// Events: toggle:<component>, resetLayout, dockChanged
// Components: files | editor | emulator | debug | errors

export type DockComponent = 'files' | 'editor' | 'emulator' | 'debug' | 'errors';
export type DockEvent =
  | { type: 'toggle'; component: DockComponent }
  | { type: 'reset' }
  | { type: 'changed' };

export type DockListener = (ev: DockEvent) => void;

class DockBus {
  private listeners: Set<DockListener> = new Set();
  emit(ev: DockEvent) {
    this.listeners.forEach(l => l(ev));
  }
  on(listener: DockListener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
}

export const dockBus = new DockBus();

export function toggleComponent(component: DockComponent) {
  dockBus.emit({ type: 'toggle', component });
}

export function resetLayout() {
  dockBus.emit({ type: 'reset' });
}

export function notifyDockChanged() {
  dockBus.emit({ type: 'changed' });
}
