use solana_program::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn complete(
    signer: &Pubkey,
    prereq: &Pubkey,
    system_program: &Pubkey,
    github: Vec<u8>,
) -> Instruction {
    // Create instruction data with 8-byte discriminator
    let mut data = vec![245, 3, 151, 124, 0, 0, 0, 0];  // 8-byte discriminator for 'complete'
    data.extend(github);  // Add github username directly after discriminator

    Instruction {
        program_id: Pubkey::from_str("HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1").unwrap(),
        accounts: vec![
            AccountMeta::new(*signer, true),
            AccountMeta::new(*prereq, false),
            AccountMeta::new_readonly(*system_program, false),
        ],
        data,
    }
} 