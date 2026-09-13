#![allow(unexpected_cfgs)]
use aeon_engine::daemon::AmaDaemon;
use aeon_engine::gawd::ama::AmaMasterAgent;
use aeon_engine::gemi::server::GemiServer;
use aeon_engine::gmcp::server::GmcpServer;

use aeon_engine::AEON_VERSION;

use std::env;
use std::io::{self, Read, Write, IsTerminal};
use std::path::PathBuf;
use log::{info, warn, error};

const MAX_STDIN_SIZE: usize = 100 * 1024 * 1024;  // Fluid Scaling: 100MB baseline limit
const STDIN_TIMEOUT_SECS: u64 = 120; // Increased to 2 minutes

fn read_stdin_bounded() -> io::Result<Option<String>> {
    let stdin = io::stdin();

    // Set read timeout on Unix
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let fd = stdin.as_raw_fd();
        let timeout = libc::timeval {
            tv_sec: STDIN_TIMEOUT_SECS as _,
            tv_usec: 0,
        };
        unsafe {
            libc::setsockopt(fd, libc::SOL_SOCKET, libc::SO_RCVTIMEO,
                &timeout as *const _ as *const libc::c_void,
                std::mem::size_of::<libc::timeval>() as u32);
        }
    }

    let mut buffer = Vec::new();
    let mut limited = stdin.take(MAX_STDIN_SIZE as u64);
    limited.read_to_end(&mut buffer)?;

    if buffer.len() >= MAX_STDIN_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Input exceeds {} bytes limit", MAX_STDIN_SIZE)
        ));
    }

    let content = String::from_utf8(buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let trimmed = content.trim();
    Ok(if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    })
}

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
    println!("  select-model             Select or override active model");
    println!("  deep-scan                Parallel deep scan of user home for local models");
    println!("  mcp-scout                Autonomous web-scouting of open-source MCP servers");
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



