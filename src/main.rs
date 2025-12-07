use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use solana_address_lookup_table_program::instruction as alt_instruction;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    address_lookup_table_account::AddressLookupTableAccount,
    commitment_config::CommitmentConfig,
    hash::Hash,
    //instruction::Instruction,
    message::VersionedMessage,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction,
    transaction::VersionedTransaction,
};
use std::time::Duration;

fn load_keypair_default() -> Result<Keypair> {
    let keypair_path = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("failed to read $HOME"))?
        .join(".config")
        .join("solana")
        .join("id.json");

    read_keypair_file(keypair_path).map_err(|e| anyhow::anyhow!("{}", e)) // <-- convert to string
}

fn main() -> Result<()> {
    // RPC client
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_timeout(rpc_url.to_string(), Duration::from_secs(30));

    // Load payer keypair (~/.config/solana/id.json)
    let payer = load_keypair_default()?;
    println!("Payer: {}", payer.pubkey());

    // Generate 3 recipients locally for demo
    let recipient1 = Pubkey::new_unique();
    let recipient2 = Pubkey::new_unique();
    let recipient3 = Pubkey::new_unique();
    println!("Recipients:");
    println!("  1: {}", recipient1);
    println!("  2: {}", recipient2);
    println!("  3: {}", recipient3);

    // 1) Create Address Lookup Table (ALT) on-chain
    let recent_slot = client.get_slot()?;
    println!("Recent slot for ALT creation: {}", recent_slot);

    // create_lookup_table returns (Instruction, Pubkey)
    let (create_ix, lookup_table_pubkey) =
        alt_instruction::create_lookup_table(payer.pubkey(), payer.pubkey(), recent_slot);
    println!("Derived lookup table pubkey: {}", lookup_table_pubkey);

    let blockhash = client.get_latest_blockhash()?;
    // sign with payer (authority/payer)
    let create_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let create_sig = client.send_and_confirm_transaction(&create_tx)?;
    println!("Create ALT tx sig: {}", create_sig);

    // 2) Extend the ALT with the three recipients
    let extend_ix = alt_instruction::extend_lookup_table(
        lookup_table_pubkey,
        payer.pubkey(),
        Some(payer.pubkey()),
        vec![recipient1, recipient2, recipient3],
    );

    let blockhash = client.get_latest_blockhash()?;
    let extend_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[extend_ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let extend_sig = client.send_and_confirm_transaction(&extend_tx)?;
    println!("Extend ALT tx sig: {}", extend_sig);

    // 3) Build 3 system transfer instructions (payer -> each recipient)
    let lamports = 10_000; // small amount for demo
    let ix1 = system_instruction::transfer(&payer.pubkey(), &recipient1, lamports);
    let ix2 = system_instruction::transfer(&payer.pubkey(), &recipient2, lamports);
    let ix3 = system_instruction::transfer(&payer.pubkey(), &recipient3, lamports);

    // 4) Locally construct an AddressLookupTableAccount with the same addresses we extended
    //    so Message::try_compile can use it to produce a v0 message.
    let alt_account = AddressLookupTableAccount {
        key: lookup_table_pubkey,
        addresses: vec![recipient1, recipient2, recipient3],
    };

    // 5) Compile a v0 Message using Message::try_compile (it handles indices for us)
    // Message::try_compile signature:
    // try_compile(payer: &Pubkey, instructions: &[Instruction], address_lookup_table_accounts: &[AddressLookupTableAccount], recent_blockhash: Hash)
    use solana_sdk::message::v0::Message as V0Message;
    let blockhash: Hash = client.get_latest_blockhash()?;
    let v0_msg: V0Message =
        V0Message::try_compile(&payer.pubkey(), &[ix1, ix2, ix3], &[alt_account], blockhash)?;

    // Wrap into VersionedMessage::V0 and sign transaction
    let versioned = VersionedMessage::V0(v0_msg);
    let vtx = VersionedTransaction::try_new(versioned.clone(), &[&payer])?;

    // Serialize to base64 (bincode)
    let tx_bytes = bincode::serialize(&vtx)?;
    let b64 = general_purpose::STANDARD.encode(&tx_bytes);
    println!("\nBase64 serialized v0 transaction:\n{}\n", b64);

    // 6) Simulate via RPC (no state change)
    let sim = client.simulate_transaction_with_config(
        &vtx,
        solana_client::rpc_config::RpcSimulateTransactionConfig {
            sig_verify: true,
            replace_recent_blockhash: false, // <-- changed to false
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
    )?;

    println!("Simulation result: {:#?}", sim);

    Ok(())
}
