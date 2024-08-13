use core::num;
use std::{ collections::VecDeque, sync::{ Arc, Condvar, Mutex }, thread, time::Duration, usize };
pub fn ch_1() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from main thread");
    t1.join().unwrap();
    t2.join().unwrap();

    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("average:{average}");
    scoped_threads();
    reference_counting();
    mutex();
    // thread_parking();
    conditional_variables();
}

fn f() {
    println!("Hello from another thread");

    let id = thread::current().id();
    println!("This is my thread id :{id:?} ")
}

fn scoped_threads() {
    let numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("length:{}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}
fn reference_counting() {
    let a = Arc::new([1, 2, 4]);
    let b = a.clone();

    thread
        ::spawn(move || dbg!(a))
        .join()
        .unwrap();
    thread
        ::spawn(move || { dbg!(b) })
        .join()
        .unwrap();
}
fn mutex() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000)
}
// keep the amount of time mutex is locked as short as possible.
// keeping a mutex locked longer than necessary can completely nullify the
// benifits of parallelism, effectively forcing every-thing to happen
// serially instead

// fn reader_writer_lock() {
//     /*
//    A reader-writer lock is slightly more complicated version of a mutex that
//    understands the difference between exclusive and shared access, and can
//    provide either.
//    It has three states:
//     1) unlocked
//     2) locaked by a single writer (for exclusive access)
//     3) locked by a number of readers (for shared access)

//     commonly used for data that is often read by multiple threads,
//     but only updated once in a while
//    */
// }

/*
    Thread Parking: 
      One way to wait for a notification from another thread is called thread
      parking. A thread can park itself, which puts it to sleep, stopping it from
      consuming any CPU cycles. Another thread can then unpark the parked thread,
      waking it up from its nap


*/

fn thread_parking() {
    let queue = Mutex::new(VecDeque::new());
    thread::scope(|s| {
        let t = s.spawn(|| {
            loop {
                let item = queue.lock().unwrap().pop_front();
                if let Some(item) = item {
                    dbg!(item);
                } else {
                    thread::park();
                }
            }
        });
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    })
}

fn conditional_variables() {
    // more commonly used option for waiting for something to happen to
    // data protected by a mutex.
    // They have two basic operations: wait and notify.

    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();
    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        q = not_empty.wait(q).unwrap();
                    }
                };
                drop(q);
                dbg!(item);
            }
        });
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    })
}
/*
  SUMMARY:
  --> Multiple threads can run within the same program and can be spawned at any
  time.
  --> When the main thread ends, the entire program ends.
  --> Data that is Send can be sent to other threads, data that is Sync can be
     shared bw threads.
  --> Regular thread might run as long as the program does, and thus can only
      borrow 'static data such as statics and leaked allocations.
  --> Reference Counting (Arc) can be used to share ownership to make sure data
  lives as long as at least one thread is using it.
  --> Scoped threads are useful to limit lifetime of a thread to allow it to 
  borrow non-'static data, such as local variables.
  --> Thread parking can be a convinient way to wait for some condition.
  --> when a condition is about data protected by a mutex, using a Condvar is more
  convinient ,and can be more efficient, than parking.
*/
