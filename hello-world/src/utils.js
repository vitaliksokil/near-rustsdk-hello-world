import * as nearAPI from 'near-api-js';
import getConfig from './config.js';

const nearConfig = getConfig(process.env.VUE_APP_NEAR_ENV || 'testnet');

async function getHelloMessage(name) {
    if (name){
        const provider = new nearAPI.providers.JsonRpcProvider(nearConfig.nodeUrl);
        const argsBase64 = window.btoa(JSON.stringify({name:name}))
        const rawResult = await provider.query({
            request_type: "call_function",
            finality: "final",
            account_id: nearConfig.contractName,
            method_name: "get_hello_message",
            args_base64: argsBase64,
        }); // Base 58 of '{}'
        return JSON.parse(rawResult.result.map((x) => String.fromCharCode(x)).join(''));
    }
}

export {getHelloMessage}