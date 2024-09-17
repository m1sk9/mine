use anyhow::Context;
use poise::{
    serenity_prelude::{CreateEmbed, CreateEmbedFooter},
    CreateReply,
};

use crate::model::StatusResponse;

/// サーバーのステータスを表示します
#[poise::command(prefix_command, slash_command)]
pub async fn status(ctx: crate::Context<'_>) -> anyhow::Result<(), anyhow::Error> {
    let env = crate::env::load_envs();

    let url = match &env.server_port {
        Some(port) => format!("https://api.mcsrvstat.us/3/{}:{}", env.server_ip, port),
        None => format!("https://api.mcsrvstat.us/3/{}", env.server_ip),
    };
    let res = reqwest::get(url)
        .await?
        .json::<StatusResponse>()
        .await
        .context("Failed to get server status")?;

    if !res.online {
        poise::send_reply(
            ctx,
            CreateReply::default()
                .content("サーバーはオフラインです")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let embed = CreateEmbed::default()
        .title("サーバー情報")
        .description(&env.server_name)
        // オフラインだったらそもそも見ない
        .field("ステータス", "オンライン", true)
        .field("バージョン", res.version.unwrap_or_default(), true)
        .field(
            "プレイヤー",
            format!("**{}**/{}", res.players.online, res.players.max),
            true,
        )
        .footer(CreateEmbedFooter::new(
            "プレイヤーリストは /playerlist で確認できます",
        ))
        .color(0x27BD59);
    let reply = CreateReply::default().embed(embed).ephemeral(true);

    poise::send_reply(ctx, reply).await?;

    Ok(())
}

/// サーバーのプレイヤーリストを表示します
#[poise::command(prefix_command, slash_command)]
pub async fn playerlist(ctx: crate::Context<'_>) -> anyhow::Result<(), anyhow::Error> {
    let env = crate::env::load_envs();

    let url = match &env.server_port {
        Some(port) => format!("https://api.mcsrvstat.us/3/{}:{}", env.server_ip, port),
        None => format!("https://api.mcsrvstat.us/3/{}", env.server_ip),
    };
    let res = reqwest::get(url)
        .await?
        .json::<StatusResponse>()
        .await
        .context("Failed to get server status")?;

    if !res.online {
        poise::send_reply(
            ctx,
            CreateReply::default()
                .content("サーバーはオフラインです")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    if res.players.online == 0 {
        poise::send_reply(
            ctx,
            CreateReply::default()
                .content("現在プレイヤーはいません")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let players = res
        .players
        .list
        .unwrap_or_default()
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>()
        .join(", ");
    let reply = CreateReply::default()
        .content(format!(
            "{}\n-# 現在 {} 人がプレイ中です",
            players, res.players.online
        ))
        .ephemeral(true);

    poise::send_reply(ctx, reply).await?;

    Ok(())
}
