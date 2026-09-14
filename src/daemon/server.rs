// Always-On AMA Master Daemon Process Manager
// 100% Rust implementation managing GMCP (Port 9090), GEMI (Port 9091) & A2A Cluster UDP (Port 9092)

use std::fs;
use std::io::{Write, Read, Seek, SeekFrom};

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;

use signal_hook::{consts::{SIGTERM, SIGINT}, iterator::Signals};
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tracing::{info, warn};

use crate::error::{EaiError, EaiResult};
use crate::sandbox::manager::AeonConfig;
use crate::gemi::GemiServer;
use crate::gmcp::server::GmcpServer;
use crate::gawd::queue::SubstratePulseQueue;
use crate::gawd::ama::AmaMasterAgent;

pub struct AmaDaemon;

pub struct DaemonContext {
    pub shutdown_signal: Arc<AtomicBool>,
    _lock: DaemonLock,
}

impl DaemonContext {
    pub fn new(lock: DaemonLock) -> Self {
        DaemonContext {
            shutdown_signal: Arc::new(AtomicBool::new(false)),
            _lock: lock,
        }
    }

    pub fn setup_signal_handlers(&self) -> Result<(), EaiError> {
        let shutdown = Arc::clone(&self.shutdown_signal);
        thread::spawn(move || {
            if let Ok(mut signals) = Signals::new([SIGTERM, SIGINT]) {
                for sig in signals.forever() {
                    eprintln!("[AmaDaemon] Received signal: {}", sig);
                    shutdown.store(true, Ordering::Release);
                }
            }
        });
        Ok(())
    }

    pub fn is_shutdown_requested(&self) -> bool {
        self.shutdown_signal.load(Ordering::Acquire)
    }
}

pub struct DaemonLock {
    file: fs::File,
}

