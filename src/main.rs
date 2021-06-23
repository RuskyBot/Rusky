use rusky::{async_main, async_run, config::RuskyConfig, events::Handler, rusky::Rusky, setup};

use log::{error, info};

async_main!({
    async_run! {{
        setup!();
        info!("Rusky a simple bot for Discord!");
        info!("Loading configuration...");
        let config = RuskyConfig::load("Rusky.toml")?;

        info!("Starting Rusky with {} shards...", config.discord.shard_amount);
        let mut rusky = Rusky::new(&config, Handler).await?;

        rusky.login(config.discord.shard_amount).await?;

    } catch err => {
        error!("Rusky error: {:?}", err);
    }};
});
