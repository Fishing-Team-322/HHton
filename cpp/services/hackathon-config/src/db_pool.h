#pragma once

#include <memory>
#include <pqxx/pqxx>
#include <string>

class DbPool {
public:
    DbPool(std::string conn_str, std::string conn_source)
        : conn_str_(std::move(conn_str)), conn_source_(std::move(conn_source)) {}
    virtual ~DbPool() = default;

    virtual std::unique_ptr<pqxx::connection> acquire() const;

private:
    std::string conn_str_;
    std::string conn_source_;
};
