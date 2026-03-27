use anyhow::Result;
use serde::Serialize;

use super::limavel_config::LimavelConfig;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LimaConfig {
    pub vm_type: String,
    pub os: String,
    pub arch: String,
    pub images: Vec<LimaImage>,
    pub cpus: u32,
    pub memory: String,
    pub disk: String,
    pub mount_type: String,
    pub mounts: Vec<LimaMount>,
    pub networks: Vec<LimaNetwork>,
    pub port_forwards: Vec<LimaPortForward>,
    pub ssh: LimaSsh,
    pub containerd: LimaContainerd,
    pub provision: Vec<LimaProvision>,
}

#[derive(Debug, Serialize)]
pub struct LimaImage {
    pub location: String,
    pub arch: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LimaMount {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_point: Option<String>,
    pub writable: bool,
}

#[derive(Debug, Serialize)]
pub struct LimaNetwork {
    #[serde(rename = "vzNAT")]
    pub vz_nat: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LimaPortForward {
    pub guest_port: u16,
    pub host_port: u16,
}

#[derive(Debug, Serialize)]
pub struct LimaSsh {
    #[serde(rename = "loadDotSSHPubKeys")]
    pub load_dot_ssh_pub_keys: bool,
}

#[derive(Debug, Serialize)]
pub struct LimaContainerd {
    pub system: bool,
    pub user: bool,
}

#[derive(Debug, Serialize)]
pub struct LimaProvision {
    pub mode: String,
    pub script: String,
}

impl LimaConfig {
    pub fn from_config(config: &LimavelConfig, ssh_pubkey: &str) -> Self {
        let mut mounts: Vec<LimaMount> = config
            .folders
            .iter()
            .map(|f| {
                let expanded = shellexpand::tilde(&f.map).to_string();
                LimaMount {
                    location: expanded,
                    mount_point: Some(f.to.clone()),
                    writable: true,
                }
            })
            .collect();

        // Add /tmp/lima mount
        mounts.push(LimaMount {
            location: "/tmp/lima".to_string(),
            mount_point: None,
            writable: true,
        });

        let port_forwards: Vec<LimaPortForward> = config
            .ports
            .iter()
            .map(|p| LimaPortForward {
                guest_port: p.to,
                host_port: p.send,
            })
            .collect();

        let bootstrap_script = generate_bootstrap_script(ssh_pubkey);

        LimaConfig {
            vm_type: "vz".to_string(),
            os: "Linux".to_string(),
            arch: "default".to_string(),
            images: vec![
                LimaImage {
                    location: "https://cloud.debian.org/images/cloud/trixie/daily/latest/debian-13-genericcloud-amd64-daily.qcow2".to_string(),
                    arch: "x86_64".to_string(),
                },
                LimaImage {
                    location: "https://cloud.debian.org/images/cloud/trixie/daily/latest/debian-13-genericcloud-arm64-daily.qcow2".to_string(),
                    arch: "aarch64".to_string(),
                },
            ],
            cpus: config.cpus,
            memory: format!("{}MiB", config.memory),
            disk: "50GiB".to_string(),
            mount_type: "virtiofs".to_string(),
            mounts,
            networks: vec![LimaNetwork { vz_nat: true }],
            port_forwards,
            ssh: LimaSsh {
                load_dot_ssh_pub_keys: true,
            },
            containerd: LimaContainerd {
                system: false,
                user: false,
            },
            provision: vec![LimaProvision {
                mode: "system".to_string(),
                script: bootstrap_script,
            }],
        }
    }

    pub fn to_yaml(&self) -> Result<String> {
        let yaml = serde_yml::to_string(self)?;
        Ok(yaml)
    }
}

fn generate_bootstrap_script(ssh_pubkey: &str) -> String {
    format!(
        r#"#!/bin/bash
set -eux

# Wait for dpkg lock
while fuser /var/lib/dpkg/lock-frontend >/dev/null 2>&1; do
    sleep 2
done

export DEBIAN_FRONTEND=noninteractive

# Install Ansible and dependencies
apt-get update -y
apt-get install -y ansible python3-pip python3-pymysql python3-psycopg2 gnupg2 ca-certificates lsb-release

# Install required Ansible collections
ansible-galaxy collection install community.mysql community.postgresql

# Create limavel user if not exists
if ! id -u limavel &>/dev/null; then
    useradd -m -s /bin/bash -G sudo limavel
    echo "limavel ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/limavel
fi

# Copy skeleton files and fix ownership (useradd -m skips these when /home/limavel already exists)
cp -rn /etc/skel/. /home/limavel/
chown -R limavel:limavel /home/limavel

# Set up SSH authorized keys for limavel user
mkdir -p /home/limavel/.ssh
echo '{ssh_pubkey}' > /home/limavel/.ssh/authorized_keys
chmod 700 /home/limavel/.ssh
chmod 600 /home/limavel/.ssh/authorized_keys
chown -R limavel:limavel /home/limavel/.ssh

# Remove the home directory Lima creates for the host user
for d in /home/*.linux; do
    [ -d "$d" ] && rm -rf "$d"
done

# Create ansible directory
mkdir -p /opt/limavel/ansible
"#
    )
}
