use anyhow::Result;
use colored::Colorize;

use crate::config::limavel_config::LimavelConfig;
use crate::hosts;
use crate::lima::client::LimaClient;
use crate::ansible::runner;

pub fn execute(name: &str) -> Result<()> {
    LimaClient::check_installed()?;

    let config = LimavelConfig::load(name)?;
    let instance = config.instance_name();
    LimaClient::ensure_running(instance)?;

    println!("{} Running provisioning...", "→".cyan());
    runner::provision(instance, &config)?;
    println!("{} Provisioning complete!", "✓".green());

    // Refresh /etc/hosts in case sites changed
    let ip = LimaClient::guest_ip(instance)?;
    let domains: Vec<String> = config.sites.iter().map(|s| s.map.clone()).collect();
    if !domains.is_empty() {
        println!("{} Updating /etc/hosts ({})...", "→".cyan(), ip);
        hosts::update(instance, &ip, &domains)?;
        println!("{} /etc/hosts updated.", "✓".green());
    }

    Ok(())
}
