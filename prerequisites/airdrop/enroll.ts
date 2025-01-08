import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";

// Initialize connection and wallet
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");

// Your github username as bytes
const github = Buffer.from("retrogtx", "utf8");

// Create anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed"
});

// Initialize program
const program = new Program<Turbin3Prereq>(IDL, provider);

// Create PDA for enrollment account
const enrollment_seeds = [
    Buffer.from("prereq"),
    keypair.publicKey.toBuffer()
];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
    enrollment_seeds, 
    program.programId
);

// Execute enrollment transaction
(async () => {
    try {
        console.log("Starting enrollment...");
        console.log("Public key:", keypair.publicKey.toString());
        console.log("Github:", github.toString());
        console.log("PDA:", enrollment_key.toString());
        console.log("Program ID:", program.programId.toString());

        const txhash = await program.methods
            .complete(github)
            .accounts({
                signer: keypair.publicKey,
                prereq: enrollment_key,
                systemProgram: SystemProgram.programId
            })
            .signers([keypair])
            .rpc();
            
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error("Detailed error information:");
        console.error("Error name:", (e as any).name);
        console.error("Error message:", (e as Error).message);
        if (typeof e === 'object' && e && 'error' in e) {
            console.error("Anchor error code:", (e as any).error.errorCode);
            console.error("Anchor error message:", (e as any).error.errorMessage);
        }
        console.error("Full error:", e);
    }
})(); 