use semaphore::Semaphore;
#[allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{
        atomic::{Ordering::*, *},
        *,
    },
    thread::{self, Thread},
};
use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread::JoinHandle,
};
use std::{thread::sleep, time::Duration};

mod semaphore;

// All children should post process started before ending
//

struct Barrier {
    started: Arc<Semaphore>,
    proceed: Arc<Semaphore>,
    waiters: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

fn child(started: Arc<Semaphore>, proceed: Arc<Semaphore>) {
    println!("Child process started");
    started.post();
    proceed.wait();
    println!("Child process ended");
}
fn main() {
    let barrier = Barrier {
        started: Arc::new(Semaphore::new(0)),
        proceed: Arc::new(Semaphore::new(0)),
        waiters: Arc::new(Mutex::new(Vec::new())),
    };

    for _ in 0..5 {
        let started_sem = barrier.started.clone();
        let proceed_sem = barrier.proceed.clone();
        let handle = std::thread::spawn(move || child(started_sem, proceed_sem));
        barrier.waiters.lock().unwrap().push(handle);
    }

    // Wait for all children to signal they've started
    for _ in 0..5 {
        barrier.started.wait();
    }

    // Allow all children to proceed
    for _ in 0..5 {
        barrier.proceed.post();
    }
    // Take ownership of the vector of handles to join them
    let handles = std::mem::take(&mut *barrier.waiters.lock().unwrap());
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Parent: End");
}
