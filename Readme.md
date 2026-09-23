## About
Big Brother is watching your docker container images for available updates.

It automatically compares the cryptographic digests (SHA256) of your local Docker images against the remote registries to check for upstream changes so you don't have to track image updates manually.

This project is intended to be used in homelabs, media centers, local dev environments, and similar. I do not recommend using Big Brother in a commercial or production environment

This project is inspired by [Diun](https://github.com/crazy-max/diun)

## Installation and usage
### Docker compose
```yml
services:
  docker_notif:
    build: .
    image: anhkhoitruong/big-brother:latest
    container_name: big-brother
    restart: unless-stopped
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      # Mounts the local config.yaml into the container at /app/config.yaml
      - ./config.yaml:/app/config.yaml:ro
```
### Scheduling
We can schedule Big-Brother to check update by configurate the `config.yaml`. The format will be
```yaml
# This means check every 6h
watch:
  schedule: "* * */6 * * *"
  timezone: Europe/Paris
```
The cron format is:
```conf
sec   min   hour   day of month   month   day of week
*     *     *      *              *       *
```
If the `config.yaml` is not provided, the default config like above will be used
### Discord notification
More details for how to get discord webhook url [here](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks)

To have discord notification, we can add these to the `config.yaml` file
```yaml
discord:
  webhook_url: "Your webhook url"
```
## Features
- Global Scan: Automatically checks every container running on your local machine.
- Digest Comparison: Pulls remote manifests to accurately compare local and upstream digests.
- Now fully support every registry.
- Scheduling cron job via `config.yaml` file
- Notifications via Discord
## Coming soon
- Web Dashboard: A lightweight local UI to track your container images visually.
- Notifications: Automated push notifications via Mail, and more.
- Configuration file for each service
- Integration into Docker with Dockerfile

