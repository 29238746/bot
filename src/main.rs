use azalea::prelude::*;
use anyhow::Result;

const PASSWORD: &str = "Bot#12345";   // ← CHANGE THIS to your server's password

#[tokio::main]
async fn main() -> Result<()> {
    let account = Account::offline("zenix_czosnikowy");  // ← CHANGE bot name
    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "renciarze.aternos.me")                // ← CHANGE to your server IP
        .await?;
    Ok(())
}

#[derive(Default, Clone, Component)]
pub struct State;

async fn handle(bot: Client, event: Event, _state: State) -> Result<()> {
    match event {
        Event::Chat(m) => {
            let message = m.message().to_string();
            let username = m.username();
            let msg_lower = message.to_ascii_lowercase();

            // Auto‑register / login (if the server asks)
            if msg_lower.contains("/register") {
                bot.chat(&format!("/register {} {}", PASSWORD, PASSWORD))?;
            } else if msg_lower.contains("/login") {
                bot.chat(&format!("/login {}", PASSWORD))?;
            }

            // Only respond to !ping (all other !commands are ignored)
            if username != bot.username() && message.starts_with('!') {
                let args: Vec<&str> = message[1..].split_whitespace().collect();
                if let Some(command) = args.first() {
                    if *command == "ping" {
                        bot.chat(&format!("Pong, {}!", username))?;
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
