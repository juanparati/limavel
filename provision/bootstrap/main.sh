#!/bin/bash
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
