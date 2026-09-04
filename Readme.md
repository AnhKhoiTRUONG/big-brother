## About
Big Brother is watching your docker container images for available updates.

It automatically compares the cryptographic digests (SHA256) of your local Docker images against the remote registries to check for upstream changes so you don't have to track image updates manually.

This project is intended to be used in homelabs, media centers, local dev environments, and similar. I do not recommend using Big Brother in a commercial or production environment



## Installation and usage
### Prerequisites
- [Rust and Cargo](https://rust-lang.org/tools/install/) installed
### Docker Socket Permissions
Big Brother uses the `/var/run/docker.sock` to scan your local containers. Ensure that your user is part of the docker group:
```bash
# Add your user to the docker group
sudo usermod -aG docker $USER

# Apply the new group (or log out and log back in)
newgrp docker
```

```bash
git clone https://github.com/AnhKhoiTRUONG/big-brother.git
cd big-brother
### Run the project
cargo run
```
### Scheduling
We can schedule Big-Brother to check update by configurate the `config.yaml`. The format will be
```yaml
# This means check every 6h
watch:
  schedule: "* */6 * * * *"
  timezone: Europe/Paris
```
The cron format is:
```conf
sec   min   hour   day of month   month   day of week
*     *     *      *              *       *
```
If the `config.yaml` is not provided, the default config like above will be used

## Features
- Global Scan: Automatically checks every container running on your local machine.
- Digest Comparison: Pulls remote manifests to accurately compare local and upstream digests.
- Now fully support every registry.
- Scheduling cron job via `config.yaml` file
## Coming soon
- Web Dashboard: A lightweight local UI to track your container images visually.
- Notifications: Automated push notifications via Discord, Mail, and more.
- Configuration file for each service
- Integration into Docker with Dockerfile

