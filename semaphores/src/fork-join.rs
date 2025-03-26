use std::sync::atomic::{AtomicU32, Ordering};
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
use std::{thread::sleep, time::Duration};
struct Semaphore {
    mutex: Mutex<u32>,
    cond: Condvar,
}

impl Semaphore {
    fn new(value: u32) -> Self {
        Semaphore {
            cond: Condvar::new(),
            mutex: Mutex::new(value),
        }
    }

    fn wait(&self) {
        let mut guard = self.mutex.lock().unwrap();
        while *guard == 0 {
            guard = self.cond.wait(guard).unwrap();
        }
        *guard -= 1;
    }

    fn post(&self) {
        let mut guard = self.mutex.lock().unwrap();
        *guard += 1;
        self.cond.notify_one();
    }
}

fn child(semaphore: Arc<Semaphore>) {
    println!("Child process started");
    thread::sleep(Duration::from_secs(1));
    println!("Child process ended");
    semaphore.post();
}
fn main() {
    let semaphore = Arc::new(Semaphore::new(0));
    let c_semaphore = semaphore.clone();
    println!("Parent: Begin");
    std::thread::spawn(move || child(c_semaphore));
    semaphore.wait();
    println!("Parent: End");
}
