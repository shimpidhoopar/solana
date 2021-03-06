use solana_drone::drone::{request_airdrop_transaction, run_local_drone};
use solana_sdk::hash::Hash;
use solana_sdk::message::Message;
use solana_sdk::signature::{Keypair, KeypairUtil};
use solana_sdk::system_instruction::SystemInstruction;
use solana_sdk::transaction::Transaction;
use std::sync::mpsc::channel;

#[test]
fn test_local_drone() {
    let keypair = Keypair::new();
    let to = Keypair::new().pubkey();
    let lamports = 50;
    let blockhash = Hash::new(&to.as_ref());
    let create_instruction = SystemInstruction::new_account(&keypair.pubkey(), &to, lamports);
    let message = Message::new(vec![create_instruction]);
    let expected_tx = Transaction::new(&[&keypair], message, blockhash);

    let (sender, receiver) = channel();
    run_local_drone(keypair, sender);
    let drone_addr = receiver.recv().unwrap();

    let result = request_airdrop_transaction(&drone_addr, &to, lamports, blockhash);
    assert_eq!(expected_tx, result.unwrap());
}
