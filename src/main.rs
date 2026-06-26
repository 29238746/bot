use azalea::prelude::*;
use azalea::prelude::AppExit;   // ✅ FIXED: AppExit is inside prelude

const PASSWORD: &str = "Botczosnikowy345";   // 👈 CHANGE THIS

#[tokio::main]
async fn main() -> AppExit {
    let account = Account::offline("zenix_czosnikowy");  // 👈 CHANGE THIS
    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "renciarze.aternos.me")          // 👈 CHANGE THIS
        .await
}

#[derive(Default, Clone, Component)]
pub struct State;

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(chat_packet) => {
            let (username_opt, message) = chat_packet.split_sender_and_content();
            let msg_lower = message.to_ascii_lowercase();

            if msg_lower.contains("/register") {
                bot.chat(&format!("/register {} {}", PASSWORD, PASSWORD));
            } else if msg_lower.contains("/login") {
                bot.chat(&format!("/login {}", PASSWORD));
            }

            if let Some(username) = username_opt {
                if username != bot.username() && message.starts_with('!') {
                    let args: Vec<&str> = message[1..].split_whitespace().collect();
                    if let Some(command) = args.first() {
                        if *command == "ping" {
                            bot.chat(&format!("Pong, {}!", username));
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
