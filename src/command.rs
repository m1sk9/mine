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

    let status = if res.online {
        "オンライン"
    } else {
        "オフライン"
    };

    let embed = CreateEmbed::default()
        .title("サーバー情報")
        .description(&env.server_name)
        .field("ステータス", status, true)
        .field("バージョン", res.version, true)
        .field(
            "プレイヤー",
            format!("**{}**/{}", res.players.online, res.players.max),
            true,
        )
        .footer(CreateEmbedFooter::new(
            "プレイヤーリストは /playerlist で確認できます",
        ))
        .color(0x27BD59);
    let reply = CreateReply::default().embed(embed);

    poise::send_reply(ctx, reply).await?;

    Ok(())
}

/// サーバーのプレイヤーリストを表示します
#[poise::command(prefix_command, slash_command)]
pub async fn playerlist(
    ctx: crate::Context<'_>,
    #[description = "UUID を表示するかどうか"] is_show_uuid: Option<bool>,
) -> anyhow::Result<(), anyhow::Error> {
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
        poise::say_reply(ctx, "サーバーはオフラインです").await?;
        return Ok(());
    }

    if res.players.online == 0 {
        poise::say_reply(ctx, "現在プレイヤーはいません").await?;
        return Ok(());
    }

    // Note: `-#` を使うと Discord では文字を小さく表示できる
    match is_show_uuid {
        Some(c) if c => {
            let uuids = res
                .players
                .list
                .iter()
                .map(|p| p.uuid.clone())
                .collect::<Vec<String>>()
                .join(", ");
            let result = format!("{}\n-# 現在 {} 人がプレイ中です", uuids, res.players.online);
            poise::say_reply(ctx, result).await?;
        }
        _ => {
            let players = res
                .players
                .list
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>()
                .join(", ");
            let result = format!(
                "{}\n-# 現在 {} 人がプレイ中です",
                players, res.players.online
            );
            poise::say_reply(ctx, result).await?;
        }
    }

    Ok(())
}
