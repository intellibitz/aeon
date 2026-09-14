#![allow(unexpected_cfgs)]
use aeon_engine::daemon::AmaDaemon;
use aeon_engine::gawd::ama::AmaMasterAgent;
use aeon_engine::gemi::server::GemiServer;
use aeon_engine::gmcp::server::GmcpServer;
use aeon_engine::AEON_VERSION;

use clap::{Parser, Subcommand};
use std::env;
use std::io::{self, Read, Write, IsTerminal};
use std::path::PathBuf;
use tracing::{info, warn, error};

const MAX_STDIN_SIZE: usize = 100 * 1024 * 1024;  // Fluid Scaling: 100MB baseline limit
const STDIN_TIMEOUT_SECS: u64 = 120; // Increased to 2 minutes

#[derive(Parser)]
#[command(name = "aeon")]
#[command(version = AEON_VERSION)]
#[command(about = "EAI: Exponential Intelligence for Any AI - GAWD, GEMI & GMCP Multi-Agent Engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Natural language intent or pulse
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    intent: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start persistent AEON Pulse Shell
    Shell,
    /// Initialize sandboxed .aeon environment
    Install,
    /// Clean up sandboxed .aeon environment
    Uninstall,
    /// Start native MCP server
    Mcp,
    /// Start GEMI REST server
    Gemi,
    /// Inspect workspace health report
    Status,
    /// List available models
    Models,
    /// Select or override active model
    SelectModel { model: String },
    /// Parallel deep scan of user home for local models
    DeepScan,
    /// Autonomous web-scouting of open-source MCP servers
    McpScout,
    /// Ingest a natural language intent into pulse.md
    Pulse {
        #[arg(trailing_var_arg = true)]
        intent: Vec<String>
    },
    /// Perform compliance audit and technical verification
    Audit,
    /// Administrative commands
    Admin {
        #[command(subcommand)]
        subcommand: AdminCommands,
    },
    /// Clean workspace build artifacts
    Clean,
    /// Internal daemon start (Called by ensure_daemon_running)
    DaemonStart { workspace: String },
}

#[derive(Subcommand)]
enum AdminCommands {
    /// Synchronize version consistency
    Sync,
    /// Ingest pulse via admin
    Pulse {
        #[arg(trailing_var_arg = true)]
        intent: Vec<String>
    },
    /// Compliance audit
    Audit,
    /// Verify version alignment
    Verify,
    /// Full release orchestration
    Release,
    /// Run clippy
    Lint,
    /// Run cargo audit
    AuditDeps,
}

fn read_stdin_bounded() -> io::Result<Option<String>> {
    let stdin = io::stdin();
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let fd = stdin.as_raw_fd();
        let timeout = libc::timeval { tv_sec: STDIN_TIMEOUT_SECS as _, tv_usec: 0 };
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
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Input exceeds {} bytes limit", MAX_STDIN_SIZE)));
    }
    let content = String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let trimmed = content.trim();
    Ok(if trimmed.is_empty() { None } else { Some(trimmed.to_string()) })
}

