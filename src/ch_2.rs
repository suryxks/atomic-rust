use std::{
    sync::atomic::{ AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering::Relaxed },
    thread::{ self, sleep },
    time::{ Duration, Instant },
};

pub fn ch_2() {
    example_statistics();
    // example_process_reporting_multiple_threads();
}
fn process_item(i: usize) {
    sleep(Duration::from_secs(1));
    // println!("processing {i}")
}
fn example_process_reporting_multiple_threads() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i); // Assuming this takes some time.
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        // The main thread shows status updates, every second.
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_millis(100));
        }
    });

    println!("Done!");
}

fn example_statistics() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }

            if n == 0 {
                println!("working nothing yet...");
            } else {
                println!(
                    "Working.. {n}/100 done {:?} average, {:?} peak",
                    total_time / (n as u32),
                    max_time
                );
            }
            thread::sleep(Duration::from_millis(100));
        }
    })
}
fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => {
                return;
            }
            Err(v) => {
                current = v;
            }
        }
    }
}
fn id_allocation() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < 1000, "too many IDs!");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => {
                return id;
            }
            Err(v) => {
                id = v;
            }
        }
    }
}
