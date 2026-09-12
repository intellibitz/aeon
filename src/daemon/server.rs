// Always-On AMA Master Daemon Process Manager
// 100% Rust implementation managing GMCP (Port 9090), GEMI (Port 9091) & A2A Cluster UDP (Port 9092)

use std::fs;
use std::io::{Write, Read, Seek, SeekFrom};
use std::net::UdpSocket;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;

use crate::gemi::GemiServer;
use crate::gmcp::server::GmcpServer;

pub struct AmaDaemon;

struct DaemonLock {
    file: fs::File,
}

impl DaemonLock {
    fn acquire(path: &Path) -> Result<Self, String> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .map_err(|e| e.to_string())?;

        #[cfg(unix)]
        {
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

        if cfg!(target_os = "windows") {
            let _ = Command::new(&bin_to_run)
                .arg("daemon-start")
                .arg(workspace.to_str().unwrap_or("."))
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
        } else {
            let _ = Command::new("nohup")
                .arg(bin_to_run)
                .arg("daemon-start")
                .arg(workspace.to_str().unwrap_or("."))
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
        }
    }

    pub fn run_daemon_loop(workspace: PathBuf, global_dir: PathBuf) {
        let lock_file_path = Self::get_lock_file(&global_dir);
        let mut lock = match DaemonLock::acquire(&lock_file_path) {
            Ok(l) => l,
            Err(_) => {
                eprintln!("[AmaDaemon] Failed to acquire lock. Daemon likely already running.");
                return;
            }
        };

        if let Err(e) = lock.write_pid() {
            eprintln!("[AmaDaemon] Failed to write PID to lock file: {}", e);
        }

        let cfg = crate::sandbox::manager::AeonConfig::load(&global_dir).expect("Fatal: Malformed configuration");

        // Substrate Administration & Hardware Optimization (Pillar 1)
        crate::daemon::runtime_admin::AeonRuntimeAdmin::start_administration_cycle(&workspace);

        let workspace_gemi = workspace.clone();
        let gemi_port = cfg.gemi_port;
        // 1. Spawn GEMI HTTP REST Server Thread (Port 9091 / Dynamic)
        thread::spawn(move || {
            GemiServer::start_http_server(workspace_gemi, gemi_port);
        });

        let workspace_gmcp = workspace.clone();
        let gmcp_port = cfg.gmcp_port;
        // 2. Spawn GMCP TCP Server Thread (Port 9090 / Dynamic)
        thread::spawn(move || {
            GmcpServer::start_tcp_server(workspace_gmcp, gmcp_port, crate::AEON_VERSION.to_string());
        });

        let workspace_gmcp_http = workspace.clone();
        let gmcp_http_port = cfg.gmcp_http_port;
        // 3. Spawn GMCP HTTP/SSE Server Thread (Port 9093 / Dynamic)
        thread::spawn(move || {
            GmcpServer::start_http_server(workspace_gmcp_http, gmcp_http_port);
        });

        let udp_port = cfg.udp_discovery_port;
        // 4. Spawn A2A Cluster UDP Discovery Listener Thread (Port 9092 / Dynamic)
        thread::spawn(move || {
            Self::start_udp_discovery_server(udp_port, gmcp_port);
        });

        // 5. Keep main daemon thread alive
        loop {
            thread::sleep(Duration::from_secs(86400));
        }
    }

    fn start_udp_discovery_server(port: u16, gmcp_port: u16) {
        let addr = format!("0.0.0.0:{}", port);
        if let Ok(socket) = UdpSocket::bind(&addr) {
            eprintln!("[A2A Cluster UDP] Discovery listener active on {}", addr);
            let mut buf = [0u8; 512];
            while let Ok((amt, src)) = socket.recv_from(&mut buf) {
                let msg = String::from_utf8_lossy(&buf[..amt]);
                if msg.contains("AEON_LAN_PING") {
                    let pong = format!("AEON_LAN_PONG:aeon-daemon-node:{}", gmcp_port);
                    let _ = socket.send_to(pong.as_bytes(), src);
                }
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
