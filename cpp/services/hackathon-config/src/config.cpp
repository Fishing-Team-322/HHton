#include "config.h"

#include <cstdlib>
#include <regex>
#include <string>

namespace {
std::string describe_conn_string(const std::string& dsn) {
    // Try to parse URL-style connection strings without exposing credentials.
    const std::regex url_re(R"((?:postgres(?:ql)?://)(?:[^@/]+@)?([^/:]+)(?::(\d+))?/([^/?]+))",
                             std::regex::icase);
    std::smatch match;
    if (std::regex_search(dsn, match, url_re) && match.size() >= 4) {
        const auto& host = match[1];
        const auto& port = match[2];
        const auto& db = match[3];
        std::string port_str = port.matched ? port.str() : "(default)";
        return "host=" + host.str() + " db=" + db.str() + " port=" + port_str;
    }

    // Fallback when parsing fails.
    return "connection string configured";
}
} // namespace

AppConfig load_config_from_env() {
    AppConfig cfg;
    const char* dsn = nullptr;
    if ((dsn = std::getenv("HACKATHON_CONFIG_DB_URL"))) {
        cfg.db_conn_source = "HACKATHON_CONFIG_DB_URL";
    } else if ((dsn = std::getenv("DATABASE_URL"))) {
        cfg.db_conn_source = "DATABASE_URL";
    } else if ((dsn = std::getenv("DB_URL"))) {
        cfg.db_conn_source = "DB_URL";
    }

    if (dsn) {
        cfg.db_conn_str = dsn;
    } else {
        cfg.db_conn_str = "postgresql://hhton:hhton@localhost:5432/hhton";
        cfg.db_conn_source = "fallback";
    }

    cfg.db_conn_log_hint = describe_conn_string(cfg.db_conn_str);
    if (const char* port = std::getenv("HHTHON_HTTP_PORT")) {
        cfg.http_port = std::atoi(port);
    }
    if (cfg.http_port <= 0) {
        cfg.http_port = 8080;
    }
    return cfg;
}
