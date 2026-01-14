// Background script

import init, { check_grammar_wasm, enhance_text_wasm } from './akkurate_extension.js';

let wasmLoaded = false;

async function loadWasm() {
    if (!wasmLoaded) {
        await init();
        wasmLoaded = true;
        console.log("Akkurate WASM loaded in background");
    }
}

// Initialize WASM on start
loadWasm();

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    if (request.action === "check_grammar") {
        handleCheck(request.text, request.apiKey).then(sendResponse);
        return true; // async response
    }
    if (request.action === "enhance_text") {
        handleEnhance(request.text, request.apiKey, request.preset).then(sendResponse);
        return true;
    }
});

async function handleCheck(text, apiKey) {
    await loadWasm();
    try {
        const resultJson = await check_grammar_wasm(text, apiKey);
        return { success: true, data: JSON.parse(resultJson) };
    } catch (e) {
        return { success: false, error: e.toString() };
    }
}

async function handleEnhance(text, apiKey, preset) {
    await loadWasm();
    try {
        const resultJson = await enhance_text_wasm(text, apiKey, preset);
        return { success: true, data: JSON.parse(resultJson) };
    } catch (e) {
        return { success: false, error: e.toString() };
    }
}
