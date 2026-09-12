use aeon_engine::daemon::AmaDaemon;
use aeon_engine::gawd::ama::AmaMasterAgent;
use aeon_engine::gemi::server::GemiServer;
use aeon_engine::gmcp::server::GmcpServer;
use aeon_engine::gmcp::tools::ToolRegistry;
use aeon_engine::sandbox::manager::SandboxManager;
use aeon_engine::AEON_VERSION;

use std::env;
use std::io::{self, Read, IsTerminal};
use std::path::{Path, PathBuf};

fn get_home_dir() -> PathBuf {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn print_help() {
    println!("aeon v{}", AEON_VERSION);
    println!("Usage: aeon [COMMAND | INTENT]\n");
    println!("Commands & Intents:");
    println!("  version, -v, --version   Print version");
    println!("  help, -h, --help         Show help");
    println!("  install                  Initialize sandboxed .aeon environment");
    println!("  uninstall                Clean up sandboxed .aeon environment");
    println!("  mcp                      Start native MCP server");
    println!("  gemi                     Start GEMI REST server");
    println!("  status                   Inspect workspace health report");
    println!("  models                   List available models");
    println!("  agents                   List active agents");
    println!("  engines                  List active engines");
    println!("  benchmark                Run performance benchmark");
    println!("  build                    Build validation & autonomous healing");
    println!("  test                     Run test harness");
    println!("  clean                    Clean workspace build artifacts");
    println!("\nPowered by GAWD Agent System & ToolRegistry.");
    println!("Examples:");
    println!("  aeon \"analyze current git status\"");
    println!("  cat error.log | aeon \"debug this error\"");
    println!("  aeon scout_model <model_id> > model.json");
}

fn run_install(global_dir: &Path) {
    println!("Initializing aeon runtime...");
    let _ = SandboxManager::ensure_global_sandbox(global_dir);
    let cfg = aeon_engine::sandbox::manager::AeonConfig::load(global_dir).unwrap_or_else(|e| {
        eprintln!("FATAL: {}", e);
        std::process::exit(1);
    });

    // Zero-Config Autonomous Model Provisioning (Rule 31)
    if aeon_engine::gemi::models::ModelManager::get_selected_model().is_none() {
        println!("No local reasoning substrate detected. Provisioning alpha weights...");
        let res = aeon_engine::gemi::models::ModelManager::install_model(&cfg.alpha_weights_url);
        println!("Provisioning status: {}", res);
    }

    AmaDaemon::ensure_daemon_running(global_dir, global_dir);
    println!("aeon runtime initialized.");
}

fn main() {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let home = get_home_dir();
    let global_dir = home.join(".aeon");

    if env::args().nth(1).as_deref() != Some("daemon-start") {
        AmaDaemon::ensure_daemon_running(&cwd, &global_dir);
    }

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        if !io::stdin().is_terminal() {
            let mut buffer = String::new();
            if io::stdin().read_to_string(&mut buffer).is_ok() {
                let trimmed = buffer.trim();
                if !trimmed.is_empty() {
                    let ama = AmaMasterAgent::new();
                    let answer = ama.solve_clean(trimmed, &cwd, AEON_VERSION);
                    print!("{}", answer);
                    return;
                }
            }
        }
        return;
    }

    let first_arg = args[0].to_lowercase();
    let clean_first_arg = first_arg.trim_start_matches(':').trim_start_matches('/');

    match clean_first_arg {
        "help" | "-h" | "--help" => {
            print_help();
        }
        "version" | "-v" | "--version" => {
            println!("aeon v{}", AEON_VERSION);
        }
        "install" => {
            run_install(&global_dir);
        }
        "uninstall" => {
            let _ = std::fs::remove_dir_all(&global_dir);
            println!("aeon runtime removed.");
        }
        "daemon-start" => {
            AmaDaemon::run_daemon_loop(global_dir.clone(), global_dir);
        }
        "gmcp-server" | "mcp-server" | "mcp" => {
            GmcpServer::run_stdio(&cwd, AEON_VERSION);
        }
        "gemi-server" | "gemi" => {
            let cfg = aeon_engine::sandbox::manager::AeonConfig::load(&global_dir).unwrap_or_else(|e| {
                eprintln!("FATAL: {}", e);
                std::process::exit(1);
            });
            GemiServer::start_http_server(cwd.clone(), cfg.gemi_port);
        }
        "models" => {
            let res = aeon_engine::gmcp::GmcpHost::dispatch("list_models", &serde_json::json!(null).to_string(), &cwd);
            println!("{}", res);
        }
        "pulse" => {
            let intent = args.get(1..).map(|s| s.join(" ")).unwrap_or_default();
            if intent.is_empty() {
                println!("Usage: aeon pulse <natural language instruction>");
            } else {
                match aeon_engine::daemon::admin::AeonAdmin::ingest_natural_intent(&cwd, &intent) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Pulse ingestion failed: {}", e),
                }
            }
        }
        "audit" => {
            match aeon_engine::daemon::admin::AeonAdmin::audit_compliance(&cwd, None) {
                Ok(report) => println!("{}", report),
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        "admin" => {
            let sub_cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
            match sub_cmd {
                "sync" => {
                    match aeon_engine::daemon::admin::AeonAdmin::enforce_version_consistency(&cwd) {
                        Ok(v) => println!("Version synchronization complete: v{}", v),
                        Err(e) => eprintln!("Sync failed: {}", e),
                    }
                }
                "pulse" => {
                    let intent = args.get(2..).map(|s| s.join(" ")).unwrap_or_default();
                    if intent.is_empty() {
                        println!("Usage: aeon admin pulse <natural language instruction>");
                    } else {
                        match aeon_engine::daemon::admin::AeonAdmin::ingest_natural_intent(&cwd, &intent) {
                            Ok(msg) => println!("{}", msg),
                            Err(e) => eprintln!("Pulse ingestion failed: {}", e),
                        }
                    }
                }
                "audit" => {
                    match aeon_engine::daemon::admin::AeonAdmin::audit_compliance(&cwd, None) {
                        Ok(report) => println!("{}", report),
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
                "verify" => {
                    match aeon_engine::daemon::admin::AeonAdmin::verify_version_alignment(&cwd) {
                        Ok(_) => println!("Version alignment verified."),
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
                "release" => {
                    match aeon_engine::daemon::admin::AeonAdmin::execute_release(&cwd) {
                        Ok(msg) => println!("{}", msg),
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
                _ => println!("Admin commands: sync, audit, verify, release"),
            }
        }
        "clean" => {
            let _ = std::fs::remove_dir_all(cwd.join("target"));
            println!("Workspace build artifacts cleaned.");
        }
        _ => {
            let cmd_name = clean_first_arg;
            let cmd_arg = args.get(1..).map(|s| s.join(" ")).unwrap_or_default();
            let mut goal = args.join(" ");

            if !io::stdin().is_terminal() {
                let mut buffer = String::new();
                if io::stdin().read_to_string(&mut buffer).is_ok() {
                    let trimmed = buffer.trim();
                    if !trimmed.is_empty() {
                         goal = format!("{}\n\n[INPUT DATA]:\n{}", goal, trimmed);
                    }
                }
            }

            let ama = AmaMasterAgent::new();

            // Unified Meta-Substrate Dispatch (Host -> ToolRegistry)
            if ToolRegistry::exists(cmd_name) {
                let res = aeon_engine::gmcp::GmcpHost::dispatch(cmd_name, &cmd_arg, &cwd);
                if !io::stdout().is_terminal() {
                    print!("{}", res);
                } else {
                    println!("{}", res);
                }
                return;
            }

            // Axiomatic Pulse Ingestion: Automatically anchor any natural language instruction into pulse.md
            match aeon_engine::daemon::admin::AeonAdmin::ingest_natural_intent(&cwd, &goal) {
                Ok(msg) => println!("{}", msg),
                Err(_) => {
                    // Fallback to direct solving if ingestion fails
                    let answer = ama.solve_clean(&goal, &cwd, AEON_VERSION);
                    if !io::stdout().is_terminal() {
                        print!("{}", answer);
                    } else {
                        println!("{}", answer);
                    }
                }
            }
        }
    }
}
