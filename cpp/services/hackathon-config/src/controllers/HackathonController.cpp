#include "HackathonController.h"

#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>
#include <optional>

using nlohmann::json;

HackathonController::HackathonController(std::shared_ptr<HackathonRepository> repo)
    : repository_(std::move(repo)) {}

void HackathonController::create_hackathon(const drogon::HttpRequestPtr& req, std::function<void (const drogon::HttpResponsePtr &)> &&callback) {
    try {
        auto payload = json::parse(req->getBody());
        if (!payload.contains("organizer_id") || !payload.contains("title") || !payload.contains("description") ||
            !payload.contains("format") || !payload.contains("status")) {
            callback(bad_request("Missing required fields"));
            return;
        }

        Hackathon h{};
        h.organizer_id = payload.value("organizer_id", 0LL);
        h.title = payload.value("title", "");
        h.description = payload.value("description", "");
        h.format = payload.value("format", "");
        h.location = payload.contains("location") && !payload["location"].is_null() ? std::optional<std::string>(payload["location"].get<std::string>()) : std::nullopt;
        h.start_at = payload.contains("start_at") && !payload["start_at"].is_null() ? std::optional<std::string>(payload["start_at"].get<std::string>()) : std::nullopt;
        h.end_at = payload.contains("end_at") && !payload["end_at"].is_null() ? std::optional<std::string>(payload["end_at"].get<std::string>()) : std::nullopt;
        h.status = payload.value("status", "draft");

        auto new_id = repository_->create(h);
        json response{{"id", new_id}};
        auto resp = drogon::HttpResponse::newHttpJsonResponse(response);
        resp->setStatusCode(drogon::k201Created);
        callback(resp);
    } catch (const std::exception& ex) {
        spdlog::error("Failed to create hackathon: {}", ex.what());
        callback(bad_request("Invalid request payload"));
    }
}

void HackathonController::get_hackathon(const drogon::HttpRequestPtr&, std::function<void (const drogon::HttpResponsePtr &)> &&callback, long long id) {
    try {
        auto result = repository_->get_by_id(id);
        if (!result) {
            callback(not_found());
            return;
        }

        json response = {
            {"id", result->id},
            {"organizer_id", result->organizer_id},
            {"title", result->title},
            {"description", result->description},
            {"format", result->format},
            {"location", result->location ? json(*result->location) : json(nullptr)},
            {"start_at", result->start_at ? json(*result->start_at) : json(nullptr)},
            {"end_at", result->end_at ? json(*result->end_at) : json(nullptr)},
            {"status", result->status}
        };

        auto resp = drogon::HttpResponse::newHttpJsonResponse(response);
        callback(resp);
    } catch (const std::exception& ex) {
        spdlog::error("Failed to get hackathon {}: {}", id, ex.what());
        callback(not_found());
    }
}

void HackathonController::list_hackathons(const drogon::HttpRequestPtr&, std::function<void (const drogon::HttpResponsePtr &)> &&callback, long long organizer_id) {
    try {
        auto items = repository_->list_by_organizer(organizer_id);
        json response = json::array();
        for (const auto& h : items) {
            response.push_back({
                {"id", h.id},
                {"title", h.title},
                {"status", h.status},
                {"start_at", h.start_at ? json(*h.start_at) : json(nullptr)},
                {"end_at", h.end_at ? json(*h.end_at) : json(nullptr)}
            });
        }
        callback(drogon::HttpResponse::newHttpJsonResponse(response));
    } catch (const std::exception& ex) {
        spdlog::error("Failed to list hackathons for organizer {}: {}", organizer_id, ex.what());
        callback(bad_request("Unable to list hackathons"));
    }
}

void HackathonController::update_hackathon(const drogon::HttpRequestPtr& req, std::function<void (const drogon::HttpResponsePtr &)> &&callback, long long id) {
    try {
        auto existing = repository_->get_by_id(id);
        if (!existing) {
            callback(not_found());
            return;
        }

        auto payload = json::parse(req->getBody());
        if (payload.contains("organizer_id")) {
            callback(bad_request("organizer_id cannot be updated"));
            return;
        }

        Hackathon updated = *existing;
        if (payload.contains("title")) {
            updated.title = payload.value("title", updated.title);
        }
        if (payload.contains("description")) {
            updated.description = payload.value("description", updated.description);
        }
        if (payload.contains("format")) {
            updated.format = payload.value("format", updated.format);
        }
        if (payload.contains("location")) {
            updated.location = payload["location"].is_null() ? std::nullopt : std::optional<std::string>(payload["location"].get<std::string>());
        }
        if (payload.contains("start_at")) {
            updated.start_at = payload["start_at"].is_null() ? std::nullopt : std::optional<std::string>(payload["start_at"].get<std::string>());
        }
        if (payload.contains("end_at")) {
            updated.end_at = payload["end_at"].is_null() ? std::nullopt : std::optional<std::string>(payload["end_at"].get<std::string>());
        }
        if (payload.contains("status")) {
            updated.status = payload.value("status", updated.status);
        }

        repository_->update(id, updated);
        callback(no_content());
    } catch (const std::exception& ex) {
        spdlog::error("Failed to update hackathon {}: {}", id, ex.what());
        callback(bad_request("Invalid request payload"));
    }
}

drogon::HttpResponsePtr HackathonController::bad_request(const std::string& message) {
    auto resp = drogon::HttpResponse::newHttpJsonResponse(json{{"error", message}});
    resp->setStatusCode(drogon::k400BadRequest);
    return resp;
}

drogon::HttpResponsePtr HackathonController::not_found() {
    auto resp = drogon::HttpResponse::newNotFoundResponse();
    return resp;
}

drogon::HttpResponsePtr HackathonController::no_content() {
    auto resp = drogon::HttpResponse::newHttpResponse();
    resp->setStatusCode(drogon::k204NoContent);
    return resp;
}
