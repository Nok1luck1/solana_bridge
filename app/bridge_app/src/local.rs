use std::process::{Child, Command};
pub fn run_solana_local_validator() {
    let _ = Command::new("solana-test-validator")
        .current_dir("../../bridge/")
        .arg("--geyser-plugin-config")
        .arg("geyser-config.json")
        .spawn()
        .expect("ls solana_local_validator failed to start");
}
pub fn run_evm_local_validator() {
    let _ = Command::new("anvil")
        .arg("--port")
        .arg("8550")
        .spawn()
        .expect("ls evm_local_validator failed to start");
}
