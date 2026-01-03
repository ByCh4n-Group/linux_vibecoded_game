use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct DiscordRpc {
    client: Option<DiscordIpcClient>,
    start_time: i64,
}

impl DiscordRpc {
    pub fn new(app_id: &str) -> Self {
        let mut client = DiscordIpcClient::new(app_id);
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        match client.connect() {
            Ok(_) => {
                println!("Discord IPC connected successfully.");
                Self {
                    client: Some(client),
                    start_time,
                }
            },
            Err(e) => {
                eprintln!("Failed to connect to Discord IPC: {:?}", e);
                Self {
                    client: None,
                    start_time,
                }
            }
        }
    }

    pub fn update_status(&mut self, details: &str, state: &str) {
        if let Some(ref mut client) = self.client {
            let payload = activity::Activity::new()
                .state(state)
                .details(details)
                .timestamps(activity::Timestamps::new().start(self.start_time))
                .assets(
                    activity::Assets::new()
                        .large_image("fesli_chara")
                        .large_text("GorkiTale"),
                );

            if let Err(e) = client.set_activity(payload) {
                eprintln!("Failed to set Discord activity: {:?}", e);
            }
        }
    }
}
