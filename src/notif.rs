use chrono::Utc;
use chrono_tz::Tz;
use webhook::client::WebhookClient;
pub struct DiscordNotif<'a> {
    image_tag: &'a str, // e.g. "docker.io/idk/idk:latest"
    // hostname: &str, // e.g. "pc-hades"
    created: &'a str, // e.g. "Aug 30, 2021 13:26:31 UTC"
    digest: &'a str,  // e.g. "sha256:866c12d..."
    // platform: &str, // e.g. "linux/amd64"
    hub_link: &'a str, // e.g. "https://fasdjkfhkasdhf"
}

//helper function to know the time
pub fn get_current_time(tz: &Tz) -> String {
    let now = Utc::now();
    let local_time = now.with_timezone(tz);
    local_time.format("%b %d, %Y %H:%M:%S %Z").to_string()
}

impl<'a> DiscordNotif<'a> {
    pub fn new(image_tag: &'a str, created: &'a str, digest: &'a str, hub_link: &'a str) -> Self {
        Self {
            image_tag,
            created,
            digest,
            hub_link,
        }
    }

    pub async fn send_discord_notif(&'a self, webhook_url: &str) -> Result<(), String> {
        let client = WebhookClient::new(webhook_url);

        // let logo_url = "https://idkwhat";

        // 1. Text displayed above the embed box

        let content = format!("Docker tag {}  is available.", self.image_tag);

        // 2. Build and send the payload

        client
            .send(|message| {
                message
                    .username("Small Brother")
                    .content(&content)
                    .embed(|embed| {
                        embed
                            // Header at the top of the card
                            .author("Big Brother", None, None)
                            // Key-value rows (name, value, inline = false)
                            .field("Created", self.created, false)
                            .field("Digest", self.digest, false)
                            .field("HubLink", self.hub_link, false)
                            // Small text at the very bottom
                            .footer("Big Brother is watching you", None)
                    })
            })
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
