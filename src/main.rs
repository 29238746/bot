use azalea::prelude::*;
use azalea::prelude::AppExit;
use std::io::Write;

const PASSWORD: &str = "Botczosnikowy345";

#[tokio::main]
async fn main() -> AppExit {
    let _ = std::io::stderr().flush();
    eprintln!("[1] Bot starting...");
    let _ = std::io::stderr().flush();

    let account = Account::offline("zenix_czosnikowy");
    eprintln!("[2] Account created");
    let _ = std::io::stderr().flush();

    eprintln!("[3] Connecting to renciarze.aternos.me:16628...");
    let _ = std::io::stderr().flush();

    let result = ClientBuilder::new()
        .set_handler(handle)
        .version("26.1.1")  // 👈 ADD THIS LINE – match your server's version
        .start(account, "renciarze.aternos.me:16628")
        .await;

    eprintln!("[4] Connection finished. Result: {:?}", result);
    let _ = std::io::stderr().flush();

    result
}

#[derive(Default, Clone, Component)]
pub struct State;

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Connect => {
            eprintln!("[EVENT] Connected to server!");
            let _ = std::io::stderr().flush();
        }
        Event::Spawn => {
            eprintln!("[EVENT] Bot spawned in the world!");
            let _ = std::io::stderr().flush();
        }
        Event::Disconnect(disconnect_packet) => {
            eprintln!("[EVENT] Disconnected: {}", disconnect_packet.reason);
            let _ = std::io::stderr().flush();
        }
        Event::Chat(chat_packet) => {
            let (username_opt, message) = chat_packet.split_sender_and_content();
            let msg_lower = message.to_ascii_lowercase();

            if let Some(username) = &username_opt {
                eprintln!("[CHAT] <{}> {}", username, message);
            } else {
                eprintln!("[CHAT] (system) {}", message);
            }
            let _ = std::io::stderr().flush();

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
