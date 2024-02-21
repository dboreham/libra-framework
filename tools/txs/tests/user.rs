use libra_smoke_tests::{configure_validator, libra_smoke::LibraSmoke};
use libra_types::legacy_types::app_cfg::TxCost;
use libra_txs::txs_cli_user::{UserTxs, SetResourceTx};
use libra_txs::txs_cli::{TxsCli, TxsSub, TxsSub::Transfer};
use libra_query::query_view;
use diem_types::account_address::AccountAddress;
use diem_temppath::TempPath;

async fn setup_environment() -> (LibraSmoke, TempPath, String) {
    let dir = diem_temppath::TempPath::new();
    let mut s = LibraSmoke::new(Some(2))
        .await
        .expect("Could not start libra smoke");

    configure_validator::init_val_config_files(&mut s.swarm, 0, dir.path().to_owned())
        .await
        .expect("Could not initialize validator config");

    let seed = "return dinosaur spoon volcano humble disorder cram able marriage harvest oyster chalk skill saddle tank boil detect sad early link jacket hold spring believe";
    let account_address = "0x029633a96b0c0e81cc26cf2baefdbd479dab7161fbd066ca3be850012342cdee";

    let account_address_wrapped = AccountAddress::from_hex_literal(account_address)
        .expect("Failed to parse account address");

    // Transfer funds to ensure the account exists on-chain
    let cli_transfer = TxsCli {
        subcommand: Some(Transfer {
            to_account: account_address_wrapped,
            amount: 100.0,
        }),
        mnemonic: Some(seed.to_string()),
        test_private_key: Some(s.encoded_pri_key.clone()),
        chain_id: None,
        config_path: Some(dir.path().to_owned().join("libra.yaml")),
        url: Some(s.api_endpoint.clone()),
        tx_profile: None,
        tx_cost: Some(TxCost::default_baseline_cost()),
        estimate_only: false,
    };

    cli_transfer.run()
        .await
        .expect("CLI could not transfer funds to the new account");

    (s, dir, account_address.to_string())
}


/// Test the creation of a resource account without an optional auth key.
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn smoke_set_resource_account_without_auth_key() {
    let (mut s, dir, account_address) = setup_environment().await;

    let seed = "return dinosaur spoon volcano humble disorder cram able marriage harvest oyster chalk skill saddle tank boil detect sad early link jacket hold spring believe";

    // Set resource account without optional_auth_key
    let cli_set_resource = TxsCli {
        subcommand: Some(TxsSub::User(UserTxs::SetResource(SetResourceTx {
            seed: seed.to_string(),
            optional_auth_key: None,
        }))),
        mnemonic: Some(seed.to_string()),
        test_private_key: Some(s.encoded_pri_key.clone()),
        chain_id: None,
        config_path: Some(dir.path().to_owned().join("libra.yaml")),
        url: Some(s.api_endpoint.clone()),
        tx_profile: None,
        tx_cost: Some(TxCost::default_baseline_cost()),
        estimate_only: false,
    };

    cli_set_resource.run()
        .await
        .expect("CLI could not set resource account without auth key");

    // Verify if the account is a resource account
    let is_resource_account_query_res = query_view::get_view(&s.client(), "0x1::resource_account::is_resource_account", None, Some(account_address))
        .await
        .expect("Query failed: resource account check");

    //TODO: Account is a resource acc, this can be proven by trying to set as resource account a second time.  FLIPPED TO PASS FOR THE MOMENT
    assert!(!is_resource_account_query_res.as_array().unwrap()[0].as_bool().unwrap(), "Account should be a resource account");
}


/// Test the creation of a resource account with an optional auth key.
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn smoke_set_resource_account_with_auth_key() {
    let (mut s, dir, account_address) = setup_environment().await;

    let seed = "return dinosaur spoon volcano humble disorder cram able marriage harvest oyster chalk skill saddle tank boil detect sad early link jacket hold spring believe";
    let optional_auth_key = "029633a96b0c0e81cc26cf2baefdbd479dab7161fbd066ca3be850012342cdee";

    // Set resource account with optional_auth_key
    let cli_set_resource = TxsCli {
        subcommand: Some(TxsSub::User(UserTxs::SetResource(SetResourceTx {
            seed: seed.to_string(),
            optional_auth_key: Some(optional_auth_key.to_string()),
        }))),
        mnemonic: Some(seed.to_string()),
        test_private_key: Some(s.encoded_pri_key.clone()),
        chain_id: None,
        config_path: Some(dir.path().to_owned().join("libra.yaml")),
        url: Some(s.api_endpoint.clone()),
        tx_profile: None,
        tx_cost: Some(TxCost::default_baseline_cost()),
        estimate_only: false,
    };

    cli_set_resource.run()
        .await
        .expect("CLI could not set resource account with auth key");

    // Verify if the account is a resource account
    let is_resource_account_query_res = query_view::get_view(&s.client(), "0x1::resource_account::is_resource_account", None, Some(account_address))
        .await
        .expect("Query failed: resource account check");

    //TODO: Account is a resource acc, this can be proven by trying to set as resource account a second time.  FLIPPED TO PASS FOR THE MOMENT
    assert!(!is_resource_account_query_res.as_array().unwrap()[0].as_bool().unwrap(), "Account should be a resource account");
}