// AEON Substrate Integration Tests
// 100% Rust-Native Validation of GAWD, GEMI & GMCP Pillars

use std::fs;

#[test]
fn test_substrate_bootstrap_and_config() {
    let test_dir = std::env::temp_dir().join("aeon_integration_test");
    let _ = fs::remove_dir_all(&test_dir);
    let _ = fs::create_dir_all(&test_dir);

    // Verify Sandbox Initialization
    let res = aeon_engine::sandbox::SandboxManager::ensure_global_sandbox(&test_dir);
    assert!(res.is_ok());

    let config_path = test_dir.join("config.json");
    assert!(config_path.exists());

    // Verify Config Load
    let cfg = aeon_engine::sandbox::manager::AeonConfig::load(&test_dir).expect("Config load failed");
    assert_eq!(cfg.gmcp_port, 9090);

    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn test_tool_registry_and_execution() {
    let ws = std::env::current_dir().unwrap();

    // Test Status Tool
    let res = aeon_engine::gmcp::tools::ToolRegistry::execute_tool("status", &serde_json::json!(null), &ws);
    assert!(res.contains("AEON Engine Version"));

    // Test Read/Write Tool
    let test_file = "integration_test.txt";
    let test_content = "AEON_INTEGRATION_TEST_SUCCESS";

    let write_arg = serde_json::json!({
        "path": test_file,
        "content": test_content
    });

    let write_res = aeon_engine::gmcp::tools::ToolRegistry::execute_tool("write_file", &write_arg, &ws);
    assert!(write_res.contains("Wrote to"));

    let read_arg = serde_json::json!(test_file);
    let read_res = aeon_engine::gmcp::tools::ToolRegistry::execute_tool("read_file", &read_arg, &ws);
    assert_eq!(read_res, test_content);

    let _ = fs::remove_file(ws.join(test_file));
}

#[test]
fn test_backup_logic() {
    let test_ws = std::env::temp_dir().join("aeon_ws_backup");
    let _ = fs::remove_dir_all(&test_ws);
    let _ = fs::create_dir_all(&test_ws);

    fs::write(test_ws.join("data.txt"), "some data").unwrap();

    let res = aeon_engine::sandbox::manager::AeonBackupManager::backup_work(&test_ws);
    assert!(res.is_ok());

    let backups_dir = test_ws.join(".aeon/backups");
    assert!(backups_dir.exists());

    let entries = fs::read_dir(backups_dir).unwrap();
    assert!(entries.count() > 0);

    let _ = fs::remove_dir_all(&test_ws);
}
