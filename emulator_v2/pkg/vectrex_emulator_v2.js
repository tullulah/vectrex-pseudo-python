let wasm;

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

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

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

export function wasm_init() {
    wasm.wasm_init();
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

const VectorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_vector_free(ptr >>> 0, 1));
/**
 * Vector structure matching JSVecx vector_t
 * JSVecx Original: function vector_t() { this.x0 = 0; this.y0 = 0; this.x1 = 0; this.y1 = 0; this.color = 0; }
 */
export class Vector {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Vector.prototype);
        obj.__wbg_ptr = ptr;
        VectorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VectorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vector_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get x0() {
        const ret = wasm.__wbg_get_vector_x0(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x0(arg0) {
        wasm.__wbg_set_vector_x0(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y0() {
        const ret = wasm.__wbg_get_vector_y0(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set y0(arg0) {
        wasm.__wbg_set_vector_y0(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get x1() {
        const ret = wasm.__wbg_get_vector_x1(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x1(arg0) {
        wasm.__wbg_set_vector_x1(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y1() {
        const ret = wasm.__wbg_get_vector_y1(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set y1(arg0) {
        wasm.__wbg_set_vector_y1(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get color() {
        const ret = wasm.__wbg_get_vector_color(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set color(arg0) {
        wasm.__wbg_set_vector_color(this.__wbg_ptr, arg0);
    }
    constructor() {
        const ret = wasm.vector_new();
        this.__wbg_ptr = ret >>> 0;
        VectorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const VectrexEmulatorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_vectrexemulator_free(ptr >>> 0, 1));
/**
 * Main WASM Emulator class matching JSVecx VecX API
 * JSVecx Original: function VecX()
 */
export class VectrexEmulator {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VectrexEmulatorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vectrexemulator_free(ptr, 0);
    }
    /**
     * Constructor matching JSVecx: new VecX()
     */
    constructor() {
        const ret = wasm.vectrexemulator_new();
        this.__wbg_ptr = ret >>> 0;
        VectrexEmulatorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Initialize emulator with BIOS
     * JSVecx Pattern: init() + loadBios()
     * Auto-loads embedded BIOS ROM (8192 bytes: 4KB real + 4KB padding)
     * @returns {boolean}
     */
    init() {
        const ret = wasm.vectrexemulator_init(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Load BIOS from bytes (for custom BIOS)
     * JSVecx Pattern: loadBiosFromBytes() (custom extension for WASM)
     * @param {Uint8Array} bios_data
     * @returns {boolean}
     */
    loadBiosBytes(bios_data) {
        const ptr0 = passArray8ToWasm0(bios_data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.vectrexemulator_loadBiosBytes(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Load ROM/cartridge
     * JSVecx Pattern: loadRom(file)
     * @param {string} rom_path
     * @returns {boolean}
     */
    loadRom(rom_path) {
        const ptr0 = passStringToWasm0(rom_path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.vectrexemulator_loadRom(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Reset emulator
     * JSVecx Pattern: reset()
     */
    reset() {
        wasm.vectrexemulator_reset(this.__wbg_ptr);
    }
    /**
     * Start emulation loop
     * JSVecx Pattern: start()
     */
    start() {
        wasm.vectrexemulator_start(this.__wbg_ptr);
    }
    /**
     * Stop emulation loop
     * JSVecx Pattern: stop()
     */
    stop() {
        wasm.vectrexemulator_stop(this.__wbg_ptr);
    }
    /**
     * Check if running
     * JSVecx Pattern: isRunning()
     * @returns {boolean}
     */
    isRunning() {
        const ret = wasm.vectrexemulator_isRunning(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Execute one frame (called by JS animation loop)
     * JSVecx Pattern: vecx_emu(cycles, 0) called in loop
     * @param {bigint} cycles
     */
    runFrame(cycles) {
        wasm.vectrexemulator_runFrame(this.__wbg_ptr, cycles);
    }
    /**
     * Get vector count
     * JSVecx Pattern: this.vector_draw_cnt
     * @returns {number}
     */
    getVectorCount() {
        const ret = wasm.vectrexemulator_getVectorCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get vector at index (returns raw values for JS to construct object)
     * JSVecx Pattern: accessing this.vectors_draw[i]
     * @param {number} index
     * @returns {Vector | undefined}
     */
    getVector(index) {
        const ret = wasm.vectrexemulator_getVector(this.__wbg_ptr, index);
        return ret === 0 ? undefined : Vector.__wrap(ret);
    }
    /**
     * Get all vectors as JSON
     * JSVecx Extension: For easier consumption from JS
     * @returns {string}
     */
    getVectorsJson() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.vectrexemulator_getVectorsJson(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get metrics as JSON
     * JSVecx Pattern: getMetrics() returns { totalCycles, instructionCount, frameCount, running }
     * @returns {string}
     */
    getMetrics() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.vectrexemulator_getMetrics(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get CPU registers as JSON
     * JSVecx Pattern: getRegisters() returns { PC, A, B, X, Y, U, S, DP, CC }
     * @returns {string}
     */
    getRegisters() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.vectrexemulator_getRegisters(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Read memory byte
     * JSVecx Pattern: read8(address)
     * @param {number} address
     * @returns {number}
     */
    read8(address) {
        const ret = wasm.vectrexemulator_read8(this.__wbg_ptr, address);
        return ret;
    }
    /**
     * Write memory byte
     * JSVecx Pattern: write8(address, value)
     * @param {number} address
     * @param {number} value
     */
    write8(address, value) {
        wasm.vectrexemulator_write8(this.__wbg_ptr, address, value);
    }
    /**
     * Handle key down
     * JSVecx Pattern: onkeydown(event)
     * @param {number} key_code
     */
    onKeyDown(key_code) {
        wasm.vectrexemulator_onKeyDown(this.__wbg_ptr, key_code);
    }
    /**
     * Handle key up
     * JSVecx Pattern: onkeyup(event)
     * @param {number} key_code
     */
    onKeyUp(key_code) {
        wasm.vectrexemulator_onKeyUp(this.__wbg_ptr, key_code);
    }
    /**
     * Set joystick position directly (-127 to 127)
     * Extension: For programmatic control
     * @param {number} x
     * @param {number} y
     */
    setJoystick(x, y) {
        wasm.vectrexemulator_setJoystick(this.__wbg_ptr, x, y);
    }
    /**
     * Set button state
     * Extension: For programmatic control
     * @param {number} button
     * @param {boolean} pressed
     */
    setButton(button, pressed) {
        wasm.vectrexemulator_setButton(this.__wbg_ptr, button, pressed);
    }
    /**
     * Get Program Counter
     * @returns {number}
     */
    getPC() {
        const ret = wasm.vectrexemulator_getPC(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get register A
     * @returns {number}
     */
    getA() {
        const ret = wasm.vectrexemulator_getA(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get register B
     * @returns {number}
     */
    getB() {
        const ret = wasm.vectrexemulator_getB(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get register D (A:B concatenated)
     * @returns {number}
     */
    getD() {
        const ret = wasm.vectrexemulator_getD(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get register X
     * @returns {number}
     */
    getX() {
        const ret = wasm.vectrexemulator_getX(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get register Y
     * @returns {number}
     */
    getY() {
        const ret = wasm.vectrexemulator_getY(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get register U (User stack pointer)
     * @returns {number}
     */
    getU() {
        const ret = wasm.vectrexemulator_getU(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get register S (System stack pointer)
     * @returns {number}
     */
    getS() {
        const ret = wasm.vectrexemulator_getS(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get Direct Page register
     * @returns {number}
     */
    getDP() {
        const ret = wasm.vectrexemulator_getDP(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get Condition Codes register
     * @returns {number}
     */
    getCC() {
        const ret = wasm.vectrexemulator_getCC(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get total cycles executed
     * @returns {bigint}
     */
    getTotalCycles() {
        const ret = wasm.vectrexemulator_getTotalCycles(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Read byte from memory (for debugging)
     * @param {number} address
     * @returns {number}
     */
    readMemory(address) {
        const ret = wasm.vectrexemulator_read8(this.__wbg_ptr, address);
        return ret;
    }
    /**
     * Execute single instruction (for step debugging)
     */
    step() {
        wasm.vectrexemulator_step(this.__wbg_ptr);
    }
    /**
     * Get last error message (for debugging)
     * @returns {string}
     */
    getLastError() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.vectrexemulator_getLastError(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get last PC (for debugging panics)
     * @returns {number}
     */
    getLastPC() {
        const ret = wasm.vectrexemulator_getLastPC(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get last opcode (for debugging panics)
     * @returns {number}
     */
    getLastOpcode() {
        const ret = wasm.vectrexemulator_getLastOpcode(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get PC history as JSON string (last N instructions before current)
     * @returns {string}
     */
    getPCHistory() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.vectrexemulator_getPCHistory(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
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
    imports.wbg.__wbg_call_2f8d426a20a307fe = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_call_f53f0647ceb9c567 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.call(arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_crypto_574e78ad8b13b65f = function(arg0) {
        const ret = arg0.crypto;
        return ret;
    };
    imports.wbg.__wbg_error_41f0589870426ea4 = function(arg0) {
        console.error(arg0);
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_getRandomValues_b8f5dbd5f3995a9e = function() { return handleError(function (arg0, arg1) {
        arg0.getRandomValues(arg1);
    }, arguments) };
    imports.wbg.__wbg_length_904c0910ed998bf3 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_msCrypto_a61aeb35a24c1329 = function(arg0) {
        const ret = arg0.msCrypto;
        return ret;
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_newnoargs_a81330f6e05d8aca = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_newwithlength_ed0ee6c1edca86fc = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_node_905d3e251edff8a2 = function(arg0) {
        const ret = arg0.node;
        return ret;
    };
    imports.wbg.__wbg_process_dc0fbacc7c1c06f7 = function(arg0) {
        const ret = arg0.process;
        return ret;
    };
    imports.wbg.__wbg_prototypesetcall_c5f74efd31aea86b = function(arg0, arg1, arg2) {
        Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
    };
    imports.wbg.__wbg_randomFillSync_ac0988aba3254290 = function() { return handleError(function (arg0, arg1) {
        arg0.randomFillSync(arg1);
    }, arguments) };
    imports.wbg.__wbg_require_60cc747a6bc5215a = function() { return handleError(function () {
        const ret = module.require;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_1f13249cc3acc96d = function() {
        const ret = typeof global === 'undefined' ? null : global;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_THIS_df7ae94b1e0ed6a3 = function() {
        const ret = typeof globalThis === 'undefined' ? null : globalThis;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_SELF_6265471db3b3c228 = function() {
        const ret = typeof self === 'undefined' ? null : self;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_WINDOW_16fb482f8ec52863 = function() {
        const ret = typeof window === 'undefined' ? null : window;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_subarray_a219824899e59712 = function(arg0, arg1, arg2) {
        const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_versions_c01dfd4722a88165 = function(arg0) {
        const ret = arg0.versions;
        return ret;
    };
    imports.wbg.__wbg_warn_07ef1f61c52799fb = function(arg0) {
        console.warn(arg0);
    };
    imports.wbg.__wbg_wbindgenisfunction_ea72b9d66a0e1705 = function(arg0) {
        const ret = typeof(arg0) === 'function';
        return ret;
    };
    imports.wbg.__wbg_wbindgenisobject_dfe064a121d87553 = function(arg0) {
        const val = arg0;
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_wbindgenisstring_4b74e4111ba029e6 = function(arg0) {
        const ret = typeof(arg0) === 'string';
        return ret;
    };
    imports.wbg.__wbg_wbindgenisundefined_71f08a6ade4354e7 = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbg_wbindgenthrow_4c11a24fca429ccf = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
        // Cast intrinsic for `Ref(String) -> Externref`.
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_cast_cb9088102bce6b30 = function(arg0, arg1) {
        // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
        const ret = getArrayU8FromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_2;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
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


    wasm.__wbindgen_start();
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
        module_or_path = new URL('vectrex_emulator_v2_bg.wasm', import.meta.url);
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
