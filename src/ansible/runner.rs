use anyhow::{Context, Result};
use serde::Serialize;
use tempfile::TempDir;

use crate::config::limavel_config::LimavelConfig;
use crate::lima::client::LimaClient;

use super::playbooks;

#[derive(Serialize)]
struct AnsibleVars {
    ssh_public_key: String,
    sites: Vec<AnsibleSite>,
    php_versions: Vec<String>,
    db_type: String,
    db_version: String,
    db_password: String,
    databases: Vec<String>,
    features: AnsibleFeatures,
    nodejs_version: String,
}

#[derive(Serialize)]
struct AnsibleSite {
    domain: String,
    root: String,
    php_version: String,
}

#[derive(Serialize)]
struct AnsibleFeatures {
    ohmyzsh: bool,
    webdriver: bool,
    mailhog: bool,
    mongodb: bool,
    valkey: bool,
}

fn ensure_ansible_installed(name: &str) -> Result<()> {
    let check = LimaClient::shell(name, "which ansible-playbook");
    if check.is_err() {
        println!("Ansible not found in guest, installing...");
        LimaClient::shell(name, "sudo apt-get update -qq && sudo apt-get install -y -qq ansible")
            .context("Failed to install ansible in guest")?;
    }
    Ok(())
}

pub fn provision(name: &str, config: &LimavelConfig) -> Result<()> {
    let ssh_pubkey = config
        .read_ssh_pubkey()
        .context("Failed to read SSH public key")?;

    let vars = AnsibleVars {
        ssh_public_key: ssh_pubkey,
        sites: config
            .sites
            .iter()
            .map(|s| AnsibleSite {
                domain: s.map.clone(),
                root: s.to.clone(),
                php_version: s.php.clone(),
            })
            .collect(),
        php_versions: config.php_versions(),
        db_type: config.database.db_type.clone(),
        db_version: config.database.version.clone(),
        db_password: config.database.password.clone(),
        databases: config.databases.clone(),
        features: AnsibleFeatures {
            ohmyzsh: config.features.ohmyzsh,
            webdriver: config.features.webdriver,
            mailhog: config.features.mailhog,
            mongodb: config.features.mongodb,
            valkey: config.features.valkey,
        },
        nodejs_version: "24".to_string(),
    };

    let vars_yaml = serde_yml::to_string(&vars).context("Failed to serialize ansible vars")?;

    // Write playbooks to temp directory
    let tmpdir = TempDir::new().context("Failed to create temp directory")?;
    let ansible_dir = tmpdir.path();

    playbooks::write_all(ansible_dir)?;

    // Write vars file
    std::fs::write(ansible_dir.join("vars.yml"), &vars_yaml)?;

    // Ensure ansible is installed in the guest
    ensure_ansible_installed(name)?;

    // Copy playbooks to guest via tar pipe (avoids limactl copy directory nesting issues)
    LimaClient::shell(name, "sudo mkdir -p /opt/limavel/ansible")?;
    LimaClient::tar_to_guest(name, ansible_dir, "/opt/limavel/ansible")?;

    // Run ansible-playbook inside guest
    LimaClient::shell_interactive(
        name,
        "sudo ansible-playbook -c local -i 'localhost,' /opt/limavel/ansible/playbook.yml -e @/opt/limavel/ansible/vars.yml",
    )?;

    Ok(())
}
