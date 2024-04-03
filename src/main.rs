use std::{collections::HashMap, env};

use a2s::A2SClient;
use dotenvy::dotenv;
use serenity::{
    all::{Context, EventHandler, GatewayIntents, Ready},
    async_trait, Client,
};
use tabled::{
    settings::{object::Rows, Alignment, Modify, Style},
    Table, Tabled,
};

struct Handler;
struct PData;

#[derive(Tabled)]
struct PlayerField {
    name: String,
    score: i32,
    duration: String,
}

type PError = Box<dyn std::error::Error + Send + Sync>;
type PContext<'a> = poise::Context<'a, PData, PError>;

impl PlayerField {
    fn new(name: String, score: i32, duration: String) -> Self {
        Self {
            name: name.to_owned(),
            score: score.to_owned(),
            duration: duration.to_owned(),
        }
    }
}

#[poise::command(slash_command)]
async fn server(
    ctx: PContext<'_>,
    #[description = "Server's Code Name to retrieve information about."]
    #[choices(
        "S.O.S", "S.O.S II", "A.C.E", "A.C.E II", "F.B.I", "I.C.E", "F.U.N", "F.U.N II"
    )]
    choice: &'static str,
) -> Result<(), PError> {
    let client = A2SClient::new().unwrap();

    // ! that looks very fucked up
    let mut server_list: HashMap<&str, &str> = HashMap::new();
    server_list.insert("S.O.S", "128.140.90.181:27017");
    server_list.insert("S.O.S II", "128.140.90.181:27024");
    server_list.insert("A.C.E", "128.140.90.181:27018");
    server_list.insert("A.C.E II", "128.140.90.181:27025");
    server_list.insert("F.B.I", "128.140.90.181:27019");
    server_list.insert("I.C.E II", "128.140.90.181:27022");
    server_list.insert("F.U.N", "128.140.90.181:27021");
    server_list.insert("F.U.N II", "128.140.90.181:27023");

    let address = server_list.get(choice).unwrap();
    let mut data = Vec::new();

    let info = client.info(address).unwrap();
    let mut players = client.players(address).unwrap();

    // weirdest shit i've ever written
    while let Some(player) = players.pop() {
        let seconds = (player.duration % 60.0).floor();
        let minutes = ((player.duration / 60.0) % 60.0).floor();
        let hours = ((player.duration / 60.0) / 60.0).floor();

        data.push(PlayerField::new(
            player.name,
            player.score,
            format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}"),
        ));
    }

    let mut table = Table::new(data);
    table
        .with(Style::markdown())
        .with(Modify::new(Rows::first()).with(Alignment::center()));

    let response = format!(
        "```\nName: {}\nAddress: {}\nMap: {}\n\n{}```",
        info.name, address, info.map, table
    );

    let reply = poise::CreateReply {
        ephemeral: Some(true),
        content: Some(response),
        ..Default::default()
    };

    ctx.send(reply).await?;

    Ok(())
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, event: Ready) {
        println!("{} xuxu'ing all over a place", event.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("'.env' file couldn't be found (wtf?)");

    let token =
        env::var("DISCORD_BOT_TOKEN").expect("'DISCORD_BOT_TOKEN' was not defined in '.env'");
    let intents = GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![server()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(PData {})
            })
        })
        .build();

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Client couldn't be created.");

    client.start().await.expect("xuxu");
}
