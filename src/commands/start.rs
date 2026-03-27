use anyhow::Result;
use colored::Colorize;
use tempfile::NamedTempFile;
use std::io::Write;

use crate::config::limavel_config::LimavelConfig;
use crate::config::lima_config::LimaConfig;
use crate::hosts;
use crate::lima::client::LimaClient;
use crate::ansible::runner;

fn update_etc_hosts(instance: &str, config: &LimavelConfig) -> Result<()> {
    let ip = LimaClient::guest_ip(instance)?;
    let domains: Vec<String> = config.sites.iter().map(|s| s.map.clone()).collect();
    if !domains.is_empty() {
        println!("{} Updating /etc/hosts ({})...", "→".cyan(), ip);
        hosts::update(instance, &ip, &domains)?;
        println!("{} /etc/hosts updated.", "✓".green());
    }
    Ok(())
}

pub fn execute(name: &str, no_hosts: bool) -> Result<()> {
    LimaClient::check_installed()?;

    let config = LimavelConfig::load(name)?;
    config.validate_folders()?;
    let instance = config.instance_name();

    if LimaClient::instance_exists(instance)? {
        let status = LimaClient::instance_status(instance)?;
        if status == "Running" {
            println!("{} VM '{}' is already running.", "ℹ".cyan(), instance);
            return Ok(());
        }

        println!("{} Starting VM '{}'...", "→".cyan(), instance);
        LimaClient::start(instance)?;
        println!("{} VM '{}' started.", "✓".green(), instance);
        if !no_hosts {
            update_etc_hosts(instance, &config)?;
        }
        return Ok(());
    }

    // Create new instance
    println!("{} Creating VM '{}'...", "→".cyan(), instance);

    let ssh_pubkey = config.read_ssh_pubkey()?;
    let lima_config = LimaConfig::from_config(&config, &ssh_pubkey);
    let yaml = lima_config.to_yaml()?;

    let mut tmpfile = NamedTempFile::with_suffix(".yaml")?;
    tmpfile.write_all(yaml.as_bytes())?;
    tmpfile.flush()?;

    LimaClient::create(instance, tmpfile.path().to_str().unwrap())?;
    println!("{} VM '{}' created.", "✓".green(), instance);

    println!("{} Starting VM '{}'...", "→".cyan(), instance);
    LimaClient::start(instance)?;
    println!("{} VM '{}' started.", "✓".green(), instance);

    // Run initial provisioning
    println!("{} Running initial provisioning...", "→".cyan());
    runner::provision(instance, &config)?;
    println!("{} Provisioning complete!", "✓".green());

    if !no_hosts {
        update_etc_hosts(instance, &config)?;
    }

    println!("Type \"{}\" for accessing to your development environment", "limaval ssh".green());

    Ok(())
}
