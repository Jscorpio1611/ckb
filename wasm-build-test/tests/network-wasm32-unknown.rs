use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn network_should_work() {
    // let resource = ckb_resource::Resource::bundled("specs/mainnet.toml");
    // let spec = ckb_chain_spec::ChainSpec::load_from(&resource).expect("load spec by name");
    // let consensus = spec.build_consensus().expect("build consensus");

    // let mut config = NetworkConfig::default();
    // config.max_outbound_peers = 8;
    // config.bootnodes = vec![Multiaddr::from_str(
    //     "/ip4/192.168.64.1/tcp/8115/ws/p2p/QmWzDLD9E5ideU2kcjFmzsmJVYiSiAezQNzrYZmLHEZVmX",
    // )
    // .unwrap()];
    // let network_state =
    //     Arc::new(NetworkState::from_config(config).expect("Init network state failed"));
    // let exit_handler = DefaultExitHandler::default();
    // let required_protocol_ids = vec![SupportProtocols::Sync.protocol_id()];

    // let handle = WasmHandle;

    // NetworkService::new(
    //     Arc::clone(&network_state),
    //     vec![],
    //     required_protocol_ids,
    //     consensus.identify_name(),
    //     "ckb-network-wasm32-unknown-test".to_string(),
    //     exit_handler,
    // )
    // .start(&handle)
    // .expect("Start network service failed")
}
