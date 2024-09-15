use anyhow::Ok;
use poise::{
    samples::create_application_commands,
    serenity_prelude::{self, FullEvent, GatewayIntents, GuildId},
    FrameworkError,
};

mod command;
mod env;
mod model;

struct Data {}
type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().compact().init();

    dotenvy::dotenv().ok();

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;
    let framework_options = poise::FrameworkOptions {
        commands: vec![command::status(), command::playerlist()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("s!".to_string()),
            ..Default::default()
        },
        pre_command: |ctx| {
            Box::pin(async move {
                tracing::info!(
                    "{} が /{} を実行しました",
                    ctx.author().name,
                    ctx.command().qualified_name,
                );
            })
        },
        on_error: |err| {
            Box::pin(async move {
                if let FrameworkError::Command { error, ctx, .. } = err {
                    let message = format!(
                        "/{} を実行時にエラーが発生しました: {:?}",
                        ctx.command().qualified_name,
                        error
                    );
                    tracing::error!("{}", message);
                    let _ = ctx.say(message).await;
                }
            })
        },
        event_handler: |ctx, event, framework, _| {
            Box::pin(async move {
                if let FullEvent::Ready { .. } = event {
                    let commands = create_application_commands(&framework.options().commands);
                    GuildId::new(env::load_envs().discord_guild_id)
                        .set_commands(&ctx.http, commands)
                        .await?;
                    tracing::info!("Ready!");
                }
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework: poise::Framework<Data, anyhow::Error> = poise::Framework::builder()
        .options(framework_options)
        .setup(move |_, _, _| Box::pin(async move { Ok(Data {}) }))
        .build();

    let mut client =
        serenity_prelude::ClientBuilder::new(&env::load_envs().discord_api_token, intents)
            .framework(framework)
            .await
            .expect("Failed to create a new client.");

    if let Err(why) = client.start().await {
        Err(anyhow::anyhow!("Failed to start the client: {}", why))?;
    }

    Ok(())
}
