// Always-On AMA Master Daemon Process Manager
// 100% Rust implementation managing GMCP (Port 9090), GEMI (Port 9091) & A2A Cluster UDP (Port 9092)

use std::fs;
use std::net::{TcpStream, UdpSocket};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use crate::gemi::GemiServer;
use crate::gmcp::server::GmcpServer;

pub struct AmaDaemon;

impl AmaDaemon {
    pub fn get_lock_file(global_dir: &Path) -> PathBuf {
        global_dir.join("ama.lock")
    }

    pub fn check_status(global_dir: &Path) -> Option<u32> {
        let lock_file = Self::get_lock_file(global_dir);
        if let Ok(content) = fs::read_to_string(&lock_file)
            && let Ok(pid) = content.trim().parse::<u32>()
        {
            if cfg!(target_os = "linux") {
                let proc_path = PathBuf::from(format!("/proc/{}", pid));
                if proc_path.exists() {
                    return Some(pid);
                }
            } else {
                // Cross-platform fallback for Windows & macOS: TCP ping on GMCP server port
                let cfg = crate::sandbox::manager::AeonConfig::load(global_dir);
                let addr = format!("127.0.0.1:{}", cfg.gmcp_port);
                if let Ok(addr_parsed) = addr.parse()
                    && TcpStream::connect_timeout(&addr_parsed, Duration::from_millis(100)).is_ok()
                {
                    return Some(pid);
                }
            }
        }
        None
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
        let pid = std::process::id();
        let lock_file = Self::get_lock_file(&global_dir);
        let _ = fs::write(&lock_file, pid.to_string());

        let cfg = crate::sandbox::manager::AeonConfig::load(&global_dir);

        // High-Priority Hardware-Bounded Model Auto-Provisioning (Background Thread)
        crate::gemi::models::ModelManager::spawn_background_hardware_model_provisioner(&workspace);

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

        // 4. Keep main daemon thread alive
        loop {
            thread::sleep(Duration::from_secs(3600));
        }
    }

    fn start_udp_discovery_server(port: u16, gmcp_port: u16) {
        let addr = format!("0.0.0.0:{}", port);
        if let Ok(socket) = UdpSocket::bind(&addr) {
            eprintln!("🌐 [A2A Cluster UDP] Discovery listener active on {}", addr);
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
