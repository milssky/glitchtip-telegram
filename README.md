# Glitchtip Telegram

HTTP webhook relay that receives [Glitchtip](https://glitchtip.com/)
notifications and forwards them to Telegram.

## Glitchtip webhook setup

In Glitchtip, when adding an alert rule or notification integration,
choose **General (slack-compatible) Webhook** as the webhook type.
Use this service’s URL as the webhook endpoint.

## Running (Docker)

```bash
docker build -t glitchtip-telegram .
docker run -d \
  -p 8000:8000 \
  -e BOT_TOKEN=YOUR_BOT_TOKEN \
  -e CHAT_ID=YOUR_CHAT_ID \
  -e PORT=8000 \
  -e WEBHOOK=secret-webhook \  # Optional
  --name glitchtip-telegram \
  glitchtip-telegram
```

The service listens on port 8000. Environment variables: `BOT_TOKEN`, `CHAT_ID`, `PORT`, `WEBHOOK`.
`WEBHOOK` env are optional. By default `WEBHOOK=/webhook`.

## Releases

When a release is published (or the release workflow is run manually),
you can download ready-to-use binaries from the release page assets.

Docker image is published to:

- `ghcr.io/<owner>/<repo>:<tag>`
- `ghcr.io/<owner>/<repo>:latest`

Example for this repository:

```bash
docker run -d \
  -p 8000:8000 \
  -e BOT_TOKEN=YOUR_BOT_TOKEN \
  -e CHAT_ID=YOUR_CHAT_ID \
  -e PORT=8000 \
  -e WEBHOOK=secret-webhook \
  --name glitchtip-telegram \
  ghcr.io/kondratevdev/glitchtip-telegram:latest
```

## Configuration

| Option        | Env var     | Description                                      |
|---------------|-------------|--------------------------------------------------|
| `--bot-token` | `BOT_TOKEN` | Telegram Bot API token                           |
| `--chat-id`   | `CHAT_ID`   | Target chat or channel ID for notifications      |
| `--port`      | `PORT`      | HTTP server port (e.g. 8000 in Docker)           |
| `--webhook`   | `WEBHOOK`   | Local server webhook path
