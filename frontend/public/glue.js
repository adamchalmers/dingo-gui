const invoke = window.__TAURI__.invoke

export async function invokeResolve(hostname) {
    return await invoke("resolve", {hostname: hostname});
}
