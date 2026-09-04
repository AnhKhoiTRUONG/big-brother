// Right now to config the cron job, we can use .yaml file
// The format will be:
// watch:
//      schedule: "0 */6 * * *"
//      timezone: Europe/Paris
//
// The cron format
//sec   min   hour   day of month   month   day of week
//*     *     *      *              *       *

use serde::Deserialize;
use std::fs;

const DEFAULT_SCHEDULE: &str = "0 */6 * * * *";
const DEFAULT_TZ: &str = "UTC";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub watch: Watch,
    pub discord: Option<DiscordConfig>,
}

#[derive(Debug, Deserialize)]
pub struct Watch {
    pub schedule: String,
    pub timezone: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordConfig {
    pub webhook_url: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            watch: Watch::default(),
            discord: None,
        }
    }

    pub fn parse_yaml() -> Result<Self, String> {
        let contents = fs::read_to_string("config.yaml").map_err(|e| e.to_string())?;

        serde_saphyr::from_str(contents.as_str()).map_err(|e| e.to_string())
    }
}

impl Watch {
    pub fn default() -> Self {
        Self {
            schedule: DEFAULT_SCHEDULE.to_string(),
            timezone: DEFAULT_TZ.to_string(),
        }
    }
}
