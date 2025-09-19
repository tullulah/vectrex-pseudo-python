let wasm;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

const WasmEmuFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmemu_free(ptr >>> 0, 1));

export class WasmEmu {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmEmuFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmemu_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.wasmemu_new();
        this.__wbg_ptr = ret >>> 0;
        WasmEmuFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Uint8Array} data
     * @returns {boolean}
     */
    load_bios(data) {
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export_0);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmemu_load_bios(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * @param {number} base
     * @param {Uint8Array} data
     */
    load_bin(base, data) {
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export_0);
        const len0 = WASM_VECTOR_LEN;
        wasm.wasmemu_load_bin(this.__wbg_ptr, base, ptr0, len0);
    }
    reset() {
        wasm.wasmemu_reset(this.__wbg_ptr);
    }
    reset_stats() {
        wasm.wasmemu_reset_stats(this.__wbg_ptr);
    }
    /**
     * @param {number} count
     * @returns {number}
     */
    step(count) {
        const ret = wasm.wasmemu_step(this.__wbg_ptr, count);
        return ret >>> 0;
    }
    /**
     * Ejecuta instrucciones hasta que el frame_count cambie (heurística WAIT_RECAL) o se alcance el límite.
     * Devuelve el número de instrucciones ejecutadas. Reintroducido tras refactor.
     * @param {number} max_instructions
     * @returns {number}
     */
    run_until_wait_recal(max_instructions) {
        const ret = wasm.wasmemu_run_until_wait_recal(this.__wbg_ptr, max_instructions);
        return ret >>> 0;
    }
    /**
     * @returns {string}
     */
    registers_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmemu_registers_json(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_1(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    memory_ptr() {
        const ret = wasm.wasmemu_memory_ptr(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Read a single byte from unified bus memory (debug helper for JS console).
     * @param {number} addr
     * @returns {number}
     */
    read_mem8(addr) {
        const ret = wasm.wasmemu_read_mem8(this.__wbg_ptr, addr);
        return ret;
    }
    /**
     * Return the base address where BIOS was loaded (F000 for 4K, E000 for 8K) or default if not present yet.
     * @returns {number}
     */
    bios_base() {
        const ret = wasm.wasmemu_bios_base(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {boolean} en
     * @param {number} limit
     */
    enable_trace(en, limit) {
        wasm.wasmemu_enable_trace(this.__wbg_ptr, en, limit);
    }
    trace_clear() {
        wasm.wasmemu_trace_clear(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    trace_len() {
        const ret = wasm.wasmemu_trace_len(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {string}
     */
    trace_log_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmemu_trace_log_json(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_1(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    metrics_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmemu_metrics_json(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_1(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    integrator_segments_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmemu_integrator_segments_json(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_1(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Devuelve las últimas llamadas BIOS registradas (máx 256) en formato JSON array de strings "FFFF:LABEL".
     * @returns {string}
     */
    bios_calls_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmemu_bios_calls_json(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_1(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Limpia el buffer de llamadas BIOS (útil en depuración / reinicios parciales en la UI).
     */
    clear_bios_calls() {
        wasm.wasmemu_clear_bios_calls(this.__wbg_ptr);
    }
    /**
     * @returns {string}
     */
    integrator_segments_peek_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmemu_integrator_segments_peek_json(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_1(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    integrator_segments_ptr() {
        const ret = wasm.wasmemu_integrator_segments_ptr(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    integrator_segments_len() {
        const ret = wasm.wasmemu_integrator_segments_len(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    integrator_segment_stride() {
        const ret = wasm.wasmemu_integrator_segment_stride(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Devuelve el número de segmentos actualmente acumulados SIN copiar ni drenar.
     * Útil para saber si hay algo antes de decidir usar JSON o acceso compartido.
     * @returns {number}
     */
    integrator_segments_count() {
        const ret = wasm.wasmemu_integrator_segments_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    integrator_drain_segments() {
        wasm.wasmemu_integrator_drain_segments(this.__wbg_ptr);
    }
    demo_triangle() {
        wasm.wasmemu_demo_triangle(this.__wbg_ptr);
    }
    /**
     * @param {boolean} en
     */
    set_auto_demo(en) {
        wasm.wasmemu_set_auto_demo(this.__wbg_ptr, en);
    }
    /**
     * @returns {boolean}
     */
    auto_demo_enabled() {
        const ret = wasm.wasmemu_auto_demo_enabled(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {string}
     */
    loop_watch_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmemu_loop_watch_json(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_1(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {boolean} en
     */
    set_irq_frame_fallback(en) {
        wasm.wasmemu_set_irq_frame_fallback(this.__wbg_ptr, en);
    }
    /**
     * @returns {boolean}
     */
    irq_frame_fallback() {
        const ret = wasm.wasmemu_irq_frame_fallback(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} merge
     */
    set_integrator_merge_lines(merge) {
        wasm.wasmemu_set_integrator_merge_lines(this.__wbg_ptr, merge);
    }
    /**
     * @returns {boolean}
     */
    integrator_merge_lines() {
        const ret = wasm.wasmemu_integrator_merge_lines(this.__wbg_ptr);
        return ret !== 0;
    }
    reset_integrator_segments() {
        wasm.wasmemu_reset_integrator_segments(this.__wbg_ptr);
    }
    /**
     * @param {boolean} en
     */
    set_integrator_auto_drain(en) {
        wasm.wasmemu_set_integrator_auto_drain(this.__wbg_ptr, en);
    }
    /**
     * @returns {boolean}
     */
    integrator_auto_drain() {
        const ret = wasm.wasmemu_integrator_auto_drain(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Actualiza estado de entrada (joystick analógico -128..127, botones bits 0..3)
     * @param {number} x
     * @param {number} y
     * @param {number} buttons
     */
    set_input_state(x, y, buttons) {
        wasm.wasmemu_set_input_state(this.__wbg_ptr, x, y, buttons);
    }
}

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;



    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('vectrex_emulator_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
