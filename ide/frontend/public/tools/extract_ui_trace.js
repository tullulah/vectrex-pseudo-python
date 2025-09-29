// Extractor de trace data desde UI del emulador
// Ejecutar en consola del navegador con la UI abierta

console.log('=== EXTRACTOR DE TRACE DATA UI ===');

// Obtener referencia al core del emulador
const core = window.emuCore || window?.emuCore;
if (!core) {
    console.error('No se encontró emuCore en window. Asegúrate de que la UI esté cargada.');
    throw new Error('emuCore no disponible');
}

console.log('Core del emulador encontrado:', core);

// Verificar métodos de trace disponibles
const traceLog = core.traceLog ? core.traceLog() : [];
console.log('Trace entries disponibles:', traceLog.length);

if (traceLog.length === 0) {
    console.warn('No hay entries en el trace. Asegúrate de haber capturado trace data.');
    console.log('Para capturar trace:');
    console.log('1. Habilitar trace capture: core.enableTraceCapture(true, 10000)');
    console.log('2. Ejecutar emulador por unos frames');
    console.log('3. Volver a ejecutar este script');
} else {
    console.log('=== ANÁLISIS DEL TRACE ===');
    
    // Mostrar primeras 50 entradas para ver el inicio
    console.log('PRIMERAS 50 ENTRADAS (inicio BIOS):');
    const first50 = traceLog.slice(0, 50);
    first50.forEach((entry, idx) => {
        const pc = entry.pc.toString(16).padStart(4, '0').toUpperCase();
        const opStr = entry.m || 'UNK';
        const hex = entry.hex || '';
        const operand = entry.operand || '';
        const repeat = entry.repeat > 0 ? ` [x${entry.repeat}]` : '';
        const A = entry.a.toString(16).padStart(2, '0').toUpperCase();
        const B = entry.b.toString(16).padStart(2, '0').toUpperCase();
        const X = entry.x.toString(16).padStart(4, '0').toUpperCase();
        const Y = entry.y.toString(16).padStart(4, '0').toUpperCase();
        
        console.log(`${idx.toString().padStart(3, ' ')}: ${pc}: ${opStr.padEnd(8)} (${hex}) ${operand} A=${A} B=${B} X=${X} Y=${Y}${repeat}`);
    });
    
    // Buscar patrones de bucle
    console.log('\n=== BÚSQUEDA DE BUCLES ===');
    const pcCounts = {};
    traceLog.forEach(entry => {
        const pc = entry.pc;
        pcCounts[pc] = (pcCounts[pc] || 0) + 1;
    });
    
    // Encontrar PCs más visitados (posibles bucles)
    const hotPCs = Object.entries(pcCounts)
        .filter(([pc, count]) => count > 10)
        .sort((a, b) => b[1] - a[1])
        .slice(0, 20);
    
    console.log('TOP 20 PCs MÁS VISITADOS (posibles bucles):');
    hotPCs.forEach(([pc, count]) => {
        const pcHex = parseInt(pc).toString(16).padStart(4, '0').toUpperCase();
        console.log(`${pcHex}: ${count} veces`);
    });
    
    // Buscar específicamente el rango F4EB-F4EF (bucle de delay conocido)
    console.log('\n=== ANÁLISIS DEL BUCLE F4EB-F4EF ===');
    const delayLoopEntries = traceLog.filter(entry => 
        entry.pc >= 0xF4EB && entry.pc <= 0xF4EF
    );
    console.log(`Entradas en rango F4EB-F4EF: ${delayLoopEntries.length}`);
    
    if (delayLoopEntries.length > 0) {
        console.log('PRIMERAS 20 entradas del bucle de delay:');
        delayLoopEntries.slice(0, 20).forEach((entry, idx) => {
            const pc = entry.pc.toString(16).padStart(4, '0').toUpperCase();
            const opStr = entry.m || 'UNK';
            const Y = entry.y.toString(16).padStart(4, '0').toUpperCase();
            console.log(`${idx}: ${pc}: ${opStr} Y=${Y}`);
        });
        
        console.log(`\nULTIMAS 10 entradas del bucle de delay:`);
        delayLoopEntries.slice(-10).forEach((entry, idx) => {
            const pc = entry.pc.toString(16).padStart(4, '0').toUpperCase();
            const opStr = entry.m || 'UNK';
            const Y = entry.y.toString(16).padStart(4, '0').toUpperCase();
            console.log(`${delayLoopEntries.length - 10 + idx}: ${pc}: ${opStr} Y=${Y}`);
        });
    }
    
    // Buscar últimas entradas para ver donde termina
    console.log('\n=== ÚLTIMAS 20 ENTRADAS ===');
    const last20 = traceLog.slice(-20);
    last20.forEach((entry, idx) => {
        const pc = entry.pc.toString(16).padStart(4, '0').toUpperCase();
        const opStr = entry.m || 'UNK';
        const hex = entry.hex || '';
        const operand = entry.operand || '';
        const repeat = entry.repeat > 0 ? ` [x${entry.repeat}]` : '';
        const A = entry.a.toString(16).padStart(2, '0').toUpperCase();
        const B = entry.b.toString(16).padStart(2, '0').toUpperCase();
        const X = entry.x.toString(16).padStart(4, '0').toUpperCase();
        const Y = entry.y.toString(16).padStart(4, '0').toUpperCase();
        
        console.log(`${(traceLog.length - 20 + idx).toString().padStart(4, ' ')}: ${pc}: ${opStr.padEnd(8)} (${hex}) ${operand} A=${A} B=${B} X=${X} Y=${Y}${repeat}`);
    });
    
    // Exportar trace completo como texto
    console.log('\n=== EXPORTACIÓN ===');
    const fullTrace = traceLog.map((entry, idx) => {
        const pc = entry.pc.toString(16).padStart(4, '0').toUpperCase();
        const opStr = entry.m || 'UNK';
        const hex = entry.hex || '';
        const operand = entry.operand || '';
        const repeat = entry.repeat > 0 ? ` [x${entry.repeat}]` : '';
        const A = entry.a.toString(16).padStart(2, '0').toUpperCase();
        const B = entry.b.toString(16).padStart(2, '0').toUpperCase();
        const X = entry.x.toString(16).padStart(4, '0').toUpperCase();
        const Y = entry.y.toString(16).padStart(4, '0').toUpperCase();
        const U = entry.u.toString(16).padStart(4, '0').toUpperCase();
        const S = entry.s.toString(16).padStart(4, '0').toUpperCase();
        const DP = entry.dp.toString(16).padStart(2, '0').toUpperCase();
        
        return `${idx.toString().padStart(5, ' ')}: ${pc}: ${opStr.padEnd(8)} (${hex}) ${operand}${repeat} A=${A} B=${B} X=${X} Y=${Y} U=${U} S=${S} DP=${DP}`;
    }).join('\n');
    
    // Crear descarga
    const blob = new Blob([fullTrace], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'ui_trace_analysis.txt';
    a.click();
    URL.revokeObjectURL(url);
    
    console.log('Trace completo exportado como ui_trace_analysis.txt');
}