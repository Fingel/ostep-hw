use semaphore::Semaphore;
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

mod semaphore;

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
