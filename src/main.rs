// aeon: Multi-Agent Engine

mod error;
mod daemon;
mod gawd;
mod gemi;
mod gmcp;
mod native;
mod sandbox;

use std::env;
use std::io::{self, Read, IsTerminal};
use std::path::{Path, PathBuf};

use crate::gmcp::tools::ToolRegistry;
use daemon::AmaDaemon;
use gawd::AmaMasterAgent;
use gemi::GemiServer;
use gmcp::server::GmcpServer;
use sandbox::SandboxManager;

pub const AEON_VERSION: &str = "0.1.2022690";

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
    println!("\nPowered by GAWD Agent Fleet & ToolRegistry for any natural language intent.");
    println!("Examples:");
    println!("  aeon \"analyze current git status\"");
    println!("  cat error.log | aeon \"debug this error\"");
    println!("  aeon scout_model <model_id> > model.json");
}

fn run_install(global_dir: &Path) {
    println!("Initializing aeon runtime...");
    let _ = SandboxManager::ensure_global_sandbox(global_dir);
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
            let cfg = crate::sandbox::manager::AeonConfig::load(&global_dir);
            GemiServer::start_http_server(cwd.clone(), cfg.gemi_port);
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

            // 🔋 Unified Meta-Substrate Dispatch (Host -> ToolRegistry)
            if ToolRegistry::exists(cmd_name) {
                let res = crate::gmcp::GmcpHost::dispatch(cmd_name, &cmd_arg, &cwd);
                if !io::stdout().is_terminal() {
                    print!("{}", res);
                } else {
                    println!("{}", res);
                }
                return;
            }

            let answer = ama.solve_clean(&goal, &cwd, AEON_VERSION);
            if !io::stdout().is_terminal() {
                print!("{}", answer);
            } else {
                println!("{}", answer);
            }
        }
    }
}
