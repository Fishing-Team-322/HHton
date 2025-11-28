#pragma once

#include <memory>
#include <pqxx/pqxx>
#include <string>

class DbPool {
public:
    explicit DbPool(std::string conn_str);
    virtual ~DbPool() = default;

    virtual std::unique_ptr<pqxx::connection> acquire() const;

private:
    std::string conn_str_;
};
