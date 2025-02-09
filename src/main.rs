use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Keypair,
    signer::Signer,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let keypair = Keypair::new();
    println!("Address: {}\n", keypair.pubkey());

    let rpc_url = String::from("http://127.0.0.1:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let signature = client.request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL)?;
    println!("Transaction Signature: {}\n", signature);

    loop {
        // wait for confirmation
        if client.confirm_transaction(&signature)? {
            break;
        }
    }

    let account = client.get_account(&keypair.pubkey())?;
    println!("{:#?}\n", account);

    Ok(())
}
