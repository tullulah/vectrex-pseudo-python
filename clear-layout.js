// Clear dock layout localStorage to force default layout with PyPilot
localStorage.removeItem('vpy_dock_model_v2');
localStorage.removeItem('vpy_hidden_panels_v1');
localStorage.removeItem('vpy_pinned_panels_v1');
console.log('✓ Dock layout cache cleared - reload to see PyPilot tab!');
alert('Dock layout cache cleared! Please reload (F5) to see the PyPilot tab.');