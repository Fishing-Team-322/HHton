#include "config.h"

#include <cstdlib>

AppConfig load_config_from_env() {
    AppConfig cfg;
    if (const char* dsn = std::getenv("HHTHON_DB_DSN")) {
        cfg.db_conn_str = dsn;
    }
    if (const char* port = std::getenv("HHTHON_HTTP_PORT")) {
        cfg.http_port = std::atoi(port);
    }
    if (cfg.http_port <= 0) {
        cfg.http_port = 8080;
    }
    return cfg;
}
