#pragma once

#include "../db_pool.h"
#include "../models/Hackathon.h"

#include <optional>
#include <vector>

class HackathonRepository {
public:
    explicit HackathonRepository(const DbPool& pool) : pool_(pool) {}

    long long create(const Hackathon& h);
    std::optional<Hackathon> get_by_id(long long id);
    std::vector<Hackathon> list_by_organizer(long long organizer_id);
    void update(long long id, const Hackathon& h);

private:
    Hackathon map_row(const pqxx::row& row) const;

    const DbPool& pool_;
};
