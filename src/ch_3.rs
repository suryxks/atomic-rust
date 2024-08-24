use std::{
    sync::atomic::{
        AtomicBool,
        AtomicI32,
        AtomicPtr,
        AtomicU64,
        Ordering::{ Acquire, Relaxed, Release, SeqCst },
    },
    thread::{ self, yield_now },
    time::Duration,
};

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);
static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn a() {
    X.store(10, Relaxed);
    Y.store(20, Relaxed);
}
fn b() {
    let y = Y.load(Relaxed);
    let x = X.load(Relaxed);
    println!("{x} {y}")
}

pub fn ch_3() {
    // X.store(1, Relaxed);
    // let t = thread::spawn(f);
    // X.store(2, Relaxed);
    // t.join().unwrap();
    // X.store(3, Relaxed);
    // ex();
    // release_and_acquire_ordering();
    // thread::scope(|s| {
    //     for _ in 0..100 {
    //         s.spawn(ex_locking);
    //     }
    // });
    // println!("{:?}", unsafe { DATA.to_string() })
    seq_cst();
}

fn f() {
    let x = X.load(Relaxed);
    assert!(x == 1 || x == 2);
}
fn ex() {
    let a = thread::spawn(|| {
        let x = X.load(Relaxed);
        Y.store(x, Relaxed)
    });
    let b = thread::spawn(|| {
        let y = Y.load(Relaxed);
        X.store(y, Relaxed);
    });
    a.join().unwrap();
    b.join().unwrap();
    assert_eq!(X.load(Relaxed), 0);
    assert_eq!(Y.load(Relaxed), 0);
}
fn release_and_acquire_ordering() {
    static mut DATA: u64 = 0;
    // static DATA: AtomicU64 = AtomicU64::new(0);
    static READY: AtomicBool = AtomicBool::new(false);

    thread::spawn(|| {
        // DATA.store(123, Relaxed);

        unsafe {
            DATA = 123;
        }
        READY.store(true, Release);
    });
    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting");
    }
    // println!("{}", DATA.load(Relaxed));
    println!("{}", unsafe { DATA });
}
fn ex_locking() {
    if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
        unsafe {
            DATA.push('!');
            LOCKED.store(false, Release);
        }
    }
}
fn seq_cst() {
    static A: AtomicBool = AtomicBool::new(false);
    static B: AtomicBool = AtomicBool::new(false);

    static mut S: String = String::new();

    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe {
                S.push('!');
            }
        }
    });
    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe {
                S.push('1');
            }
        }
    });
    a.join().unwrap();
    b.join().unwrap();

    println!("{:?}", unsafe { S.as_str() })
}

/*
                             MEMORY ORDERING
    
HAPPENS BEFORE RELATIONSHIP:
     The memory model defines the order in which operations are executed in terms
     of happens before relationships. This means that it does not say anything about
     machine instructions,caches, buffers, timing , instruction reordering, compiler
     optimizatoins , and so on ,but instead only defines situations where one opreations 
     is guaranteed to happen before the other and leaves the order of everything else
     undefined. 

SeqCst ORDERING:
     While atomic operations using relaxed memory ordering do not provide any happens 
     before relationship, the do guarantee a total modification order of each individual atomic variable.
     All modifications of the same atomic vairable happen in an order that is the same
     from the prespective of every single thread

RELEASE AND ACQUIRE ORDERING:
     Release and acquire memory ordering are used in a pair to form a happens-before
     relationship between threads. Release memory ordering applies to store operations
     while Acquire memory ordering applies to load operations.
     
     A happens before relationship is formed when an acquire-load operation observes
     the result of a release-store operation. In this case , the store and everything
     before it happened before the load and everything after it.

Sequentially Consistent Ordering:
     The strongest memory ordering is sequentially consistent ordering 
     It includes all guarantess of acquire ordering and release ordering and 
     also guarantees a globally onsistent order of operations.

     This means that every single operation using SeqCst ordering within a program
     is part of a single total order that all threads agree on. This total order
     is consistent with the total modification order of each individual variable.


     Since it is strictly stornger than acquire and release memory ordering, a 
     sequentially consistent load or store can take place of a Releaase store or 
     Acquire-load in a release acquire pair and forms a happens before relationship.
     
     In other words , an acquire-load can not only form a happens before relationship 
     with release-store but also with a SeqCst store and similarly the other way around.

*/
