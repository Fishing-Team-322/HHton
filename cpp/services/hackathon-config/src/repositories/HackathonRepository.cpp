#include "HackathonRepository.h"

#include <pqxx/pqxx>
#include <spdlog/spdlog.h>
#include <chrono>

long long HackathonRepository::create(const Hackathon& h) {
    auto conn = pool_.acquire();
    pqxx::work tx(*conn);

    auto result = tx.exec_params(
        "INSERT INTO hackathons (organizer_id, title, description, format, location, start_at, end_at, status, created_at, updated_at) "
        "VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW()) RETURNING id",
        h.organizer_id,
        h.title,
        h.description,
        h.format,
        h.location ? *h.location : pqxx::null{},
        h.start_at ? *h.start_at : pqxx::null{},
        h.end_at ? *h.end_at : pqxx::null{},
        h.status);

    tx.commit();
    return result[0][0].as<long long>();
}

std::optional<Hackathon> HackathonRepository::get_by_id(long long id) {
    auto conn = pool_.acquire();
    pqxx::work tx(*conn);

    auto result = tx.exec_params(
        "SELECT id, organizer_id, title, description, format, location, start_at, end_at, status FROM hackathons WHERE id = $1",
        id);

    if (result.empty()) {
        return std::nullopt;
    }

    return map_row(result[0]);
}

std::vector<Hackathon> HackathonRepository::list_by_organizer(long long organizer_id) {
    auto conn = pool_.acquire();
    pqxx::work tx(*conn);

    auto result = tx.exec_params(
        "SELECT id, organizer_id, title, description, format, location, start_at, end_at, status FROM hackathons WHERE organizer_id = $1 ORDER BY created_at DESC",
        organizer_id);

    std::vector<Hackathon> items;
    items.reserve(result.size());
    for (const auto& row : result) {
        items.push_back(map_row(row));
    }
    return items;
}

void HackathonRepository::update(long long id, const Hackathon& h) {
    auto conn = pool_.acquire();
    pqxx::work tx(*conn);

    tx.exec_params(
        "UPDATE hackathons SET title = $2, description = $3, format = $4, location = $5, start_at = $6, end_at = $7, status = $8, updated_at = NOW() WHERE id = $1",
        id,
        h.title,
        h.description,
        h.format,
        h.location ? *h.location : pqxx::null{},
        h.start_at ? *h.start_at : pqxx::null{},
        h.end_at ? *h.end_at : pqxx::null{},
        h.status);

    tx.commit();
}

Hackathon HackathonRepository::map_row(const pqxx::row& row) const {
    Hackathon h{};
    h.id = row["id"].as<long long>();
    h.organizer_id = row["organizer_id"].as<long long>();
    h.title = row["title"].as<std::string>();
    h.description = row["description"].as<std::string>();
    h.format = row["format"].as<std::string>();
    if (!row["location"].is_null()) {
        h.location = row["location"].as<std::string>();
    }
    if (!row["start_at"].is_null()) {
        h.start_at = row["start_at"].as<std::string>();
    }
    if (!row["end_at"].is_null()) {
        h.end_at = row["end_at"].as<std::string>();
    }
    h.status = row["status"].as<std::string>();
    return h;
}
