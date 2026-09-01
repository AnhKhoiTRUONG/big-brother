## About
Big Brother is watching your docker container images for available updates.

It automatically compares the cryptographic digests (SHA256) of your local Docker images against the remote registries to check for upstream changes so you don't have to track image updates manually.

## Installation and usage
```bash
git clone
cd big-brother

### Run the project
cargo run
```

## Features
- Global Scan: Automatically checks every container running on your local machine.
- Digest Comparison: Pulls remote manifests to accurately compare local and upstream digests.
- Docker Hub Support: Fully supports public images hosted on Docker Hub.
## Coming soon
- Multi-Registry Support: GitHub Container Registry (ghcr.io) integration.
- Web Dashboard: A lightweight local UI to track your container images visually.
- Notifications: Automated push notifications via Discord, Mail, and more.
- Scheduling: Run periodic background checks on a schedule (cron).
- Configuration file for each service

## Prerequisites
- [Rust and Cargo](https://rust-lang.org/tools/install/) installed

### Docker Socket Permissions

Big Brother uses the `/var/run/docker.sock` to scan your local containers. Ensure that your user is part of the docker group:
```bash
# Add your user to the docker group
sudo usermod -aG docker $USER

# Apply the new group (or log out and log back in)
newgrp docker
```
```

```
