use chrono::Local;
use cron::Schedule;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

fn main() {
    let schedule = Schedule::from_str("*/10 * * * * *").unwrap();
    let job = Job {
        schedule,
        task: Box::new(task),
    };
    job.schedule()
}

fn task() {
    let dt = Local::now().time();
    println!("[{dt}] performed task")
}

struct Job {
    schedule: Schedule,
    task: Box<dyn Fn()>,
}

impl Job {
    fn schedule(&self) {
        loop {
            match self.schedule.upcoming(Local).next() {
                Some(firing_time) => {
                    let dur = firing_time - Local::now();
                    thread::sleep(dur.to_std().unwrap_or(Duration::from_millis(0)));
                    (self.task)()
                }
                None => break
            }
        }
    }
}
