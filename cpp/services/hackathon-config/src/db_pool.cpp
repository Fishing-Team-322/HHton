#include "db_pool.h"

#include <spdlog/spdlog.h>
#include <stdexcept>

DbPool::DbPool(std::string conn_str) : conn_str_(std::move(conn_str)) {
    if (conn_str_.empty()) {
        throw std::invalid_argument("Database connection string cannot be empty");
    }
}

std::unique_ptr<pqxx::connection> DbPool::acquire() const {
    try {
        return std::make_unique<pqxx::connection>(conn_str_);
    } catch (const std::exception& ex) {
        spdlog::error("Failed to create database connection: {}", ex.what());
        throw;
    }
}
