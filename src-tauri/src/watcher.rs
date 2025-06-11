use std::path::PathBuf;
use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tracing::warn;

use libc::{pid_t, fork, pipe, setsid, execl, getpid, close, write, read, waitpid, _exit};

static WATCHER_BIN: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/watcher"));

#[derive(Debug)]
pub struct Watcher {
    aeroe_pid: u32,
    file: PathBuf,
    wallet_dir: PathBuf,
}

impl Watcher {
    pub fn new(file: PathBuf, wallet_dir: PathBuf) -> Self {
        let aeroe_pid = std::process::id();
        Self {
            aeroe_pid,
            file,
            wallet_dir,
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        self.deploy_watcher()?;
        let mut watcher_pid = self.start_watcher()?;

        loop {
            let watcher_pid_t: pid_t = watcher_pid as pid_t;
            let result = unsafe { libc::kill(watcher_pid_t, 0) };
            if result == -1 {
                // If ESRCH, the process does not exist
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
                if errno == libc::ESRCH {
                    warn!("Process {} has exited. Restarting watcher...", watcher_pid_t);
                    self.deploy_watcher()?;
                    watcher_pid = self.start_watcher()?;
                } else {
                    warn!("Unexpected error when checking watcher process: {}", errno);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        // Ok(()) // unreachable
    }

    fn deploy_watcher(&self) -> Result<(), String> {
        // Write the embedded binary to self.file
        fs::write(&self.file, WATCHER_BIN)
            .map_err(|e| format!("Failed to write watcher binary: {}", e))?;

        // Set executable permissions (0o755)
        let mut perms = fs::metadata(&self.file)
            .map_err(|e| format!("Failed to get metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&self.file, perms)
            .map_err(|e| format!("Failed to set permissions: {}", e))?;
        Ok(())
    }

    fn start_watcher(&self) -> Result<u32, String> {
        // Convert paths and aeroe_pid to CStrings
        let file_str = self
            .file
            .to_str()
            .ok_or_else(|| "Failed to convert watcher path to string".to_string())?;
        let c_file = CString::new(file_str)
            .map_err(|e| format!("Invalid watcher path (interior nul?): {}", e))?;
        let c_arg0 = c_file.clone();

        let c_arg1 = CString::new(self.aeroe_pid.to_string())
            .map_err(|e| format!("Invalid aeroe_pid to CString: {}", e))?;

        let wallet_str = self
            .wallet_dir
            .to_str()
            .ok_or_else(|| "Failed to convert wallet_dir to string".to_string())?;
        let c_arg2 = CString::new(wallet_str)
            .map_err(|e| format!("Invalid wallet_dir (interior nul?): {}", e))?;

        unsafe {
            // Create a pipe so the grandchild can send its PID back
            let mut fds: [c_int; 2] = [0, 0];
            if pipe(fds.as_mut_ptr()) == -1 {
                return Err("pipe failed".into());
            }
            let read_fd = fds[0];
            let write_fd = fds[1];

            // First fork
            match fork() {
                -1 => {
                    close(read_fd);
                    close(write_fd);
                    return Err("first fork failed".into());
                }
                0 => {
                    // ─── In FIRST CHILD ───
                    close(read_fd);

                    // Detach from any controlling terminal / session
                    if setsid() == -1 {
                        let err_pid: pid_t = -1;
                        let _ = write(
                            write_fd,
                            &err_pid as *const _ as *const c_void,
                            size_of::<pid_t>(),
                        );
                        close(write_fd);
                        _exit(1);
                    }

                    // Second fork
                    match fork() {
                        -1 => {
                            let err_pid: pid_t = -1;
                            let _ = write(
                                write_fd,
                                &err_pid as *const _ as *const c_void,
                                size_of::<pid_t>(),
                            );
                            close(write_fd);
                            _exit(1);
                        }
                        pid2 if pid2 > 0 => {
                            // ─── INTERMEDIATE CHILD ───
                            _exit(0);
                        }
                        _ => {
                            // ─── GRANDCHILD (the actual watcher) ───
                            let watcher_pid = getpid();
                            let pid_to_write: pid_t = watcher_pid;
                            let _ = write(
                                write_fd,
                                &pid_to_write as *const _ as *const c_void,
                                size_of::<pid_t>(),
                            );
                            close(write_fd);

                            // Replace with the watcher binary
                            execl(
                                c_file.as_ptr(),
                                c_arg0.as_ptr(),
                                c_arg1.as_ptr(),
                                c_arg2.as_ptr(),
                                ptr::null::<c_void>() as *const _,
                            );
                            _exit(1);
                        }
                    }
                }
                child1_pid => {
                    // ─── In ORIGINAL PARENT ───
                    close(write_fd);

                    // Read exactly sizeof(pid_t) bytes
                    let mut pid_buf: pid_t = 0;
                    let bytes_to_read = size_of::<pid_t>() as usize;
                    let n = read(
                        read_fd,
                        &mut pid_buf as *mut _ as *mut c_void,
                        bytes_to_read,
                    );
                    close(read_fd);

                    // Reap the intermediate child
                    let mut status: c_int = 0;
                    let _ = waitpid(child1_pid, &mut status as *mut _, 0);

                    if n != bytes_to_read as isize {
                        return Err("failed to read watcher pid".into());
                    }
                    if pid_buf <= 0 {
                        return Err("failed to start watcher".into());
                    }
                    return Ok(pid_buf as u32);
                }
            }
        }
    }
}