fn main() {
    #[cfg(tokio_unstable)]
    console_subscriber::init();

    env_logger::init();
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let home = get_home_dir();
    let global_dir = home.join(".aeon");

    if env::args().nth(1).as_deref() != Some("daemon-start") {
        AmaDaemon::ensure_daemon_running(&cwd, &global_dir);
    }

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        if !io::stdin().is_terminal() {
            match read_stdin_bounded() {
                Ok(Some(input)) => {
                    let ama = AmaMasterAgent::new();
                    let _ = ama.solve_stream(&input, &cwd, AEON_VERSION);
                    std::io::stdout().flush().ok();
                    std::process::exit(0);
                }
                Ok(None) => return,
                Err(e) => {
                    error!("stdin error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        return;
    }

    let first_arg = args[0].to_lowercase();
    let clean_first_arg = first_arg.trim_start_matches(':').trim_start_matches('/');

    match clean_first_arg {
        "--bg-solve" | "bg-solve" => {
            let intent = args.get(1..).map(|s| s.join(" ")).unwrap_or_default();
            if !intent.is_empty() {
                let ama = AmaMasterAgent::new();
                let _ = ama.solve_clean(&intent, &cwd, AEON_VERSION);
            }
        }
        "--bg-train" | "bg-train" => {
            let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
            let global_dir = home.join(".aeon");
            let _ = aeon_engine::gemi::reasoning::AeonReasoningModel::train_from_experience(&global_dir);
        }
        "help" | "-h" | "--help" => {
            print_help();
        }
        "version" | "-v" | "--version" => {
            println!("aeon v{}", AEON_VERSION);
        }
        "install" => {
            let ama = AmaMasterAgent::new();
            let answer = ama.solve_clean("admin mission: initialize sandboxed .aeon environment and provision weights", &cwd, AEON_VERSION);
            println!("{}", answer);
        }
        "uninstall" => {
            let ama = AmaMasterAgent::new();
            let answer = ama.solve_clean("admin mission: remove and clean up sandboxed .aeon environment", &cwd, AEON_VERSION);
            println!("{}", answer);
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
            let server = tiny_http::Server::http(format!("0.0.0.0:{}", cfg.gemi_port)).expect("Failed to bind GEMI port");
            GemiServer::start_http_server(cwd.clone(), server);
        }
        "status" => {
            let ama = AmaMasterAgent::new();
            let answer = ama.solve_clean("status", &cwd, AEON_VERSION);
            println!("{}", answer);
        }
        "models" => {
            let ama = AmaMasterAgent::new();
            let answer = ama.solve_clean("models", &cwd, AEON_VERSION);
            println!("{}", answer);
        }
        "select-model" | "select_model" => {
            let model = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if model.is_empty() {
                println!("Usage: aeon select-model <model_name_or_id>");
            } else {
                let ama = AmaMasterAgent::new();
                let intent = format!("admin mission: select and override active model substrate to {}", model);
                let answer = ama.solve_clean(&intent, &cwd, AEON_VERSION);
                println!("{}", answer);
            }
        }
        "deep-scan" | "deep_scan" => {
            let ama = AmaMasterAgent::new();
            let answer = ama.solve_clean("admin mission: perform parallel deep-scan of user home for local models and register them", &cwd, AEON_VERSION);
            println!("{}", answer);
        }
        "mcp-scout" | "mcp_scout" => {
            let ama = AmaMasterAgent::new();
            let answer = ama.solve_clean("admin mission: perform autonomous web-scouting of open-source MCP servers and benchmark them", &cwd, AEON_VERSION);
            println!("{}", answer);
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
            let ama = AmaMasterAgent::new();
            let answer = ama.solve_clean("admin mission: perform compliance audit and technical verification", &cwd, AEON_VERSION);
            println!("{}", answer);
        }
        "admin" => {
            let sub_cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
            let ama = AmaMasterAgent::new();
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
                    let answer = ama.solve_clean("admin mission: perform compliance audit and technical verification", &cwd, AEON_VERSION);
                    println!("{}", answer);
                }
                "verify" => {
                    let answer = ama.solve_clean("admin mission: verify version alignment across manifest and documents", &cwd, AEON_VERSION);
                    println!("{}", answer);
                }
                "release" => {
                    let answer = ama.solve_clean("admin mission: execute full release orchestration sequence", &cwd, AEON_VERSION);
                    println!("{}", answer);
                }
                "lint" => {
                    let answer = ama.solve_clean("admin mission: run linting and static analysis (clippy)", &cwd, AEON_VERSION);
                    println!("{}", answer);
                }
                "audit-deps" => {
                    let answer = ama.solve_clean("admin mission: run dependency security audit", &cwd, AEON_VERSION);
                    println!("{}", answer);
                }
                _ => println!("Admin commands: sync, audit, verify, release, lint, audit-deps"),
            }
        }
        "clean" => {
            let _ = std::fs::remove_dir_all(cwd.join("target"));
            println!("Workspace build artifacts cleaned.");
        }
        _ => {
            let _cmd_name = clean_first_arg;
            let _cmd_arg = args.get(1..).map(|s| s.join(" ")).unwrap_or_default();
            let mut goal = args.join(" ");

            #[cfg(unix)]
            if !io::stdin().is_terminal() {
                use std::os::unix::io::AsRawFd;
                let fd = io::stdin().as_raw_fd();
                let mut poll_fd = libc::pollfd {
                    fd,
                    events: libc::POLLIN,
                    revents: 0,
                };
                let ret = unsafe { libc::poll(&mut poll_fd, 1, 0) };
                if ret > 0 && (poll_fd.revents & libc::POLLIN) != 0 {
                    let mut buffer = String::new();
                    if io::stdin().read_to_string(&mut buffer).is_ok() {
                        let trimmed = buffer.trim();
                        if !trimmed.is_empty() {
                            goal = format!("{}\n\n[INPUT DATA]:\n{}", goal, trimmed);
                        }
                    }
                }
            }

            let ama = AmaMasterAgent::new();

            // Axiomatic Pulse Ingestion: Automatically anchor any natural language instruction into pulse.md
            match aeon_engine::daemon::admin::AeonAdmin::ingest_natural_intent(&cwd, &goal) {
                Ok(msg) => {
                    info!("Natural intent ingested successfully: {}", msg);
                    let _ = ama.solve_stream(&goal, &cwd, AEON_VERSION);
                    std::io::stdout().flush().ok();
                    std::process::exit(0);
                }
                Err(e) => {
                    warn!("Natural intent ingestion failed: {}. Falling back to direct swarm solving.", e);
                    let _ = ama.solve_stream(&goal, &cwd, AEON_VERSION);
                    std::io::stdout().flush().ok();
                    std::process::exit(0);
                }
            }
        }
    }
}
