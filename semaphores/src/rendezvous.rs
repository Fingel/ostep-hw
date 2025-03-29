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

// Both children should post process started before ending

fn child(started: Arc<Semaphore>, proceed: Arc<Semaphore>) {
    println!("Child process started");
    started.post();
    proceed.wait();
    thread::sleep(Duration::from_secs(1));
    println!("Child process ended");
}
fn main() {
    let start_semaphore = Arc::new(Semaphore::new(0));
    let proceed_semaphore = Arc::new(Semaphore::new(0));

    let started_sem1 = start_semaphore.clone();
    let proceed_sem1 = proceed_semaphore.clone();
    let handle1 = std::thread::spawn(move || child(started_sem1, proceed_sem1));

    let started_sem2 = start_semaphore.clone();
    let proceed_sem2 = proceed_semaphore.clone();
    let handle2 = std::thread::spawn(move || child(started_sem2, proceed_sem2));

    start_semaphore.wait();
    start_semaphore.wait();
    proceed_semaphore.post();
    proceed_semaphore.post();
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("Parent: End");
}
