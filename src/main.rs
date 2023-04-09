// use std::env;    //if you want to save the key in ENV VAR
use std::{fs::File, io::Read};  
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const INFO: &str = 
"```
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
            println!("메세지 수신 {}|{} :{}",msg.author.id,msg.author.name,msg.content);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


const KEY:&str = "../key.key";  //you need define key
#[tokio::main]
async fn main() {
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment"); //used ENC VAR
    let mut token:String = String::new();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    //GET KEY - you make a 'key' file
    match File::open(KEY) {
        Ok(mut file) => {
            match file.read_to_string(&mut token) {
                Ok(_) => println!("key is OK!"),
                Err(msg) => println!("read error: {}", msg),
            }
        },
        Err(msg) => println!("open error: {}", msg),
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