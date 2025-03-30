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
    env::args,
    sync::atomic::{AtomicU32, Ordering},
    thread::JoinHandle,
};
use std::{thread::sleep, time::Duration};

mod semaphore;

fn main() {
    let num_readers = args().nth(1).unwrap().parse().unwrap();
    let num_writers = args().nth(2).unwrap().parse().unwrap();
    let mut handles = Vec::new();

    let value = Arc::new(RwLock::new(0));

    println!(
        "Starting with {} readers and {} writers",
        num_readers, num_writers
    );
    for _ in 0..num_readers {
        let value = value.clone();
        handles.push(thread::spawn(move || {
            let r = value.read().unwrap();
            println!("Reader: Start");
            sleep(Duration::from_secs(1));
            println!("Reader: Value: {}", r);
            println!("Reader: End");
        }));
    }
    for _ in 0..num_writers {
        let value = value.clone();
        handles.push(thread::spawn(move || {
            let mut w = value.write().unwrap();
            println!("Writer: Start");
            sleep(Duration::from_secs(1));
            *w += 1;
            println!("Writer: End value {}", *w);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Main: End");
}
