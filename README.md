# Chrozone
![Chrozone Logo](./docs/LOGO.png)

Chrozone is a [Discord bot][invite] which provides epoch- and other time-related utilities via slash commands.

[invite]: https://discord.com/oauth2/authorize?client_id=1008989318901137459

# Running the Bot
Chrozone is written in [Rust]. It uses the built-in [Cargo] package manager to build the project and its dependencies. Some automation scripts are also written in TypeScript, which requires the [Deno] runtime. Before proceeding, please ensure that the appropriate toolchains have been installed.

[Rust]: https://www.rust-lang.org
[Cargo]: https://doc.rust-lang.org/cargo
[Deno]: https://deno.land

We must now register Chrozone's available slash commands. A dedicated [Deno] script automates this process for us. To invoke the script, the host must provide some credentials (obtained from the [Discord Developer Portal]) via the environment variables below.

Required? | Name | Category | Description
:-------: | ---- | -------- | -----------
&#x2714; | `APP_ID` | Discord | Sets the application ID to be used for authentication with the Discord API. [^portal]
&#x2714; | `TOKEN` | Discord | Sets the bot token to be used for authentication with the Discord API.[^portal]
&#x274c; | `GUILD_ID` | Discord | Sets whether we must register as guild commands (if present) or global commands (otherwise).

[Discord Developer Portal]: https://discord.com/developers/applications
[^portal]: May be retrieved from the application page. See the [Discord Developer Portal].

```bash
# Set required environment variables.
APP_ID=
BOT_TOKEN=
GUILD_ID=

# Ensure that the slash commands are registered beforehand.
deno run --allow-net --allow-env scripts/commands.ts
```

Once the commands have been registered, the executable then expects additional environment variables to be present before it initializes the server.

Required? | Name | Category | Description
:-------: | ---- | -------- | -----------
&#x2714; | `PORT` | Network | Configures the port at which we will bind the server's TCP socket.
&#x2714; | `PUB_KEY` | Discord | Sets the public key of the bot.[^portal] Must contain 64 hexadecimal characters. Used for validating webhooks from Discord.

[^id]: May be retrieved from the command registration script's output.

```bash
# Set required environment variables.
PORT=
PUB_KEY=

# Builds and runs the executable. The server binds
# to a TCP socket address `0.0.0.0` at some `PORT`.
cargo run --release
```
