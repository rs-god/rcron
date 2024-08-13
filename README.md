# rcron

a simple cron-like job scheduling library for Rust

## Usage

Be sure to add the rcron crate to your `Cargo.toml`:

```toml
[dependencies]
rcron = "1.2.3"
```

Creating a schedule for a job is done using the `FromStr` impl for the
`Schedule` type of the [cron](https://github.com/zslayton/cron) library.

The scheduling format is as follows:

```text
sec   min   hour   day of month   month   day of week   year
*     *     *      *              *       *             *
```

Time is for `Local` your local timezone.

Comma separated values such as `5,8,10` represent more than one time value. So
for example, a schedule of `0 2,14,26 * * * *` would execute on the 2nd, 14th,
and 26th minute of every hour.

Ranges can be specified with a dash. A schedule of `0 0 * 5-10 * *` would
execute once per hour but only on day 5 through 10 of the month.

Day of the week can be specified as an abbreviation or the full name. A
schedule of `0 0 6 * * Sun,Sat` would execute at 6am on Sunday and Saturday.

A simple usage example:

```rust
use rcron::{JobScheduler, Job};
use std::time::Duration;

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

        std::thread::sleep(Duration::from_millis(500));
        
        // Or use the following method.
        // The `time_till_next_job` method returns the duration till the next job is supposed to run. 
        // std::thread::sleep(sched.time_till_next_job());
    }
}
```

# example
```shell
cargo run --package rcron --example rcron_basic 

    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/rcron_basic`
exec task every 10 seconds!
exec task every 5 seconds!
exec task every 5 seconds!
exec task every 10 seconds!
exec task every 5 seconds!
exec task every 5 seconds!
exec task every 10 seconds!
exec task every 5 seconds!
exec task every 5 seconds!
```

## Similar Libraries

* [cron](https://github.com/zslayton/cron) the cron expression parser we use.
* [schedule-rs](https://github.com/mehcode/schedule-rs) is a similar rust library that implements it's own cron expression parser.
* [tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler) Schedule tasks on Tokio using cron-like annotation.

# refer
- https://github.com/lholden/job_scheduler
- https://github.com/mvniekerk/tokio-cron-scheduler

## License

rcron is licensed under either of

 * MIT license