fn get_home_dir() -> PathBuf {
    env::var_os("HOME").or_else(|| env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."))
}

fn run_shell(workspace: &std::path::Path) {
    use aeon_engine::gawd::queue::SubstratePulseQueue;
    let queue = SubstratePulseQueue::global();
    let ama = AmaMasterAgent::new();

    println!("AEON Pulse Shell v{} (Glass Box Telemetry Mode Active)", AEON_VERSION);
    println!("Enter pulses to interact with the substrate. Pulses are queued and processed in order.");
    println!("Type 'exit' to quit.");

    let w = workspace.to_path_buf();
    std::thread::spawn(move || {
        loop {
            if let Some(pulse) = queue.pop() {
                let _ = ama.solve_stream(&pulse.intent, &w, AEON_VERSION);
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    loop {
        print!("aeon> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            if trimmed.is_empty() { continue; }
            if trimmed == "exit" || trimmed == "quit" { break; }
            let _ = queue.ingest(trimmed, &workspace.to_path_buf(), AEON_VERSION);
        } else {
            break;
        }
    }
}

fn main() {
    #[cfg(tokio_unstable)]
    console_subscriber::init();

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stdout)
        .init();
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let home = get_home_dir();
    let global_dir = home.join(".aeon");

    let cli = Cli::parse();

    if !matches!(cli.command, Some(Commands::DaemonStart { .. })) {
        AmaDaemon::ensure_daemon_running(&cwd, &global_dir);
    }

    if let Some(command) = cli.command {
        let ama = AmaMasterAgent::new();
        match command {
            Commands::Shell => run_shell(&cwd),
            Commands::Install => {
                let answer = ama.solve_clean("admin mission: initialize sandboxed .aeon environment and provision weights", &cwd, AEON_VERSION);
                println!("{}", answer);
            }
            Commands::Uninstall => {
                let answer = ama.solve_clean("admin mission: remove and clean up sandboxed .aeon environment", &cwd, AEON_VERSION);
                println!("{}", answer);
            }
            Commands::Mcp => GmcpServer::run_stdio(&cwd, AEON_VERSION),
            Commands::Gemi => {
                let cfg = aeon_engine::sandbox::manager::AeonConfig::load(&global_dir).expect("Fatal: Malformed configuration");
                let server = tiny_http::Server::http(format!("0.0.0.0:{}", cfg.gemi_port)).expect("Failed to bind GEMI port");
                GemiServer::start_http_server(cwd.clone(), server);
            }
            Commands::Status => {
                let answer = ama.solve_clean("status", &cwd, AEON_VERSION);
                println!("{}", answer);
            }
            Commands::Models => {
                let answer = ama.solve_clean("models", &cwd, AEON_VERSION);
                println!("{}", answer);
            }
            Commands::SelectModel { model } => {
                let intent = format!("admin mission: select and override active model substrate to {}", model);
                let answer = ama.solve_clean(&intent, &cwd, AEON_VERSION);
                println!("{}", answer);
            }
            Commands::DeepScan => {
                let answer = ama.solve_clean("admin mission: perform parallel deep-scan of user home for local models and register them", &cwd, AEON_VERSION);
                println!("{}", answer);
            }
            Commands::McpScout => {
                let answer = ama.solve_clean("admin mission: perform autonomous web-scouting of open-source MCP servers and benchmark them", &cwd, AEON_VERSION);
                println!("{}", answer);
            }
            Commands::Pulse { intent } => {
                let intent_str = intent.join(" ");
                match aeon_engine::daemon::admin::AeonAdmin::ingest_natural_intent(&cwd, &intent_str) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Pulse ingestion failed: {}", e),
                }
            }
            Commands::Audit => {
                let answer = ama.solve_clean("admin mission: perform compliance audit and technical verification", &cwd, AEON_VERSION);
                println!("{}", answer);
            }
            Commands::Admin { subcommand } => {
                match subcommand {
                    AdminCommands::Sync => {
                        match aeon_engine::daemon::admin::AeonAdmin::enforce_version_consistency(&cwd) {
                            Ok(v) => println!("Version synchronization complete: v{}", v),
                            Err(e) => eprintln!("Sync failed: {}", e),
                        }
                    }
                    AdminCommands::Pulse { intent } => {
                        let intent_str = intent.join(" ");
                        match aeon_engine::daemon::admin::AeonAdmin::ingest_natural_intent(&cwd, &intent_str) {
                            Ok(msg) => println!("{}", msg),
                            Err(e) => eprintln!("Pulse ingestion failed: {}", e),
                        }
                    }
                    AdminCommands::Audit => {
                        let answer = ama.solve_clean("admin mission: perform compliance audit and technical verification", &cwd, AEON_VERSION);
                        println!("{}", answer);
                    }
                    AdminCommands::Verify => {
                        let answer = ama.solve_clean("admin mission: verify version alignment across manifest and documents", &cwd, AEON_VERSION);
                        println!("{}", answer);
                    }
                    AdminCommands::Release => {
                        let answer = ama.solve_clean("admin mission: execute full release orchestration sequence", &cwd, AEON_VERSION);
                        println!("{}", answer);
                    }
                    AdminCommands::Lint => {
                        let answer = ama.solve_clean("admin mission: run linting and static analysis (clippy)", &cwd, AEON_VERSION);
                        println!("{}", answer);
                    }
                    AdminCommands::AuditDeps => {
                        let answer = ama.solve_clean("admin mission: run dependency security audit", &cwd, AEON_VERSION);
                        println!("{}", answer);
                    }
                }
            }
            Commands::Clean => {
                let _ = std::fs::remove_dir_all(cwd.join("target"));
                println!("Workspace build artifacts cleaned.");
            }
            Commands::DaemonStart { .. } => {
                AmaDaemon::run_daemon_loop(global_dir.clone(), global_dir);
            }
        }
    } else if !cli.intent.is_empty() {
        let mut goal = cli.intent.join(" ");
        #[cfg(unix)]
        if !io::stdin().is_terminal() {
            use std::os::unix::io::AsRawFd;
            let fd = io::stdin().as_raw_fd();
            let mut poll_fd = libc::pollfd { fd, events: libc::POLLIN, revents: 0 };
            let ret = unsafe { libc::poll(&mut poll_fd, 1, 0) };
            if ret > 0 && (poll_fd.revents & libc::POLLIN) != 0 {
                let mut buffer = String::new();
                if io::stdin().read_to_string(&mut buffer).is_ok() {
                    let trimmed = buffer.trim();
                    if !trimmed.is_empty() { goal = format!("{}\n\n[INPUT DATA]:\n{}", goal, trimmed); }
                }
            }
        }

        let ama = AmaMasterAgent::new();
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
    } else if !io::stdin().is_terminal() {
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
    } else {
        // No command and no intent provided
        use clap::CommandFactory;
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
        println!();
    }
}
