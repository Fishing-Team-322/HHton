#include "config.h"

#include <cstdlib>
#include <spdlog/spdlog.h>
#include <utility>

namespace {
std::pair<std::string, std::string> choose_db_conn() {
    const char* env_vars[] = {"HACKATHON_CONFIG_DB_URL", "DATABASE_URL", "DB_URL"};
    for (const auto* env_var : env_vars) {
        if (const char* value = std::getenv(env_var)) {
            if (value[0] != '\0') {
                return {value, env_var};
            }
        }
    }

    const std::string default_dsn = "postgres://postgres:postgres@localhost:5432/hhton";
    return {default_dsn, "built-in default (localhost:5432/hhton)"};
}
} // namespace

AppConfig load_config_from_env() {
    AppConfig cfg;

    auto [dsn, source] = choose_db_conn();
    cfg.db_conn_str = std::move(dsn);
    cfg.db_conn_source = std::move(source);
    spdlog::info("Using DB connection string from {}", cfg.db_conn_source);

    if (const char* port = std::getenv("HHTHON_HTTP_PORT")) {
        cfg.http_port = std::atoi(port);
    }
    if (cfg.http_port <= 0) {
        cfg.http_port = 8080;
    }
    return cfg;
}
