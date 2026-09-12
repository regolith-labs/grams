use grams_api::prelude::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    signature::{read_keypair_file, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;

#[tokio::main]
async fn main() {
    let rpc = RpcClient::new(std::env::var("RPC").expect("Missing RPC env var"));
    let command = std::env::args().nth(1).expect("Usage: ore-cli <balance|run>");

    match command.as_str() {
        "balance" => balance(&rpc).await.unwrap(),
        "run" => run(&rpc).await.unwrap(),
        _ => panic!("Unknown command: {}. Usage: ore-cli <balance|run>", command),
    }
}

async fn balance(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let receivables_address = receivables_pda().0;
    let ore_ata = get_associated_token_address(&receivables_address, &MINT_ADDRESS);
    let store_ata = get_associated_token_address(&receivables_address, &STORE_MINT_ADDRESS);

    println!("Receivables: {}", receivables_address);
    println!("  ORE ATA:   {}", ore_ata);
    println!("  stORE ATA: {}", store_ata);

    // ORE balance
    match rpc.get_token_account_balance(&ore_ata).await {
        Ok(b) => println!("  ORE:       {}", b.ui_amount_string),
        Err(_) => println!("  ORE:       0 (ATA not found)"),
    }

    // stORE balance
    match rpc.get_token_account_balance(&store_ata).await {
        Ok(b) => println!("  stORE:     {}", b.ui_amount_string),
        Err(_) => println!("  stORE:     0 (ATA not found)"),
    }

    Ok(())
}

async fn run(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let payer =
        read_keypair_file(&std::env::var("KEYPAIR").expect("Missing KEYPAIR env var")).unwrap();

    // Build unwrap + bury instructions
    let unwrap_ix = grams_api::sdk::unwrap(payer.pubkey());
    let bury_ix = grams_api::sdk::bury();

    // Submit transaction
    let blockhash = rpc.get_latest_blockhash().await?;
    let transaction = Transaction::new_signed_with_payer(
        &[
            ComputeBudgetInstruction::set_compute_unit_limit(500_000),
            ComputeBudgetInstruction::set_compute_unit_price(100_000),
            unwrap_ix,
            bury_ix,
        ],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    match rpc.send_and_confirm_transaction(&transaction).await {
        Ok(sig) => println!("Success: {}", sig),
        Err(e) => println!("Error: {:?}", e),
    }

    Ok(())
}
