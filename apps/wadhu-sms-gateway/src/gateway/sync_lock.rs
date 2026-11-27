//! Recover from poisoned mutexes instead of aborting the process.

use std::sync::{Mutex, MutexGuard};

pub fn lock_mutex<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    match m.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}