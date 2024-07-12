use rcron::{Job, JobScheduler};

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("exec task every 10 seconds!");
    }));

    sched.add(Job::new("1/5 * * * * *".parse().unwrap(), || {
        println!("exec task every 5 seconds!");
    }));

    loop {
        sched.tick();

        std::thread::sleep(sched.time_till_next_job());
    }
}
