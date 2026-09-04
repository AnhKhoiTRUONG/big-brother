mod access;
mod notif;
mod parse_yaml;

use chrono_tz::Tz;
use std::str::FromStr;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::parse_yaml::Config;

#[tokio::main]
async fn main() {
    //If the config is not gud, we use the default config
    let config = parse_yaml::Config::parse_yaml().unwrap_or_else(|e| {
        eprintln!("Yaml parsing error, falling back to default config: {e}");
        Config::default()
    });

    let mut sched = JobScheduler::new()
        .await
        .expect("Failed to initilize job schedule");

    let tz = Tz::from_str(&config.watch.timezone).expect("Failed to parse timezone");
    let maybe_discord_config = config.discord.clone();

    sched
        .add(
            Job::new_async_tz(&config.watch.schedule, tz, move |uuid, mut l| {
                Box::pin({
                    let value = maybe_discord_config.clone();
                    async move {
                        match access::compare_all_digest(&tz, &value).await {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("{e}")
                            }
                        }

                        // Query the next execution time for this job
                        let next_tick = l.next_tick_for_job(uuid).await;
                        match next_tick {
                            Ok(Some(ts)) => println!("Next time for job is {:?}", ts),
                            _ => println!("Could not get next tick"),
                        }
                    }
                })
            })
            .unwrap_or_else(|e| {
                eprintln!("Configuration Error: Failed to parse schedule.");
                eprintln!("Reason: {e}");
                std::process::exit(1);
            }),
        )
        .await
        .unwrap();

    sched.start().await.expect("Failed to start job");
    println!("Scheduler started.");

    // 3. Prevent the main function from exiting
    // A simple infinite loop keeps the Tokio runtime alive
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
