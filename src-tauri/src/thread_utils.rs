use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::panic::{catch_unwind, AssertUnwindSafe};
use tracing::{error, info, warn};

// Define a trait for the worker function passed to spawn_restartable_thread
// The worker function returns a Result:
// - Ok(()): Indicates the current unit of work completed, and the supervisor can re-invoke if needed.
// - Err(String): Indicates a desire to stop the thread gracefully, not due to a panic.
pub trait ThreadFn: FnMut() -> Result<(), String> + Send + 'static {}
impl<F: FnMut() -> Result<(), String> + Send + 'static> ThreadFn for F {}

/// Spawns a thread that will attempt to restart its core logic if it panics
/// or if its worker function completes and is designed to be restarted.
/// Includes a graceful shutdown mechanism via an AtomicBool.
pub fn spawn_restartable_thread<F>(
    thread_name: String,
    mut work_fn: F,
    shutdown_signal: Arc<AtomicBool>,
) where
    F: ThreadFn,
{
    thread::spawn(move || {
        info!("[{}] Starting thread", thread_name);
        while !shutdown_signal.load(Ordering::Relaxed) {
            // Ensure any captured state by work_fn is either cloneable for each iteration
            // or that work_fn is designed to re-initialize itself.
            let result = catch_unwind(AssertUnwindSafe(|| {
                work_fn() // Execute the provided worker function
            }));

            match result {
                Ok(Ok(_)) => {
                    // work_fn completed its current task successfully without signaling a stop.
                    // If work_fn is a long-running task that exited, this implies it should be restarted.
                    info!("[{}] Worker function completed a cycle. Checking for shutdown before potential restart.", thread_name);
                }
                Ok(Err(stop_reason)) => {
                    // work_fn completed and explicitly signaled to stop the restart loop.
                    warn!(
                        "[{}] Worker function signaled stop: {}. Thread exiting.",
                        thread_name, stop_reason
                    );
                    break; // Exit the while loop, terminating the thread.
                }
                Err(panic_payload) => {
                    if shutdown_signal.load(Ordering::Relaxed) {
                        info!("[{}] Thread panicked during shutdown. Suppressing restart. Panic: {:?}", thread_name, panic_payload);
                        break;
                    }
                    error!(
                        "[{}] Thread panicked: {:?}. Restarting after a delay...",
                        thread_name, panic_payload
                    );
                    // Brief delay before restarting. Check shutdown signal again before sleeping.
                    for _ in 0..50 {
                        // Sleep for 5 seconds, but check shutdown every 100ms
                        if shutdown_signal.load(Ordering::Relaxed) {
                            info!("[{}] Shutdown signaled during panic recovery delay. Thread exiting.", thread_name);
                            return; // Exit thread
                        }
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }

            // Brief pause if not shutting down, to prevent tight spinning if work_fn exits very quickly
            // and is intended to be restarted.
            if !shutdown_signal.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(200));
            }
        }
        info!("[{}] Thread shutting down.", thread_name);
    });
} 