use solana_idlgen::idlgen;

idlgen!({
    "version": "0.1.0",
    "name": "wba_prereq",
    "instructions": [
        {
            "name": "complete",
            "accounts": [
                {
                    "name": "signer",
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "isMut": true,
                    "isSigner": false
                },
                {
                    "name": "systemProgram",
                    "isMut": false,
                    "isSigner": false
                }
            ],
            "args": [
                {
                    "name": "github",
                    "type": "bytes"
                }
            ]
        },
        {
            "name": "update",
            "accounts": [
                {
                    "name": "signer",
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "isMut": true,
                    "isSigner": false
                },
                {
                    "name": "systemProgram",
                    "isMut": false,
                    "isSigner": false
                }
            ],
            "args": [
                {
                    "name": "github",
                    "type": "bytes"
                }
            ]
        }
    ],
    "accounts": [
        {
            "name": "PrereqAccount",
            "type": {
                "kind": "struct",
                "fields": [
                    {
                        "name": "github",
                        "type": "bytes"
                    },
                    {
                        "name": "key",
                        "type": "publicKey"
                    }
                ]
            }
        }
    ],
    "errors": [
        {
            "code": 6000,
            "name": "InvalidGithubAccount",
            "msg": "Invalid Github account"
        }
    ],
    "metadata": {
        "address": "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1"
    }
});

use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction, system_program};
use std::str::FromStr;

// The struct name should match what the IDL generates
pub struct WbaPrereq {
    pub client: RpcClient,
    pub payer: solana_sdk::signature::Keypair,
}

impl WbaPrereq {
    pub fn new(client: RpcClient, payer: solana_sdk::signature::Keypair) -> Self {
        Self { client, payer }
    }

    pub fn derive_program_address(seeds: &[&[u8]]) -> Pubkey {
        let program_id = Pubkey::from_str("HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1").unwrap();
        let (pda, _bump) = Pubkey::find_program_address(seeds, &program_id);
        pda
    }

    pub fn complete(&self, github: Vec<u8>) -> Result<String, String> {
        let prereq = Self::derive_program_address(&[
            b"prereq",
            self.payer.pubkey().to_bytes().as_ref()
        ]);

        let blockhash = self.client
            .get_latest_blockhash()
            .map_err(|e| e.to_string())?;
        let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
            &[instruction::complete(
                &self.payer.pubkey(),
                &prereq,
                &system_program::id(),
                github,
            )],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            blockhash
        );

        self.client
            .send_and_confirm_transaction(&transaction)
            .map(|sig| sig.to_string())
            .map_err(|e| e.to_string())
    }
} 