
/* @ts-ignore */
import wasm from '../wasm/pkg/wasm_bg.wasm?module'
import init, { async_test } from '../wasm/pkg/wasm.js';
export const config = {
  runtime: 'edge',
}

export default async function handler(request: Request, event: Event) {
    await init(wasm);

    const url = new URL(request.url)
    const a = +(url.searchParams.get('a') as string);
    try {
        const value = await async_test(a);
        return new Response("Value is: " + value)
    } catch (e) {
        console.log("E me daddy", e);
    }

    return new Response("YOU SUCK")
}
