# Mine

Minecraft サーバーの情報を Discord で確認できるようにする Bot です.

## Usage

- [`compose.yaml`](./compose.yaml) を配置しているリポジトリ内に `.env` ファイルを用意し, 環境変数を設定してください.
- 設定後 `docker compose up -d` で起動できます.

## Environment Variables

| Name                | Description                          |
|---------------------|--------------------------------------|
| `DISCORD_API_TOKEN` | Discord Bot のトークン                    |
| `DISCORD_GUILD_ID`  | Discord サーバーの ID                  |
| `SERVER_NAME`       | サーバー名                             |
| `SERVER_IP`         | サーバーの IP アドレス                  |
| `SERVER_PORT`       | サーバーのポート番号[^1]           |

[^1]: ポートを公開していないサーバーの場合は指定しなくても検索できます.
