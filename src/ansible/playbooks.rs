pub const PLAYBOOK_YML: &str = include_str!("../../ansible/playbook.yml");

// Common
pub const COMMON_TASKS: &str = include_str!("../../ansible/roles/common/tasks/main.yml");

// Network
pub const NETWORK_TASKS: &str = include_str!("../../ansible/roles/network/tasks/main.yml");
pub const NETWORK_HANDLERS: &str = include_str!("../../ansible/roles/network/handlers/main.yml");

// PHP
pub const PHP_TASKS: &str = include_str!("../../ansible/roles/php/tasks/main.yml");
pub const PHP_FPM_POOL_TEMPLATE: &str =
    include_str!("../../ansible/roles/php/templates/php-fpm-pool.conf.j2");

// Nginx
pub const NGINX_TASKS: &str = include_str!("../../ansible/roles/nginx/tasks/main.yml");
pub const NGINX_SITE_TEMPLATE: &str =
    include_str!("../../ansible/roles/nginx/templates/site.conf.j2");
pub const NGINX_HANDLERS: &str = include_str!("../../ansible/roles/nginx/handlers/main.yml");

// Composer
pub const COMPOSER_TASKS: &str = include_str!("../../ansible/roles/composer/tasks/main.yml");

// Node.js
pub const NODEJS_TASKS: &str = include_str!("../../ansible/roles/nodejs/tasks/main.yml");

// Valkey
pub const VALKEY_TASKS: &str = include_str!("../../ansible/roles/valkey/tasks/main.yml");

// MariaDB
pub const MARIADB_TASKS: &str = include_str!("../../ansible/roles/mariadb/tasks/main.yml");
pub const MARIADB_HANDLERS: &str = include_str!("../../ansible/roles/mariadb/handlers/main.yml");

// MySQL
pub const MYSQL_TASKS: &str = include_str!("../../ansible/roles/mysql/tasks/main.yml");
pub const MYSQL_HANDLERS: &str = include_str!("../../ansible/roles/mysql/handlers/main.yml");

// PostgreSQL
pub const POSTGRESQL_TASKS: &str = include_str!("../../ansible/roles/postgresql/tasks/main.yml");
pub const POSTGRESQL_HANDLERS: &str =
    include_str!("../../ansible/roles/postgresql/handlers/main.yml");

// Databases
pub const DATABASES_TASKS: &str = include_str!("../../ansible/roles/databases/tasks/main.yml");

// Oh My Zsh
pub const OHMYZSH_TASKS: &str = include_str!("../../ansible/roles/ohmyzsh/tasks/main.yml");

// MailHog
pub const MAILHOG_TASKS: &str = include_str!("../../ansible/roles/mailhog/tasks/main.yml");

// MongoDB
pub const MONGODB_TASKS: &str = include_str!("../../ansible/roles/mongodb/tasks/main.yml");

// WebDriver
pub const WEBDRIVER_TASKS: &str = include_str!("../../ansible/roles/webdriver/tasks/main.yml");

pub struct PlaybookFile {
    pub relative_path: &'static str,
    pub content: &'static str,
}

pub fn all_files() -> Vec<PlaybookFile> {
    vec![
        PlaybookFile {
            relative_path: "playbook.yml",
            content: PLAYBOOK_YML,
        },
        PlaybookFile {
            relative_path: "roles/common/tasks/main.yml",
            content: COMMON_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/network/tasks/main.yml",
            content: NETWORK_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/network/handlers/main.yml",
            content: NETWORK_HANDLERS,
        },
        PlaybookFile {
            relative_path: "roles/php/tasks/main.yml",
            content: PHP_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/php/templates/php-fpm-pool.conf.j2",
            content: PHP_FPM_POOL_TEMPLATE,
        },
        PlaybookFile {
            relative_path: "roles/nginx/tasks/main.yml",
            content: NGINX_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/nginx/templates/site.conf.j2",
            content: NGINX_SITE_TEMPLATE,
        },
        PlaybookFile {
            relative_path: "roles/nginx/handlers/main.yml",
            content: NGINX_HANDLERS,
        },
        PlaybookFile {
            relative_path: "roles/composer/tasks/main.yml",
            content: COMPOSER_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/nodejs/tasks/main.yml",
            content: NODEJS_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/valkey/tasks/main.yml",
            content: VALKEY_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/mariadb/tasks/main.yml",
            content: MARIADB_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/mariadb/handlers/main.yml",
            content: MARIADB_HANDLERS,
        },
        PlaybookFile {
            relative_path: "roles/mysql/tasks/main.yml",
            content: MYSQL_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/mysql/handlers/main.yml",
            content: MYSQL_HANDLERS,
        },
        PlaybookFile {
            relative_path: "roles/postgresql/tasks/main.yml",
            content: POSTGRESQL_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/postgresql/handlers/main.yml",
            content: POSTGRESQL_HANDLERS,
        },
        PlaybookFile {
            relative_path: "roles/databases/tasks/main.yml",
            content: DATABASES_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/ohmyzsh/tasks/main.yml",
            content: OHMYZSH_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/mailhog/tasks/main.yml",
            content: MAILHOG_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/mongodb/tasks/main.yml",
            content: MONGODB_TASKS,
        },
        PlaybookFile {
            relative_path: "roles/webdriver/tasks/main.yml",
            content: WEBDRIVER_TASKS,
        },
    ]
}

pub fn write_all(dir: &std::path::Path) -> std::io::Result<()> {
    for file in all_files() {
        let path = dir.join(file.relative_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, file.content)?;
    }
    Ok(())
}
