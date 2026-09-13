use crate::access::AccessError;
use chrono::Utc;
use chrono_tz::Tz;
use discord_webhook2::message::Message;
use discord_webhook2::webhook::DiscordWebhook;

pub struct DiscordNotif<'a> {
    image_tag: &'a str, // e.g. "docker.io/idk/idk:latest"
    // hostname: &str, // e.g. "pc-idkwhat"
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

    pub async fn send_discord_notif(&'a self, webhook_url: &str) -> Result<(), AccessError> {
        let client = DiscordWebhook::new(webhook_url)?;

        let content = format!("Docker tag {} is available.", self.image_tag);

        client
            .send(&Message::new(|message| {
                message.embed(|embed| {
                    embed
                        .title("Docker Notif")
                        .description(content)
                        .url("https://example.com")
                        .footer(|footer| footer.text("Big Brother is watching you"))
                        .author(|author| author.name("Big Brother"))
                        .field(|field| field.name("Created").value(self.created))
                        .field(|field| field.name("Digest").value(self.digest))
                        .field(|field| field.name("HubLink").value(self.hub_link))
                    // .field(|field| field.value("Value 3"))
                    // .color(0x00BBFF)
                })
            }))
            .await?;

        Ok(())
    }
}