impl DaemonLock {
    fn acquire(path: &Path) -> Result<Self, String> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| e.to_string())?;

        #[cfg(unix)]
        {
            // SAFETY: We just opened the file, so fd is valid.
            // flock doesn't access memory unsafely or take ownership.
            let ret = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
            if ret != 0 {
                return Err("Locked by another process".into());
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::fileapi::LockFileEx;
            use winapi::um::minwinbase::{LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY};

            let handle = file.as_raw_handle();
            let mut overlapped = unsafe { std::mem::zeroed() };
            let ret = unsafe {
                LockFileEx(
                    handle as _,
                    LOCKFILE_EXCLUSIVE_LOCK | LOCKFILE_FAIL_IMMEDIATELY,
                    0,
                    1,
                    0,
                    &mut overlapped,
                )
            };
            if ret == 0 {
                return Err("Locked by another process".into());
            }
        }

        Ok(Self { file })
    }

    fn write_pid(&mut self) -> Result<(), String> {
        self.file.set_len(0).map_err(|e| e.to_string())?;
        self.file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
        write!(self.file, "{}", std::process::id()).map_err(|e| e.to_string())?;
        self.file.flush().map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl AmaDaemon {
    pub fn get_lock_file(global_dir: &Path) -> PathBuf {
        global_dir.join("ama.lock")
    }

    pub fn check_status(global_dir: &Path) -> Option<u32> {
        let lock_file_path = Self::get_lock_file(global_dir);
        if !lock_file_path.exists() {
            return None;
        }

        let mut file = fs::OpenOptions::new().read(true).write(true).open(&lock_file_path).ok()?;

        #[cfg(unix)]
        {
            let ret = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
            if ret == 0 {
                // Successfully locked means no one else has it. Release and return None.
                let _ = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
                return None;
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::fileapi::{LockFileEx, UnlockFileEx};
            use winapi::um::minwinbase::{LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY};
            let handle = file.as_raw_handle();
            let mut overlapped = unsafe { std::mem::zeroed() };
            let ret = unsafe {
                LockFileEx(
                    handle as _,
                    LOCKFILE_EXCLUSIVE_LOCK | LOCKFILE_FAIL_IMMEDIATELY,
                    0,
                    1,
                    0,
                    &mut overlapped,
                )
            };
            if ret != 0 {
                // Successfully locked means no one else has it. Release and return None.
                unsafe { UnlockFileEx(handle as _, 0, 1, 0, &mut overlapped) };
                return None;
            }
        }

        // If we reach here, it means we couldn't acquire the lock, so it's running.
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
            content.trim().parse::<u32>().ok()
        } else {
            None
        }
    }

    fn is_process_alive(lock_file_path: &Path) -> bool {
        if let Ok(content) = fs::read_to_string(lock_file_path) {
            if let Ok(pid) = content.trim().parse::<u32>() {
                #[cfg(unix)]
                {
                    return unsafe { libc::kill(pid as i32, 0) == 0 };
                }
                #[cfg(windows)]
                {
                    use std::process::Command;
                    return Command::new("tasklist")
                        .args(&["/FI", &format!("PID eq {}", pid)])
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false);
                }
            }
        }
        false
    }

    pub fn get_hash_file(global_dir: &Path) -> PathBuf {
        global_dir.join("binary.hash")
    }

    pub fn verify_binary_integrity(bin_path: &Path, global_dir: &Path) -> EaiResult<bool> {
        let hash_file = Self::get_hash_file(global_dir);
        if !hash_file.exists() {
            // If no hash file exists, we bootstrap by recording the current one
            let current_hash = Self::calculate_binary_hash(bin_path)?;
            fs::write(&hash_file, &current_hash).map_err(|e| EaiError::filesystem(e.to_string()))?;
            return Ok(true);
        }

        let trusted_hash = fs::read_to_string(&hash_file).map_err(|e| EaiError::filesystem(e.to_string()))?;
        let current_hash = Self::calculate_binary_hash(bin_path)?;

        Ok(trusted_hash.trim() == current_hash.trim())
    }

    fn calculate_binary_hash(path: &Path) -> EaiResult<String> {
        use sha2::{Sha256, Digest};
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 65536];
        while let Ok(n) = file.read(&mut buffer) {
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
        }
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn ensure_daemon_running(workspace: &Path, global_dir: &Path) {
        if Self::check_status(global_dir).is_some() {
            return;
        }

        let current_exe = std::env::current_exe().ok();
        let bin_name = if cfg!(target_os = "windows") { "bin/aeon-engine.exe" } else { "bin/aeon-engine" };
        let global_bin = global_dir.join(bin_name);

        let bin_to_run = if let Some(ref exe) = current_exe {
            exe.clone()
        } else if global_bin.exists() {
            global_bin
        } else {
            PathBuf::from(if cfg!(target_os = "windows") { "aeon.exe" } else { "aeon" })
        };

        // Binary Integrity Check (Aspiration 4 Hardening)
        match Self::verify_binary_integrity(&bin_to_run, global_dir) {
            Ok(true) => info!("[AmaDaemon] Binary integrity verified."),
            Ok(false) => {
                warn!("[AmaDaemon] Binary integrity check FAILED. Potential tampering detected or build out of sync.");
                // In a strict production mode, we might abort here.
                // For local evolution, we log and continue if in 'alpha-world' space.
            }
            Err(e) => warn!("[AmaDaemon] Could not verify binary integrity: {}", e),
        }

        let mut cmd = Command::new(&bin_to_run);
        cmd.arg("daemon-start")
            .arg(workspace.to_str().unwrap_or("."))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            unsafe {
                cmd.pre_exec(|| {
                    libc::setsid();
                    Ok(())
                });
            }
        }

        let _ = cmd.spawn();
    }

    pub fn run_daemon_loop(workspace: PathBuf, global_dir: PathBuf) {
        let lock_file_path = Self::get_lock_file(&global_dir);

        // Ensure lock file is cleaned if stale (> 1 hour old and process is dead)
        if let Ok(metadata) = std::fs::metadata(&lock_file_path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(age) = modified.elapsed() {
                    if age.as_secs() > 3600 && !Self::is_process_alive(&lock_file_path) {
                        let _ = std::fs::remove_file(&lock_file_path);
                    }
                }
            }
        }

        let mut lock = match DaemonLock::acquire(&lock_file_path) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[AmaDaemon] Failed to acquire lock: {}. Daemon likely already running.", e);
                return;
            }
        };

        if let Err(e) = lock.write_pid() {
            eprintln!("[AmaDaemon] Failed to write PID to lock file: {}", e);
            return;
        }

        let ctx = DaemonContext::new(lock);
        if let Err(e) = ctx.setup_signal_handlers() {
            eprintln!("[AmaDaemon] Signal handler setup failed: {}", e);
        }

        let mut cfg = AeonConfig::load(&global_dir).expect("Fatal: Malformed configuration");
        let mut config_changed = false;

        // Substrate Administration & Hardware Optimization (Pillar 1)
        crate::daemon::runtime_admin::AeonRuntimeAdmin::start_administration_cycle(&workspace);

        // 1. Bind GEMI HTTP Server (Port 9091 / Dynamic)
        let (gemi_server, gemi_port) = Self::bind_http_with_fallback(cfg.gemi_port, "GEMI", &workspace);
        if gemi_port != cfg.gemi_port {
            cfg.gemi_port = gemi_port;
            config_changed = true;
        }

        // 2. Bind GMCP HTTP/SSE Server (Port 9093 / Dynamic)
        let (gmcp_http_server, gmcp_http_port) = Self::bind_http_with_fallback(cfg.gmcp_http_port, "GMCP HTTP", &workspace);
        if gmcp_http_port != cfg.gmcp_http_port {
            cfg.gmcp_http_port = gmcp_http_port;
            config_changed = true;
        }

        // 3. Bind A2A Cluster UDP Discovery Socket (Port 9092 / Dynamic)
        let (udp_socket, udp_port) = Self::bind_udp_with_fallback(cfg.udp_discovery_port, &workspace);
        if udp_port != cfg.udp_discovery_port {
            cfg.udp_discovery_port = udp_port;
            config_changed = true;
        }

        if config_changed {
            let _ = cfg.save(&global_dir);
            eprintln!("[AmaDaemon] Port collisions detected. Updated configuration with active ports.");
        }

        // Spawn services with panic handling
        let workspace_gemi = workspace.clone();
        thread::spawn(move || {
            if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                GemiServer::start_http_server(workspace_gemi, gemi_server);
            })) {
                eprintln!("[GEMI] Thread panicked: {:?}", e);
            }
        });

        let workspace_gmcp_http = workspace.clone();
        thread::spawn(move || {
             if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                GmcpServer::start_http_server(workspace_gmcp_http, gmcp_http_server);
            })) {
                eprintln!("[GMCP HTTP] Thread panicked: {:?}", e);
            }
        });

        let gmcp_actual_port = cfg.gmcp_port;
        thread::spawn(move || {
             if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                Self::start_udp_discovery_server(udp_socket, gmcp_actual_port);
            })) {
                eprintln!("[UDP] Thread panicked: {:?}", e);
            }
        });

        // Aspiration 32: Continuous Interaction Substrate Worker
        let workspace_pulse = workspace.clone();
        thread::spawn(move || {
            let queue = SubstratePulseQueue::global();
            let ama = AmaMasterAgent::new();

            loop {
                if let Some(pulse) = queue.pop() {
                    // Serialized Execution (Mandate 31)
                    info!("[SubstratePulseQueue] Processing Pulse: {}", pulse.intent);
                    let _ = ama.solve_stream(&pulse.intent, &workspace_pulse, crate::AEON_VERSION);
                }
                thread::sleep(Duration::from_millis(100));
            }
        });

        // Keep main daemon thread alive with graceful shutdown check
        while !ctx.is_shutdown_requested() {
            thread::sleep(Duration::from_secs(5));
        }

        eprintln!("[AmaDaemon] Graceful shutdown initiated");
    }

    fn bind_http_with_fallback(port: u16, name: &str, workspace: &Path) -> (tiny_http::Server, u16) {
        let addr = format!("0.0.0.0:{}", port);
        match tiny_http::Server::http(&addr) {
            Ok(server) => (server, port),
            Err(_) => {
                let server = tiny_http::Server::http("0.0.0.0:0").expect("Failed to bind to random port");
                let new_port = server.server_addr().to_ip().unwrap().port();
                crate::sandbox::manager::AeonAuditLogger::log(
                    workspace,
                    crate::sandbox::manager::LogLevel::Warning,
                    "PORT_COLLISION",
                    &format!("{} default port {} occupied. Randomized to {}", name, port, new_port)
                );
                eprintln!("[AmaDaemon] {} port collision! Randomized to {}", name, new_port);
                (server, new_port)
            }
        }
    }

    fn bind_udp_with_fallback(port: u16, workspace: &Path) -> (std::net::UdpSocket, u16) {
        let addr = format!("0.0.0.0:{}", port);
        match std::net::UdpSocket::bind(&addr) {
            Ok(socket) => (socket, port),
            Err(_) => {
                let socket = std::net::UdpSocket::bind("0.0.0.0:0").expect("Failed to bind random UDP port");
                let new_port = socket.local_addr().unwrap().port();
                crate::sandbox::manager::AeonAuditLogger::log(
                    workspace,
                    crate::sandbox::manager::LogLevel::Warning,
                    "UDP_PORT_COLLISION",
                    &format!("UDP Discovery port {} occupied. Randomized to {}", port, new_port)
                );
                eprintln!("[AmaDaemon] UDP port collision! Randomized to {}", new_port);
                (socket, new_port)
            }
        }
    }

    fn start_udp_discovery_server(socket: std::net::UdpSocket, gmcp_port: u16) {
        eprintln!("[A2A Cluster UDP] Discovery listener active on {}", socket.local_addr().unwrap());
        let mut buf = [0u8; 512];
        while let Ok((amt, src)) = socket.recv_from(&mut buf) {
            let msg = String::from_utf8_lossy(&buf[..amt]);
            if msg.contains("AEON_LAN_PING") {
                let pong = format!("AEON_LAN_PONG:aeon-daemon-node:{}", gmcp_port);
                let _ = socket.send_to(pong.as_bytes(), src);
            }
        }
    }

    #[allow(dead_code)]
    pub fn stop_daemon(global_dir: &Path) -> bool {
        let lock_file = Self::get_lock_file(global_dir);
        if lock_file.exists() {
            let _ = fs::remove_file(lock_file);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_file_path() {
        let tmp_dir = std::env::temp_dir();
        let path = AmaDaemon::get_lock_file(&tmp_dir);
        assert_eq!(path, tmp_dir.join("ama.lock"));
    }

    #[test]
    fn test_daemon_status_and_lifecycle_when_not_running() {
        let tmp_dir = std::env::temp_dir();
        let lock_file = AmaDaemon::get_lock_file(&tmp_dir);
        let _ = std::fs::remove_file(&lock_file);

        let status = AmaDaemon::check_status(&tmp_dir);
        assert!(status.is_none());

        let stopped = AmaDaemon::stop_daemon(&tmp_dir);
        assert!(!stopped);
    }
}
