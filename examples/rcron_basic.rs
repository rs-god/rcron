use rcron::{Job, JobScheduler};
// use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("exec task every 10 seconds!");
    }));

    let job = Job::new("1/5 * * * * *".parse().unwrap(), || {
        println!("exec task every 5 seconds!");
    });
    sched.add(job);

    loop {
        sched.tick();
        // std::thread::sleep(Duration::from_millis(500));

        std::thread::sleep(sched.time_till_next_job());
    }
}
