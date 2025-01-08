import bs58 from 'bs58';
import prompt from 'prompt-sync';

const input = prompt({ sigint: true });

function base58ToWallet() {
    const base58Key = input('Enter base58 private key: ');
    const wallet = bs58.decode(base58Key);
    console.log('Wallet bytes:', Array.from(wallet));
}

function walletToBase58() {
    const rawKey = input('Enter array of wallet bytes: ');
    try {
        const bytes = Uint8Array.from(JSON.parse(rawKey));
        const base58 = bs58.encode(bytes);
        console.log('Base58 key:', base58);
    } catch (e) {
        console.error('Invalid byte array format');
    }
}

console.log('1. Base58 to Wallet bytes');
console.log('2. Wallet bytes to Base58');
const choice = input('Choose conversion (1/2): ');

choice === '1' ? base58ToWallet() : walletToBase58(); 