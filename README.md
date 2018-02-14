# sushii bot
[![Build Status](https://travis-ci.org/drklee3/sushii-bot.svg?branch=master)](https://travis-ci.org/drklee3/sushii-bot)

A [Discord](https://discordapp.com) bot written in [Rust](https://www.rust-lang.org/) with [serenity-rs](https://github.com/zeyla/serenity).  Uses a [PostgreSQL](https://www.postgresql.org) database along with [diesel-rs](https://github.com/diesel-rs/diesel) and [r2d2-diesel](https://github.com/diesel-rs/r2d2-diesel).

Work in progress.  Features may be either missing, incomplete, or broken.

# Features
* Ranking system based on message counts in daily, weekly, monthly, and all time categories
* User 24 hour activity tracker
* Profile image generation for displaying rank and activity graph (with [sushii-image-server](https://github.com/drklee3/sushii-image-server))
* Configurable self assigning role system with multiple categories and limits
* Configurable prefix per guild
* Moderation action logs and editable action reasons
* Mute evasion prevention
* Mass mention auto-mutes
* User created tags (custom commands-ish)
* Channel galleries (sends links & images from a channel to a webhook)
* Reminders
* Keyword notifications
* User join and leave messages
* Rust playground code execution
* Discord events counter
* ...and more to be added

# Installation
1. Download the latest version from the [releases](releases) page, currently only supporting x86_64-unknown-linux-gnu.
2. Allow the file to be executed.
    ```bash
    $ chmod +x x86_64-unknown-linux-gnu
    ```
3. Create an `.env` file in the same directory and update according to [`.env.example`](.env.example).  All variables must exist except for `BLOCKED_USERS` or the bot will panic.

4. Run with `./x86_64-unknown-linux-gnu` or with a process manager like [Supervisor](http://supervisord.org).

# Building from Source

1. Install dependencies.
    * [PostgreSQL](https://www.postgresql.org) (9.4+)
    * [Rust / Cargo](http://doc.crates.io)
        ```bash
        $ curl -sSf https://static.rust-lang.org/rustup.sh | sh
        ```
    * [sushii-image-server](https://github.com/drklee3/sushii-image-server) (Used for rank image generation, etc)
        1. Install [Node.js and npm](https://nodejs.org/en/download/package-manager/)
            ```bash
            $ curl -sL https://deb.nodesource.com/setup_8.x | sudo -E bash -
            $ sudo apt-get install -y nodejs
            ```
        2. Clone repository and enter the directory.
            ```bash
            $ git clone https://github.com/drklee3/sushii-image-server.git
            $ cd sushii-image-server
            ```
        3. Install sushii-image-server dependencies.
            ```bash
            $ npm install
            ```
        4. Install [chromium dependencies](https://github.com/GoogleChrome/puppeteer/blob/master/docs/troubleshooting.md#chrome-headless-doesnt-launch).
            ```bash
            $ sudo apt-get install -y gconf-service libasound2 libatk1.0-0 libc6 libcairo2 libcups2 libdbus-1-3 libexpat1 libfontconfig1 libgcc1 libgconf-2-4 libgdk-pixbuf2.0-0 libglib2.0-0 libgtk-3-0 libnspr4 libpango-1.0-0 libpangocairo-1.0-0 libstdc++6 libx11-6 libx11-xcb1 libxcb1 libxcomposite1 libxcursor1 libxdamage1 libxext6 libxfixes3 libxi6 libxrandr2 libxrender1 libxss1 libxtst6 ca-certificates fonts-liberation libappindicator1 libnss3 lsb-release xdg-utils wget
            ```
        5. Start with `npm start` or with a process manager like [PM2](https://github.com/Unitech/pm2)
2. Clone this repository and enter the directory.
    ```bash
    $ git clone https://github.com/drklee3/discord-sushii.git
    $ cd discord-sushii
    ```
3. Edit [`.env.example`](.env.example) and rename to `.env`.  Removing any key or leaving them blank will result in panics.
4. Build and run the bot.
    ```bash
    $ cargo run --release
    ```
