use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    litesvm::types::TransactionMetadata,
    litesvm::types::FailedTransactionMetadata,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_keypair::Keypair,
    solana_transaction::versioned::VersionedTransaction,
};

fn send_limit_range(
    svm: &mut LiteSVM,
    payer: &Keypair,
    a: u64,
) -> Result<TransactionMetadata, FailedTransactionMetadata> {
    let program_id = prc_sol::id();  

    let instruction = Instruction::new_with_bytes(
        program_id,
        &prc_sol::instruction::LimitRange { a }.data(),  // ✅ prc_sol
        prc_sol::accounts::LimitRange {}.to_account_metas(None),  // ✅ prc_sol
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(
        &[instruction],
        Some(&payer.pubkey()),
        &blockhash,
    );
    let tx = VersionedTransaction::try_new(
        VersionedMessage::Legacy(msg),
        &[payer],
    ).unwrap();

    svm.send_transaction(tx)  // ✅ no Ok() wrapper needed
}

fn send_func(
    svm: &mut LiteSVM,
    payer: &Keypair,
) -> Result<TransactionMetadata, FailedTransactionMetadata> {
    let program_id = prc_sol::id();  // ✅ prc_sol not day_4

    let instruction = Instruction::new_with_bytes(
        program_id,
        &prc_sol::instruction::Func {}.data(),  // ✅ prc_sol
        prc_sol::accounts::LimitRange {}.to_account_metas(None),  // ✅ prc_sol
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(
        &[instruction],
        Some(&payer.pubkey()),
        &blockhash,
    );
    let tx = VersionedTransaction::try_new(
        VersionedMessage::Legacy(msg),
        &[payer],
    ).unwrap();

    svm.send_transaction(tx)  // ✅ no Ok() wrapper needed
}

fn setup() -> (LiteSVM, Keypair) {
    let program_id = prc_sol::id();  // ✅ prc_sol
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/prc_sol.so");  // ✅ prc_sol.so
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    (svm, payer)
}

#[test]
fn test_a_is_too_small() {
    let (mut svm, payer) = setup();

    let res = send_limit_range(&mut svm, &payer, 9);

    match res {
        Ok(_) => panic!("❌ Expected error but transaction succeeded!"),
        Err(e) => {
            let err_str = format!("{:?}", e);  // ✅ fixed
            println!("Error caught: {}", err_str);
            assert!(
                err_str.contains("AisTooSmall"),  // ✅ use enum variant name
                "Expected 'AisTooSmall' but got: {}",
                err_str
            );
            println!("✅ Correctly caught: a is too small");
        }
    }
}

#[test]
fn test_a_is_too_big() {
    let (mut svm, payer) = setup();

    let res = send_limit_range(&mut svm, &payer, 101);

    match res {
        Ok(_) => panic!("❌ Expected error but transaction succeeded!"),
        Err(e) => {
            let err_str = format!("{:?}", e);  // ✅ fixed
            println!("Error caught: {}", err_str);
            assert!(
                err_str.contains("AisTooBig"),  // ✅ use enum variant name
                "Expected 'AisTooBig' but got: {}",
                err_str
            );
            println!("✅ Correctly caught: a is too big");
        }
    }
}

// ─── Test 3: value 50 → valid, should succeed ────────────────────────────
#[test]
fn test_valid_range() {
    let (mut svm, payer) = setup();

    let res = send_limit_range(&mut svm, &payer, 50);

    match res {
        Ok(tx_result) => {
            println!("\n--- TRANSACTION LOGS ---");
            for log in tx_result.logs {
                println!("{}", log);
            }
            println!("✅ Valid value 50 passed correctly");
        }
        Err(e) => panic!("❌ Expected success but got error: {:?}", e),  
    }
}

#[test]
fn test_always_errors() {
    let (mut svm, payer) = setup();

    let res = send_func(&mut svm, &payer);

    match res {
        Ok(_) => panic!("❌ Expected error but transaction succeeded!"),
        Err(e) => {
            let err_str = format!("{:?}", e); 
            println!("Error caught: {}", err_str);
            assert!(
                err_str.contains("AlwaysErrors"),  
                "Expected 'AlwaysErrors' but got: {}",
                err_str
            );
            println!("✅ Correctly caught: Always errors");
        }
    }
}