use anyhow::Result;

use crate::config::limavel_config::LimavelConfig;
use crate::error::LimavelError;
use crate::lima::client::LimaClient;

pub fn execute(name: &str) -> Result<()> {
    LimaClient::check_installed()?;
    let config = LimavelConfig::load(name)?;
    let instance = config.instance_name();

    if !LimaClient::instance_exists(instance)? {
        return Err(LimavelError::InstanceNotFound(instance.to_string()).into());
    }

    let was_running = LimaClient::instance_status(instance)? == "Running";
    if was_running {
        LimaClient::stop(instance)?;
    }

    let current_disk_gib = LimaClient::instance_disk_gib(instance)?;
    let new_disk = if config.disk > current_disk_gib {
        Some(config.disk)
    } else {
        None
    };

    LimaClient::edit(instance, config.cpus, config.memory, new_disk)?;

    if was_running {
        LimaClient::start(instance)?;
    }

    Ok(())
}
