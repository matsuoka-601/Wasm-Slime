(()=>{"use strict";var e={61:(e,n,t)=>{let r,o;const i=new Array(128).fill(void 0);function a(e){return i[e]}i.push(void 0,null,!0,!1);let _=i.length;function c(e){const n=a(e);return function(e){e<132||(i[e]=_,_=e)}(e),n}const b="undefined"!=typeof TextDecoder?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};"undefined"!=typeof TextDecoder&&b.decode();let f=null;function u(){return null!==f&&f.buffer===o.memory.buffer||(f=new Uint8Array(o.memory.buffer)),f}function g(e,n){return e>>>=0,b.decode(u().slice(e,e+n))}function s(e){_===i.length&&i.push(i.length+1);const n=_;return _=i[n],i[n]=e,n}let w=null;function d(){return null!==w&&w.buffer===o.memory.buffer||(w=new DataView(o.memory.buffer)),w}function l(e){return null==e}let h=0;const m="undefined"!=typeof TextEncoder?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}};function p(e,n){try{return e.apply(this,n)}catch(e){o.__wbindgen_exn_store(s(e))}}"undefined"==typeof FinalizationRegistry||new FinalizationRegistry((e=>o.__wbg_simulation_free(e>>>0,1)));const y="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>o.__wbg_wbg_rayon_poolbuilder_free(e>>>0,1)));class v{static __wrap(e){e>>>=0;const n=Object.create(v.prototype);return n.__wbg_ptr=e,y.register(n,n.__wbg_ptr,n),n}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,y.unregister(this),e}free(){const e=this.__destroy_into_raw();o.__wbg_wbg_rayon_poolbuilder_free(e,0)}numThreads(){return o.wbg_rayon_poolbuilder_numThreads(this.__wbg_ptr)>>>0}receiver(){return o.wbg_rayon_poolbuilder_receiver(this.__wbg_ptr)>>>0}build(){o.wbg_rayon_poolbuilder_build(this.__wbg_ptr)}}function x(){const e={wbg:{}};return e.wbg.__wbg_log_93eba7eef122310b=function(e,n){console.log(g(e,n))},e.wbg.__wbindgen_object_drop_ref=function(e){c(e)},e.wbg.__wbindgen_boolean_get=function(e){const n=a(e);return"boolean"==typeof n?n?1:0:2},e.wbg.__wbindgen_string_new=function(e,n){return s(g(e,n))},e.wbg.__wbg_instanceof_WebGl2RenderingContext_62ccef896d9204fa=function(e){let n;try{n=a(e)instanceof WebGL2RenderingContext}catch(e){n=!1}return n},e.wbg.__wbg_bufferData_94ce174a81b32961=function(e,n,t,r){a(e).bufferData(n>>>0,a(t),r>>>0)},e.wbg.__wbg_attachShader_396d529f1d7c9abc=function(e,n,t){a(e).attachShader(a(n),a(t))},e.wbg.__wbg_bindBuffer_d6b05e0a99a752d4=function(e,n,t){a(e).bindBuffer(n>>>0,a(t))},e.wbg.__wbg_clear_7a2a7ca897047e8d=function(e,n){a(e).clear(n>>>0)},e.wbg.__wbg_clearColor_837d30f5bf4f982b=function(e,n,t,r,o){a(e).clearColor(n,t,r,o)},e.wbg.__wbg_compileShader_77ef81728b1c03f6=function(e,n){a(e).compileShader(a(n))},e.wbg.__wbg_createBuffer_7b18852edffb3ab4=function(e){const n=a(e).createBuffer();return l(n)?0:s(n)},e.wbg.__wbg_createProgram_73611dc7a72c4ee2=function(e){const n=a(e).createProgram();return l(n)?0:s(n)},e.wbg.__wbg_createShader_f10ffabbfd8e2c8c=function(e,n){const t=a(e).createShader(n>>>0);return l(t)?0:s(t)},e.wbg.__wbg_drawArrays_7a8f5031b1fe80ff=function(e,n,t,r){a(e).drawArrays(n>>>0,t,r)},e.wbg.__wbg_enableVertexAttribArray_06043f51b716ed9d=function(e,n){a(e).enableVertexAttribArray(n>>>0)},e.wbg.__wbg_getAttribLocation_df9c48b51cdad438=function(e,n,t,r){return a(e).getAttribLocation(a(n),g(t,r))},e.wbg.__wbg_getShaderInfoLog_a7ca51b89a4dafab=function(e,n,t){const r=a(n).getShaderInfoLog(a(t));var i=l(r)?0:function(e,n,t){if(void 0===t){const t=m.encode(e),r=n(t.length,1)>>>0;return u().subarray(r,r+t.length).set(t),h=t.length,r}let r=e.length,o=n(r,1)>>>0;const i=u();let a=0;for(;a<r;a++){const n=e.charCodeAt(a);if(n>127)break;i[o+a]=n}if(a!==r){0!==a&&(e=e.slice(a)),o=t(o,r,r=a+3*e.length,1)>>>0;const n=function(e,n){const t=m.encode(e);return n.set(t),{read:e.length,written:t.length}}(e,u().subarray(o+a,o+r));a+=n.written,o=t(o,r,a,1)>>>0}return h=a,o}(r,o.__wbindgen_malloc,o.__wbindgen_realloc),_=h;d().setInt32(e+4,_,!0),d().setInt32(e+0,i,!0)},e.wbg.__wbg_getShaderParameter_806970126d526c29=function(e,n,t){return s(a(e).getShaderParameter(a(n),t>>>0))},e.wbg.__wbg_getUniformLocation_6a59ad54df3bba8e=function(e,n,t,r){const o=a(e).getUniformLocation(a(n),g(t,r));return l(o)?0:s(o)},e.wbg.__wbg_linkProgram_56a5d97f63b1f56d=function(e,n){a(e).linkProgram(a(n))},e.wbg.__wbg_shaderSource_b92b2b5c29126344=function(e,n,t,r){a(e).shaderSource(a(n),g(t,r))},e.wbg.__wbg_uniform2f_3cd8a4d77e78c85d=function(e,n,t,r){a(e).uniform2f(a(n),t,r)},e.wbg.__wbg_useProgram_001c6b9208b683d3=function(e,n){a(e).useProgram(a(n))},e.wbg.__wbg_vertexAttribPointer_b435a034ff758637=function(e,n,t,r,o,i,_){a(e).vertexAttribPointer(n>>>0,t,r>>>0,0!==o,i,_)},e.wbg.__wbg_instanceof_Window_5012736c80a01584=function(e){let n;try{n=a(e)instanceof Window}catch(e){n=!1}return n},e.wbg.__wbg_width_a7c8cb533b26f0bf=function(e){return a(e).width},e.wbg.__wbg_setwidth_c20f1f8fcd5d93b4=function(e,n){a(e).width=n>>>0},e.wbg.__wbg_height_affa017f56a8fb96=function(e){return a(e).height},e.wbg.__wbg_setheight_a5e39c9d97429299=function(e,n){a(e).height=n>>>0},e.wbg.__wbg_getContext_bd2ece8a59fd4732=function(){return p((function(e,n,t){const r=a(e).getContext(g(n,t));return l(r)?0:s(r)}),arguments)},e.wbg.__wbg_performance_a1b8bde2ee512264=function(e){return s(a(e).performance)},e.wbg.__wbindgen_is_undefined=function(e){return void 0===a(e)},e.wbg.__wbg_timeOrigin_5c8b9e35719de799=function(e){return a(e).timeOrigin},e.wbg.__wbg_now_abd80e969af37148=function(e){return a(e).now()},e.wbg.__wbg_newnoargs_76313bd6ff35d0f2=function(e,n){return s(new Function(g(e,n)))},e.wbg.__wbg_call_1084a111329e68ce=function(){return p((function(e,n){return s(a(e).call(a(n)))}),arguments)},e.wbg.__wbg_self_3093d5d1f7bcb682=function(){return p((function(){return s(self.self)}),arguments)},e.wbg.__wbg_window_3bcfc4d31bc012f8=function(){return p((function(){return s(window.window)}),arguments)},e.wbg.__wbg_globalThis_86b222e13bdf32ed=function(){return p((function(){return s(globalThis.globalThis)}),arguments)},e.wbg.__wbg_global_e5a3fe56f8be9485=function(){return p((function(){return s(t.g.global)}),arguments)},e.wbg.__wbg_buffer_b7b08af79b0b0974=function(e){return s(a(e).buffer)},e.wbg.__wbg_newwithbyteoffsetandlength_a69c63d7671a5dbf=function(e,n,t){return s(new Float32Array(a(e),n>>>0,t>>>0))},e.wbg.__wbindgen_object_clone_ref=function(e){return s(a(e))},e.wbg.__wbindgen_throw=function(e,n){throw new Error(g(e,n))},e.wbg.__wbindgen_module=function(){return s(A.__wbindgen_wasm_module)},e.wbg.__wbindgen_memory=function(){return s(o.memory)},e.wbg.__wbg_startWorkers_d587c7d659590d3c=function(e,n,o){return s(async function(e,n,o){if(0===o.numThreads())throw new Error("num_threads must be > 0.");const i={module:e,memory:n,receiver:o.receiver()};r=await Promise.all(Array.from({length:o.numThreads()},(async()=>{const e=new Worker(new URL(t.p+t.u(61),t.b),{type:void 0});return e.postMessage(i),await new Promise((n=>e.addEventListener("message",n,{once:!0}))),e}))),o.build()}(c(e),c(n),v.__wrap(o)))},e}async function A(e,n){if(void 0!==o)return o;let r;void 0!==e&&Object.getPrototypeOf(e)===Object.prototype?({module_or_path:e,memory:n,thread_stack_size:r}=e):console.warn("using deprecated parameters for the initialization function; pass a single object instead"),void 0===e&&(e=new URL(t(279),t.b));const i=x();("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e)),function(e,n){e.wbg.memory=n||new WebAssembly.Memory({initial:18,maximum:16384,shared:!0})}(i,n);const{instance:a,module:_}=await async function(e,n){if("function"==typeof Response&&e instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(e,n)}catch(n){if("application/wasm"==e.headers.get("Content-Type"))throw n;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",n)}const t=await e.arrayBuffer();return await WebAssembly.instantiate(t,n)}{const t=await WebAssembly.instantiate(e,n);return t instanceof WebAssembly.Instance?{instance:t,module:e}:t}}(await e,i);return function(e,n,t){if(o=e.exports,A.__wbindgen_wasm_module=n,w=null,f=null,void 0!==t&&("number"!=typeof t||0===t||t%65536!=0))throw"invalid stack size";return o.__wbindgen_start(t),o}(a,_,r)}const S=A;onmessage=async({data:{receiver:e,...n}})=>{await S(n),postMessage(!0),function(e){o.wbg_rayon_start_worker(e)}(e)}},279:(e,n,t)=>{e.exports=t.p+"19e0bf29bfa18b72eafb.wasm"}},n={};function t(r){var o=n[r];if(void 0!==o)return o.exports;var i=n[r]={exports:{}};return e[r](i,i.exports,t),i.exports}t.m=e,t.u=e=>e+".index.js",t.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),t.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),(()=>{var e;t.g.importScripts&&(e=t.g.location+"");var n=t.g.document;if(!e&&n&&(n.currentScript&&"SCRIPT"===n.currentScript.tagName.toUpperCase()&&(e=n.currentScript.src),!e)){var r=n.getElementsByTagName("script");if(r.length)for(var o=r.length-1;o>-1&&(!e||!/^http(s?):/.test(e));)e=r[o--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),t.p=e})(),t.b=self.location+"",t(61)})();