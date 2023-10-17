use std::thread;
use std::time::{Duration, Instant};

pub struct ScheduledTask<T> {
    interval: Duration,
    last_execution: Instant,
    task: T,
}

impl<T: FnMut() + Send> ScheduledTask<T> {
    pub fn new(interval: Duration, task: T) -> Self {
        ScheduledTask {
            interval,
            last_execution: Instant::now(),
            task,
        }
    }

    fn run_if_due(&mut self) {
        let elapsed_time = self.last_execution.elapsed();

        if elapsed_time >= self.interval {
            (self.task)();
            self.last_execution = Instant::now();
        }
    }
}

pub fn schedule_tasks<T: FnMut() + Send + 'static>(tasks: &mut Vec<ScheduledTask<T>>) {
    let mut threads = vec![];

    for mut scheduled_task in tasks.drain(..) {
        let thread_handle = thread::spawn(move || {
            loop {
                scheduled_task.run_if_due();
                thread::sleep(Duration::from_millis(100));
            }
        });

        threads.push(thread_handle);
    }

    for thread_handle in threads {
        thread_handle.join().unwrap();
    }
}

pub mod utils;
