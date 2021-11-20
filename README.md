[![Build status](https://github.com/MaFeLP/discord_bots/actions/workflows/rolling-release.yml/badge.svg)](https://github.com/MaFeLP/discord_bots/actions)
[![dependency status](https://deps.rs/repo/github/mafelp/discord_bots/status.svg)](https://deps.rs/repo/github/mafelp/discord_bots)
[![Issues](https://img.shields.io/github/issues/mafelp/discord_bots)](https://github.com/MaFeLP/discord_bots/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/mafelp/discord_bots)](https://github.com/MaFeLP/discord_bots/pulls/)
[![Bots](https://img.shields.io/badge/Bots-2-informational)]()
<br>
[![GitHub](https://img.shields.io/github/license/mafelp/discord_bots)](https://www.gnu.org/licenses/gpl-3.0.html)
[![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/mafelp/discord_bots)](https://github.com/MaFeLP/discord_bots/releases/)

# Discord Bots
This repository holds two discord bots written in [rust](https://www.rust-lang.org/):

- Autokommentator
- Känguru Knecht

They are inspired by [u/Kaenguru_Knecht](https://www.reddit.com/user/Kaenguru_Knecht)
and [u/AutoKommentator](https://www.reddit.com/user/AutoKommentator) (both bots of
[r/ich_iel](https://www.reddit.com/r/ich_iel/)).

## Running
This project is currently in its beta phase, so you need to compile it for yourself.

Then you can choose of a few solution:

- [docker](#running-with-docker),
- [docker-compose](#running-with-docker-compose) or
- [native](#running-native)

### Running with docker
1. Install [docker](https://www.docker.com/get-started)
2. Get the source code for this repository: `git clone https://github.com/mafelp/discord_bots.git`
3. Build the docker image: `DOCKER_BUILDKIT=1 docker build . -t discord_bots`
4. Create the env file:
   1. Rename `.env.example` to `.env`.
   2. Edit `.env` and enter behind the equals sign your [bot tokens](#bot-tokens) for your two bots
5. Run the docker image:

```shell
docker run -it --rm \
    --name xdbot \
    --env-file .env \
    xdbot
```

### Running with docker-compose
For both variants, you need to install [docker](https://www.docker.com/get-started)
and [docker-compose](https://docs.docker.com/compose/install/). You can then choose,
how to run and build your container. The recommended version, is to build and run
using the `Dockerfile`, see [recommended method](#building-inside-of-docker-recommended). If you want to build for another,
platform for example the Raspberry Pi, use the ["not recommended method"](#native-docker-compose-not-recommended).
There you build outside of docker for your or another platform using [`cross`](https://github.com/rust-embedded/cross)
and then just running the binary inside a container.

#### Building inside of docker (recommended)
1. Create the env file:
    1. Rename `.env.example` to `.env`.
    2. Edit `.env` and enter behind the equals sign your [bot tokens](#bot-tokens) for your two bots
2. Rename `docker-compose.yml.example` to `docker-compose.yml`.
3. Edit `docker-compose.yml` to you likings. **You do not _have_ to edit it because it already works,
   but if you have to change something, you can do so here.**
4. Run the following command to start the docker container

```shell
DOCKER_BUILDKIT=1 docker-compose up
```

If you want to have your container running in the background, run:

```shell
DOCKER_BUILDKIT=1 docker-compose up -d
```

To stop the container, simply run the following command:

```shell
docker-compose down
```

#### Native docker-compose (not recommended)
1. Create the env file:
    1. Rename `.env.example` to `.env`.
    2. Edit `.env` and enter behind the equals sign your [bot tokens](#bot-tokens) for your two bots
2. Rename `native.docker-compose.yml.example` to `docker-compose.yml`.
3. Build the binary:
   - If you are cross compiling for another platform:
       1. Install the [rust development tools](https://www.rust-lang.org/learn/get-started)
       2. Install [cross](https://github.com/rust-embedded/cross) using cargo: `cargo install cross`
       3. Build the binary for the target platform with `~/.cargo/bin/cross build --release --target TARGET` on linux
         or `%USERHOME%\.cargo\bin\cross build --release --target TARGET` on Windows. Replace `TARGET` with the target
         you are compiling for. See targets below.
            - Raspberry Pi 4: `aarch64-unknown-linux-gnu`
            - Linux x86: `x86_64-unknown-linux-gnu`
            - Windows x86: `x86_64-pc-windows-gnu`
   - If you are planning to run on the current platform:
       1. Install the [rust development tools](https://www.rust-lang.org/learn/get-started)
       2. Run `cargo build --release`
4. Edit `docker-compose.yml`:
    1. Change `image: debian:buster-slim` to the platform your binary has been built for.
        - For a Raspberry Pi running Ubuntu Server 20.04 use: `ubuntu:20.04`
        - For debian use: `debian:buster-slim`
        - For Archlinux (based systems) use: `archlinux`
        - ...
    2. **ONLY IF YOU WERE CROSS COMPILING IN STEP 3**: In `volumes:` change `./target/release/:/app/` to
      `./target/TARGET/release/:/app/` where `TARGET` is the target you built for in step 3. Or if you are on another
      system, change this to `PATH/TO/THE/BINARY:/app/xd_bot`.
5. Run the following command to start the docker container

```shell
docker-compose up
```

If you want to have your container running in the background, run:

```shell
docker-compose up -d
```

To stop the container, simply run the following command:

```shell
docker-compose down
```


### Running native
1. Install the [rust development tools](https://www.rust-lang.org/learn/get-started)
2. Run `cargo build --release`
3. Run the following command to start the bots. Replace `YOUR KAENGURU BOT TOKEN` and `YOUR XD BOT TOKEN` with the
   tokens used by your bots to authenticate to discord. See [bot tokens](#bot-tokens) for more information.

On Linux and MacOS:

```shell
DISCORD_TOKEN_KAENGURU="YOUR KAENGURU BOT TOKEN" \
DISCORD_TOKE_XD="YOUR XD BOT TOKEN" \
./target/xd_bot
```

On Windows (CMD):
```cmd
set DISCORD_TOKEN_KAENGURU="YOUR KAENGURU BOT TOKEN"
set DISCORD_TOKE_XD="YOUR XD BOT TOKEN"
.\target\xd_bot
```

## Bot tokens
1. Go to [https://discord.com/developers/applications/](https://discord.com/developers/applications).
2. In the top right corner, click on `New Application`.
![Step 1](./assets/token/1.png)
3. Give the bot a name and click on `Create`.
![Step 2](./assets/token/2.png) ![Step 3](./assets/token/3.png)
4. In the left side bar, click on `Bot`.
![Step 4](./assets/token/4.png)
5. In the top right corner, click on `Add bot`.
![Step 5](./assets/token/5.png)
6. Confirm your actions with `Yes, do it!`.
![Step 6](./assets/token/6.png)
7. Now Copy your Discord Bot Token by either clicking on `copy`:
![Step 7 - Copy button](./assets/token/7.png)
or clicking on `Click to Reveal Token` and then mark your Token and copy it.
![Step 7 - Revealing the Token](./assets/token/8-1.png) ![Step 7 - copy the token](./assets/token/8-2.png)

## Invite the bots
1. Copy and paste `The bot invitation token` from the console into your webbrowser and go to this website. The link should start with `https://discord.com/oauth2/authorize?client_id=`.
2. Click on the Pop-Out-Menu `Select a Server` <br>
![Open Select a Server Menu](./assets/invite/1.png)
3. From the list, selct the Server you want to invite the bot to. If the specific server does not appear there, make sure you have the required permissions to invite and manage bots to the server. <br>
![Select the Server from Drop-down-List](./assets/invite/2.png)
4. After you selcted the server, click on `Authorize`. <br>
![Click on Authorize.](./assets/invite/3.png)
5. Solve the Captcha by clicking the square left to `I am human` and selecting the fitting images by clicking on them (not everyone has to solve these captchas! Don't be worried, if you don't have to solve them.) <br>
![Solve the Captcha](./assets/invite/4.png)
6. You should now see a green tick that says `Authorized`. <br>
![Authorized](./assets/invite/5.png)
7. Your bot should now appear on your server: <br>
![Bot appeared](./assets/invite/6.png)