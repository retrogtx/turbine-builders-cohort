use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file},
    pubkey::Pubkey
};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    system_instruction::transfer,
};
use solana_sdk::{
    transaction::Transaction
};
use std::str::FromStr;
use solana_sdk::{
    message::Message,
};
mod programs;

const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)] mod tests {
    use solana_sdk;
    #[test]
    fn keygen() {} #[test] fn airdop() {} #[test] fn transfer_sol() {}
    }


#[test]
fn keygen() {
    // Create a new keypair
    let kp = Keypair::new();
    println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
}

// Convert base58 to wallet 
use base58;
use std::io::{self, BufRead};
pub fn base58_to_wallet() {
    println!("Input your private key as a byte array (comma-separated numbers):");
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read all lines until we get a closing bracket
    while !input.contains(']') {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        input.push_str(&line);
    }
    
    // Clean the input: remove brackets, newlines, and extra whitespace
    let cleaned_input = input
        .replace('\n', "")
        .replace('[', "")
        .replace(']', "")
        .replace(" ", "");
    
    // Parse the cleaned input
    let bytes: Vec<u8> = cleaned_input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    // Convert bytes to base58
    let base58_key = base58::ToBase58::to_base58(bytes.as_slice());
    println!("Private Key (base58): {}", base58_key);
}

// Convert wallet to base58
pub fn wallet_to_base58() {
    println!("Input your base58 private key:");
    let stdin = io::stdin();
    let base58_key = stdin.lock().lines().next().unwrap().unwrap();
    
    // Decode base58 string to bytes
    let bytes = base58::FromBase58::from_base58(base58_key.as_str()).unwrap();
    println!("Wallet bytes:");
    println!("{:?}", bytes);
}

#[test]
fn airdrop() {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    
    // Connected to Solana Devnet RPC Client
    let client = RpcClient::new(RPC_URL);
    
    // We're going to claim 2 devnet SOL tokens (2 billion lamports)
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(s) => {
            println!("Success! Check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
        },
        Err(e) => println!("Oops, something went wrong: {}", e.to_string())
    };
}

pub fn request_airdrop() {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    
    // Connected to Solana Devnet RPC Client
    let client = RpcClient::new(RPC_URL);
    
    println!("Requesting airdrop for: {}", keypair.pubkey());
    
    // We're going to claim 2 devnet SOL tokens (2 billion lamports)
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(s) => {
            println!("Success! Check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
        },
        Err(e) => println!("Oops, something went wrong: {}", e.to_string())
    };
}

#[test]
fn transfer_sol() {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    
    // Define our Turbin3 public key
    let to_pubkey = Pubkey::from_str("YOUR_TURBIN3_PUBLIC_KEY_HERE").unwrap();
    
    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);
    
    // Get recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    
    // Create transfer transaction
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            100_000_000 // 0.1 SOL = 100_000_000 lamports
        )],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash
    );
    
    // Send the transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
        
    // Print our transaction out
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}


pub fn transfer_to_turbin3(turbin3_address: &str) {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    
    // Define our Turbin3 public key
    let to_pubkey = Pubkey::from_str(turbin3_address).unwrap();
    
    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);
    
    println!("Transferring 0.1 SOL to {}", turbin3_address);
    
    // Get recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    
    // Create transfer transaction
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            100_000_000 // 0.1 SOL = 100_000_000 lamports
        )],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash
    );
    
    // Send the transaction
    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!(
                "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                signature
            );
        },
        Err(e) => println!("Failed to send transaction: {}", e)
    };
}

pub fn empty_wallet_to_turbin3(turbin3_address: &str) {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    
    // Define our Turbin3 public key
    let to_pubkey = Pubkey::from_str(turbin3_address).unwrap();
    
    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);
    
    // Get recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    
    // Get balance of dev wallet
    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
        
    println!("Current wallet balance: {} lamports", balance);
    
    // Create a test transaction to calculate fees
    let message = Message::new_with_blockhash(
        &[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            balance,
        )],
        Some(&keypair.pubkey()),
        &recent_blockhash
    );
    
    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");
        
    println!("Transaction fee: {} lamports", fee);
    println!("Transferring {} lamports to {}", balance - fee, turbin3_address);
    
    // Deduct fee from lamports amount and create a TX with correct balance
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            balance - fee,
        )],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash
    );
    
    // Send the transaction
    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!(
                "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                signature
            );
        },
        Err(e) => println!("Failed to send transaction: {}", e)
    };
}

pub fn complete_prerequisite(github_username: &str) {
    use crate::programs::turbin3_prereq::WbaPrereq;
    
    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);
    
    // Let's define our accounts
    let signer = read_keypair_file("turbin3-wallet.json")
        .expect("Couldn't find turbin3-wallet.json file");
        
    println!("Completing prerequisite for GitHub user: {}", github_username);
    
    // Create program instance
    let program = WbaPrereq::new(rpc_client, signer);
    
    // Convert string to bytes
    let github_bytes = github_username.as_bytes().to_vec();
    
    // Call the complete instruction
    match program.complete(github_bytes) {
        Ok(signature) => {
            println!(
                "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                signature
            );
        },
        Err(e) => println!("Failed to complete prerequisite: {}", e)
    }
}