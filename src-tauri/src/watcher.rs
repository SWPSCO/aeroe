<<<<<<< HEAD
use std::path::{Path, PathBuf};
=======
>>>>>>> feature/wallet-and-frontend-improvements
use std::ffi::CString;
use std::fs;
use std::mem::size_of;
use std::os::raw::{c_int, c_void};
<<<<<<< HEAD
use std::ptr;
use std::env;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use tracing::warn;

use libc::{pid_t, fork, pipe, setsid, execl, getpid, close, write, read, waitpid, _exit, dup2, STDOUT_FILENO, STDERR_FILENO};
=======
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::ptr;
use tracing::warn;

use libc::{_exit, close, execl, fork, getpid, pid_t, pipe, read, setsid, waitpid, write};

static WATCHER_BIN: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/watcher"));
>>>>>>> feature/wallet-and-frontend-improvements

#[derive(Debug)]
pub struct Watcher {
    wallet_dir: PathBuf,
    log_dir: PathBuf,
}

impl Watcher {
    pub fn new(wallet_dir: PathBuf, log_dir: PathBuf) -> Self {
        Self { wallet_dir, log_dir }
    }

    pub async fn start(&self) -> Result<(), String> {
        // Resolve the path to the signed sidecar binary within the app bundle
        let sidecar_path = env::current_exe()
            .map_err(|e| format!("Failed to get current exe path: {}", e))?
            .parent()
            .ok_or_else(|| "Failed to get parent directory of main executable".to_string())?
            //.join(format!("watcher-{}", env!("TAURI_ENV_TARGET_TRIPLE")));
            .join(format!("watcher"));

        if !sidecar_path.exists() {
            return Err(format!("Watcher binary not found at path: {:?}", sidecar_path));
        }

        let mut watcher_pid = self.start_watcher(&sidecar_path)?;

        // This loop now correctly monitors the PID of the true daemon process
        loop {
            let watcher_pid_t: pid_t = watcher_pid as pid_t;
            let result = unsafe { libc::kill(watcher_pid_t, 0) };
            if result == -1 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
                if errno == libc::ESRCH {
<<<<<<< HEAD
                    warn!("Daemon process {} has exited. Restarting...", watcher_pid_t);
                    watcher_pid = self.start_watcher(&sidecar_path)?;
=======
                    warn!(
                        "Process {} has exited. Restarting watcher...",
                        watcher_pid_t
                    );
                    self.deploy_watcher()?;
                    watcher_pid = self.start_watcher()?;
>>>>>>> feature/wallet-and-frontend-improvements
                } else {
                    warn!("Unexpected error when checking daemon process: {}", errno);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    // This function is a restoration of your original, correct double-fork
    // daemonization logic. It now executes the pre-signed binary at `file`.
    fn start_watcher(&self, file: &Path) -> Result<u32, String> {
        let aeroe_pid = std::process::id();
        let wallet_dir_str = self.wallet_dir.to_str().ok_or("wallet dir path is not valid utf8")?;

        let c_file = CString::new(file.to_str().unwrap()).map_err(|e| e.to_string())?;
        let c_arg0 = c_file.clone();
        let c_arg1 = CString::new(aeroe_pid.to_string()).map_err(|e| e.to_string())?;
        let c_arg2 = CString::new(wallet_dir_str).map_err(|e| e.to_string())?;

        unsafe {
            let mut fds: [c_int; 2] = [0, 0];
            if pipe(fds.as_mut_ptr()) == -1 { return Err("pipe failed".into()); }
            let read_fd = fds[0];
            let write_fd = fds[1];

            match fork() {
                -1 => {
                    close(read_fd);
                    close(write_fd);
                    return Err("first fork failed".into());
                }
                0 => { // In FIRST CHILD
                    close(read_fd);
                    if setsid() == -1 { _exit(1); }
                    match fork() {
                        -1 => { _exit(1); }
                        pid2 if pid2 > 0 => { _exit(0); } // In INTERMEDIATE CHILD
                        _ => { // In GRANDCHILD (the daemon)
                            // Redirect stdout and stderr to a log file
                            let log_path = self.log_dir.join("watcher.log");
                            if let Ok(file) = File::create(&log_path) {
                                let fd = file.as_raw_fd();
                                dup2(fd, STDOUT_FILENO);
                                dup2(fd, STDERR_FILENO);
                            }

                            let watcher_pid = getpid();
                            let _ = write(write_fd, &watcher_pid as *const _ as *const c_void, size_of::<pid_t>());
                            close(write_fd);
                            execl(c_file.as_ptr(), c_arg0.as_ptr(), c_arg1.as_ptr(), c_arg2.as_ptr(), ptr::null::<c_void>() as *const _);
                            _exit(1); // execl should not return
                        }
                    }
                }
                child1_pid => { // In ORIGINAL PARENT
                    close(write_fd);
                    let mut pid_buf: pid_t = 0;
                    let bytes_to_read = size_of::<pid_t>() as usize;
                    let n = read(read_fd, &mut pid_buf as *mut _ as *mut c_void, bytes_to_read);
                    close(read_fd);
                    let mut status: c_int = 0;
                    let _ = waitpid(child1_pid, &mut status as *mut _, 0);

                    if n != bytes_to_read as isize || pid_buf <= 0 {
                        return Err("failed to read daemon pid or daemon failed to start".into());
                    }
                    return Ok(pid_buf as u32);
                }
            }
        }
    }
}
