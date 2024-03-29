// use std::env;    //if you want to save the key in ENV VAR
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{fs::File, io::Read, num::ParseIntError};

pub mod random_call;

const INFO: &str = "```
\t나는 국진봇...
\t꺼져라 닝겐...
```";
const CALL_MSG: &str = "!국진";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == CALL_MSG {
            if let Err(why) = msg.channel_id.say(&ctx.http, INFO).await {
                println!("Error sending message: {:?}", why);
            }
            println!(
                "메세지 수신 {}|{} :{}",
                msg.author.id, msg.author.name, msg.content
            );
        } else if msg.content.starts_with("!알림") {
            let _msg_id = msg.id;
            let text = msg.content.splitn(2, ' ').collect::<Vec<&str>>();
            if text.len() == 1 {
                return;
            }

            let content = text[1].trim();

            if !content.is_empty() {
                let message_text = format!("{} {}", "", content);

                if let Err(why) = msg.delete(&ctx.http).await {
                    println!("Error deleting message: {:?}", why);
                }

                if let Err(why) = msg.channel_id.say(&ctx.http, message_text).await {
                    println!("Error sending message: {:?}", why);
                }

                println!(
                    "메세지 수신 {}|{} :{}",
                    msg.author.id, msg.author.name, msg.content
                );
            }
        } else if msg.content.starts_with("!주사위") {
            use random_call::dice::*;
            let text = msg.content.splitn(2, ' ').collect::<Vec<&str>>();

            if text.len() == 1 {
                let dice_value = get_dice();
                if let Err(why) = msg.channel_id.say(&ctx.http, dice_value).await {
                    println!("Error sending message: {:?}", why);
                }
            } else {
                let num: Result<i32, ParseIntError> = text[1].parse();
                match num {
                    Ok(a) => {
                        let dice_value = get_dice_a(a);
                        if let Err(why) = msg.channel_id.say(&ctx.http, dice_value).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    Err(err) => {
                        println!("Parse Error : {:?}", err);
                    }
                }
            }
            println!(
                "메세지 수신 {}|{} :{}",
                msg.author.id, msg.author.name, msg.content
            );
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

const KEY: &str = "../key.key"; //you need define key
#[tokio::main]
async fn main() {
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment"); //used ENV VAR
    let mut token: String = String::new();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    //GET KEY - you make a 'key' file
    match File::open(KEY) {
        Ok(mut file) => match file.read_to_string(&mut token) {
            Ok(_) => println!("key is OK!"),
            Err(msg) => println!("read error: {}", msg),
        },
        Err(msg) => {
            println!("open error: {}", msg);
            return;
        }
    }

    //make client
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    //client start
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

// use tokio::time::{sleep, Duration};
// #[tokio::main]
// async fn main() {
//     gook::gook_demo(1);
//     let x = sleep(Duration::from_millis(3000));
//     x.await;
//     gook::gook_demo(2);
// }
