use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "limavel",
    version,
    about = "Laravel Virtual Machine Manager",
    before_help = r#"

╷  ╷╭┬╮╭─╮╷ ╷╭─╴╷
│  ││││├─┤│╭╯├╴ │
╰─╴╵╵ ╵╵ ╵╰╯ ╰─╴╰─╴"#
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new configuration file
    Init {
        /// Instance name (creates <name>.yaml, defaults to "limavel")
        #[arg(default_value = "limavel")]
        name: String,
    },
    /// Start the development VM (creates it if needed)
    Start {
        /// Instance name (reads <name>.yaml, defaults to "limavel")
        #[arg(default_value = "limavel")]
        name: String,
        /// Do not update /etc/hosts with site domains
        #[arg(long)]
        no_hosts: bool,
    },
    /// Stop the development VM
    Stop {
        /// Instance name (reads <name>.yaml, defaults to "limavel")
        #[arg(default_value = "limavel")]
        name: String,
        /// Do not remove /etc/hosts entries
        #[arg(long)]
        no_hosts: bool,
    },
    /// Restart the development VM
    Reboot {
        /// Instance name (reads <name>.yaml, defaults to "limavel")
        #[arg(default_value = "limavel")]
        name: String,
    },
    /// Re-provision the development VM
    Provision {
        /// Instance name (reads <name>.yaml, defaults to "limavel")
        #[arg(default_value = "limavel")]
        name: String,
    },
    /// SSH into the development VM
    Ssh {
        /// Instance name (reads <name>.yaml, defaults to "limavel")
        #[arg(default_value = "limavel")]
        name: String,
    },
    /// Show the VM instance status
    Status {
        /// Instance name (reads <name>.yaml, defaults to "limavel")
        #[arg(default_value = "limavel")]
        name: String,
    },
    /// Edit VM resources (memory, cpus) from config
    Edit {
        /// Instance name (reads <name>.yaml, defaults to "limavel")
        #[arg(default_value = "limavel")]
        name: String,
    },
    /// Destroy the VM instance
    Destroy {
        /// Instance name (reads <name>.yaml, defaults to "limavel")
        #[arg(default_value = "limavel")]
        name: String,
    },
}
