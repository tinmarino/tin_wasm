
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

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

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

function logError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            let error = (function () {
                try {
                    return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
                } catch(_) {
                    return "<failed to stringify thrown value>";
                }
            }());
            console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
            throw e;
        }
    };
}
function __wbg_adapter_22(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hee84a5c5b55eba4d(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_25(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha036ac3743222727(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_28(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3a12d031c9227e14(arg0, arg1);
}

function __wbg_adapter_31(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h13f05bf8f5a661f6(arg0, arg1, addHeapObject(arg2));
}

/**
*/
export function main() {
    wasm.main();
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    };
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachegetFloat32Memory0 = null;
function getFloat32Memory0() {
    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory0;
}

function getArrayF32FromWasm0(ptr, len) {
    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Window_fbe0320f34c4cd31 = logError(function(arg0) {
        var ret = getObject(arg0) instanceof Window;
        _assertBoolean(ret);
        return ret;
    });
    imports.wbg.__wbg_document_2b44f2a86e03665a = logError(function(arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_requestAnimationFrame_65ebf8f2415064e2 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        _assertNum(ret);
        return ret;
    });
    imports.wbg.__wbg_setid_a24472f78d255d76 = logError(function(arg0, arg1, arg2) {
        getObject(arg0).id = getStringFromWasm0(arg1, arg2);
    });
    imports.wbg.__wbg_addEventListener_63378230aa6735d7 = handleError(function(arg0, arg1, arg2, arg3) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    });
    imports.wbg.__wbg_preventDefault_4eb36ec8e5563ad6 = logError(function(arg0) {
        getObject(arg0).preventDefault();
    });
    imports.wbg.__wbg_appendChild_98dedaeac24501f2 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_setProperty_e3d42ccff5ebac2f = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    });
    imports.wbg.__wbg_deltaY_f2e82bfd030f24e4 = logError(function(arg0) {
        var ret = getObject(arg0).deltaY;
        return ret;
    });
    imports.wbg.__wbg_setsrc_43904731d885a248 = logError(function(arg0, arg1, arg2) {
        getObject(arg0).src = getStringFromWasm0(arg1, arg2);
    });
    imports.wbg.__wbg_setcrossOrigin_c71214d15b663eaf = logError(function(arg0, arg1, arg2) {
        getObject(arg0).crossOrigin = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    });
    imports.wbg.__wbg_new_72aa46ede1a52e07 = handleError(function() {
        var ret = new Image();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_body_08ba7a3043ff8e77 = logError(function(arg0) {
        var ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_createElement_7cbe07ad3289abea = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_instanceof_WebGlRenderingContext_5f4db52925ef5603 = logError(function(arg0) {
        var ret = getObject(arg0) instanceof WebGLRenderingContext;
        _assertBoolean(ret);
        return ret;
    });
    imports.wbg.__wbg_bufferData_283c9170f3c599d2 = logError(function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
    });
    imports.wbg.__wbg_texImage2D_7f7a5392c50ea340 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9 === 0 ? undefined : getArrayU8FromWasm0(arg9, arg10));
    });
    imports.wbg.__wbg_texImage2D_0816311bcfcef11b = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_uniformMatrix4fv_f0d65bec707c0283 = logError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    });
    imports.wbg.__wbg_activeTexture_cfc15ec752ffb9c4 = logError(function(arg0, arg1) {
        getObject(arg0).activeTexture(arg1 >>> 0);
    });
    imports.wbg.__wbg_attachShader_b10f3a6e94e2e190 = logError(function(arg0, arg1, arg2) {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    });
    imports.wbg.__wbg_bindBuffer_cdca8a246dc033e5 = logError(function(arg0, arg1, arg2) {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    });
    imports.wbg.__wbg_bindTexture_36a5955154ca7269 = logError(function(arg0, arg1, arg2) {
        getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
    });
    imports.wbg.__wbg_clear_c7bb0cc46853ad89 = logError(function(arg0, arg1) {
        getObject(arg0).clear(arg1 >>> 0);
    });
    imports.wbg.__wbg_clearColor_8b13b519ef2dd2d7 = logError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
    });
    imports.wbg.__wbg_clearDepth_de3cfee3848f5b55 = logError(function(arg0, arg1) {
        getObject(arg0).clearDepth(arg1);
    });
    imports.wbg.__wbg_compileShader_406e03b35834cb67 = logError(function(arg0, arg1) {
        getObject(arg0).compileShader(getObject(arg1));
    });
    imports.wbg.__wbg_createBuffer_bad9101b9d0e33e7 = logError(function(arg0) {
        var ret = getObject(arg0).createBuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_createProgram_19eb97f37bc7d978 = logError(function(arg0) {
        var ret = getObject(arg0).createProgram();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_createShader_16e76257819c682b = logError(function(arg0, arg1) {
        var ret = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_createTexture_3008adb3eb4b8e63 = logError(function(arg0) {
        var ret = getObject(arg0).createTexture();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_depthFunc_6382ec1f95ff9893 = logError(function(arg0, arg1) {
        getObject(arg0).depthFunc(arg1 >>> 0);
    });
    imports.wbg.__wbg_drawElements_c16ecda7ff210a65 = logError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
    });
    imports.wbg.__wbg_enable_98a346f4f5f740b7 = logError(function(arg0, arg1) {
        getObject(arg0).enable(arg1 >>> 0);
    });
    imports.wbg.__wbg_enableVertexAttribArray_c7db971134fe1d3c = logError(function(arg0, arg1) {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    });
    imports.wbg.__wbg_getAttribLocation_8ce6818f0f36aca9 = logError(function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        _assertNum(ret);
        return ret;
    });
    imports.wbg.__wbg_getProgramInfoLog_efa3ee9c01a6c5d6 = logError(function(arg0, arg1, arg2) {
        var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    });
    imports.wbg.__wbg_getProgramParameter_f1068d691eca8e1f = logError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_getShaderInfoLog_c942a4f3fa1537cf = logError(function(arg0, arg1, arg2) {
        var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    });
    imports.wbg.__wbg_getShaderParameter_f330a1f677514bd0 = logError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_getUniformLocation_e97e7d6bc3036b6d = logError(function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_linkProgram_dc8c83ec66322d5e = logError(function(arg0, arg1) {
        getObject(arg0).linkProgram(getObject(arg1));
    });
    imports.wbg.__wbg_shaderSource_d8cce8917aa7df4a = logError(function(arg0, arg1, arg2, arg3) {
        getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
    });
    imports.wbg.__wbg_texParameteri_d05bbe24288d7efb = logError(function(arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    });
    imports.wbg.__wbg_uniform1i_7b9b67ffae83a0d7 = logError(function(arg0, arg1, arg2) {
        getObject(arg0).uniform1i(getObject(arg1), arg2);
    });
    imports.wbg.__wbg_useProgram_5a83ef734fbc034b = logError(function(arg0, arg1) {
        getObject(arg0).useProgram(getObject(arg1));
    });
    imports.wbg.__wbg_vertexAttribPointer_e809b011773856d1 = logError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    });
    imports.wbg.__wbg_log_2e875b1d2f6f87ac = logError(function(arg0) {
        console.log(getObject(arg0));
    });
    imports.wbg.__wbg_log_13fd5f3b2bfccacd = logError(function(arg0, arg1) {
        console.log(getObject(arg0), getObject(arg1));
    });
    imports.wbg.__wbg_settabIndex_e2dd6dbc0e525e2a = logError(function(arg0, arg1) {
        getObject(arg0).tabIndex = arg1;
    });
    imports.wbg.__wbg_style_854f82bcc16efd28 = logError(function(arg0) {
        var ret = getObject(arg0).style;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_setonload_2288267f14fd8110 = logError(function(arg0, arg1) {
        getObject(arg0).onload = getObject(arg1);
    });
    imports.wbg.__wbg_clientX_df24871aabb01061 = logError(function(arg0) {
        var ret = getObject(arg0).clientX;
        _assertNum(ret);
        return ret;
    });
    imports.wbg.__wbg_clientY_b91be9a030c7bd7c = logError(function(arg0) {
        var ret = getObject(arg0).clientY;
        _assertNum(ret);
        return ret;
    });
    imports.wbg.__wbg_instanceof_HtmlCanvasElement_bd2459c62d076bcd = logError(function(arg0) {
        var ret = getObject(arg0) instanceof HTMLCanvasElement;
        _assertBoolean(ret);
        return ret;
    });
    imports.wbg.__wbg_setwidth_80b60efe20240a3e = logError(function(arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    });
    imports.wbg.__wbg_setheight_5c308278bb4139ed = logError(function(arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    });
    imports.wbg.__wbg_getContext_7f0328be9fe8c1ec = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_keyCode_9de4c14bf3d88f3c = logError(function(arg0) {
        var ret = getObject(arg0).keyCode;
        _assertNum(ret);
        return ret;
    });
    imports.wbg.__wbg_newnoargs_ab5e899738c0eff4 = logError(function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_call_ab183a630df3a257 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_now_c110383288150953 = logError(function() {
        var ret = Date.now();
        return ret;
    });
    imports.wbg.__wbg_globalThis_34bac2d08ebb9b58 = handleError(function() {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_self_77eca7b42660e1bb = handleError(function() {
        var ret = self.self;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_window_51dac01569f1ba70 = handleError(function() {
        var ret = window.window;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_global_1c436164a66c9c22 = handleError(function() {
        var ret = global.global;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_new_1a48cdb23432547a = logError(function(arg0) {
        var ret = new Uint16Array(getObject(arg0));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_subarray_d8b002b2606dba35 = logError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_new_48c5d6d65ec9a035 = logError(function(arg0) {
        var ret = new Float32Array(getObject(arg0));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_subarray_c9acd3f2f4dd697d = logError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_instanceof_Memory_8a6f3ecc27e7fe29 = logError(function(arg0) {
        var ret = getObject(arg0) instanceof WebAssembly.Memory;
        _assertBoolean(ret);
        return ret;
    });
    imports.wbg.__wbg_buffer_bc64154385c04ac4 = logError(function(arg0) {
        var ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    });
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        var ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        _assertNum(ret);
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_rethrow = function(arg0) {
        throw takeObject(arg0);
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper695 = logError(function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 44, __wbg_adapter_22);
        return addHeapObject(ret);
    });
    imports.wbg.__wbindgen_closure_wrapper697 = logError(function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 46, __wbg_adapter_25);
        return addHeapObject(ret);
    });
    imports.wbg.__wbindgen_closure_wrapper699 = logError(function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 40, __wbg_adapter_28);
        return addHeapObject(ret);
    });
    imports.wbg.__wbindgen_closure_wrapper701 = logError(function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 42, __wbg_adapter_31);
        return addHeapObject(ret);
    });

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

