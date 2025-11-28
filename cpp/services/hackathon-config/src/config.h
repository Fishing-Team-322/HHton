#pragma once

#include <string>

struct AppConfig {
    std::string db_conn_str;
    std::string db_conn_source;
    std::string db_conn_log_hint;
    int http_port{8080};
};

AppConfig load_config_from_env();
