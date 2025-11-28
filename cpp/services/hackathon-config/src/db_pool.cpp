#include "db_pool.h"

#include <spdlog/spdlog.h>

std::unique_ptr<pqxx::connection> DbPool::acquire() const {
    try {
        return std::make_unique<pqxx::connection>(conn_str_);
    } catch (const std::exception& ex) {
        spdlog::error("Failed to create database connection (source: {}): {}", conn_source_, ex.what());
        throw;
    }
}
