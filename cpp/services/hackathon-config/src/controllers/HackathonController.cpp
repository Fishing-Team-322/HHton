#include "HackathonController.h"

#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>
#include <optional>
#include <regex>
#include <unordered_set>
#include <vector>

using nlohmann::json;

namespace {
const std::unordered_set<std::string> kAllowedFormats = {"online", "offline", "hybrid"};
const std::unordered_set<std::string> kAllowedStatuses = {"draft", "published", "archived"};

bool matches_datetime(const std::string& value) {
    static const std::regex pattern(R"(^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:Z|[+-]\d{2}:?\d{2})?$)");
    return std::regex_match(value, pattern);
}

std::optional<std::string> validate_payload(const json& payload) {
    const std::vector<std::string> required_fields = {"organizer_id", "title", "description", "format", "status"};
    for (const auto& field : required_fields) {
        if (!payload.contains(field)) {
            return std::string("Missing required field: ") + field;
        }
    }

    if (!payload["organizer_id"].is_number_integer() || payload["organizer_id"].get<long long>() <= 0) {
        return "organizer_id must be a positive integer";
    }
    if (!payload["title"].is_string() || payload["title"].get_ref<const std::string&>().empty()) {
        return "title must be a non-empty string";
    }
    if (!payload["description"].is_string() || payload["description"].get_ref<const std::string&>().empty()) {
        return "description must be a non-empty string";
    }
    if (!payload["format"].is_string()) {
        return "format must be a string";
    }
    if (!kAllowedFormats.contains(payload["format"].get_ref<const std::string&>())) {
        return "Invalid format. Allowed values: online, offline, hybrid";
    }

    if (payload.contains("location") && !payload["location"].is_string() && !payload["location"].is_null()) {
        return "location must be a string or null";
    }

    auto validate_datetime_field = [](const json& obj, const std::string& field) -> std::optional<std::string> {
        if (!obj.contains(field) || obj[field].is_null()) {
            return std::nullopt;
        }
        if (!obj[field].is_string()) {
            return field + " must be a string or null";
        }
        if (!matches_datetime(obj[field].get_ref<const std::string&>())) {
            return field + " must follow ISO-8601 format (e.g. 2025-11-28T09:06:43 or 2025-11-28T09:06:43Z)";
        }
        return std::nullopt;
    };

    if (auto err = validate_datetime_field(payload, "start_at")) {
        return err;
    }
    if (auto err = validate_datetime_field(payload, "end_at")) {
        return err;
    }

    if (!payload["status"].is_string()) {
        return "status must be a string";
    }
    if (!kAllowedStatuses.contains(payload["status"].get_ref<const std::string&>())) {
        return "Invalid status. Allowed values: draft, published, archived";
    }

    return std::nullopt;
}
} // namespace

std::shared_ptr<HackathonRepository> HackathonController::repo_ = nullptr;

void HackathonController::set_repository(std::shared_ptr<HackathonRepository> repo) {
    repo_ = std::move(repo);
}

void HackathonController::create_hackathon(const drogon::HttpRequestPtr& req, std::function<void (const drogon::HttpResponsePtr &)> &&callback) {
    if (!repo_) {
        callback(internal_error());
        return;
    }

    json payload;
    try {
        payload = json::parse(req->getBody());
    } catch (const std::exception& ex) {
        spdlog::warn("Failed to parse create_hackathon payload: {}", ex.what());
        callback(bad_request("Invalid JSON payload"));
        return;
    }

    if (auto validation_error = validate_payload(payload)) {
        callback(bad_request(*validation_error));
        return;
    }

    try {
        Hackathon h{};
        h.organizer_id = payload["organizer_id"].get<long long>();
        h.title = payload["title"].get<std::string>();
        h.description = payload["description"].get<std::string>();
        h.format = payload["format"].get<std::string>();
        h.location = payload.contains("location") && !payload["location"].is_null() ? std::optional<std::string>(payload["location"].get<std::string>()) : std::nullopt;
        h.start_at = payload.contains("start_at") && !payload["start_at"].is_null() ? std::optional<std::string>(payload["start_at"].get<std::string>()) : std::nullopt;
        h.end_at = payload.contains("end_at") && !payload["end_at"].is_null() ? std::optional<std::string>(payload["end_at"].get<std::string>()) : std::nullopt;
        h.status = payload["status"].get<std::string>();

        auto new_id = repo_->create(h);
        json response{{"id", new_id}};
        auto resp = drogon::HttpResponse::newHttpResponse();
        resp->setStatusCode(drogon::k201Created);
        resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
        resp->setBody(response.dump());
        callback(resp);
    } catch (const std::exception& ex) {
        spdlog::error("Failed to create hackathon: {}", ex.what());
        callback(internal_error());
    }
}

void HackathonController::get_hackathon(const drogon::HttpRequestPtr&, std::function<void (const drogon::HttpResponsePtr &)> &&callback, long long id) {
    if (!repo_) {
        callback(internal_error());
        return;
    }

    try {
        auto result = repo_->get_by_id(id);
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

        auto resp = drogon::HttpResponse::newHttpResponse();
        resp->setStatusCode(drogon::k200OK);
        resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
        resp->setBody(response.dump());
        callback(resp);
    } catch (const std::exception& ex) {
        spdlog::error("Failed to get hackathon {}: {}", id, ex.what());
        callback(not_found());
    }
}

void HackathonController::list_hackathons(const drogon::HttpRequestPtr&, std::function<void (const drogon::HttpResponsePtr &)> &&callback, long long organizer_id) {
    if (!repo_) {
        callback(internal_error());
        return;
    }

    try {
        auto items = repo_->list_by_organizer(organizer_id);
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
        auto resp = drogon::HttpResponse::newHttpResponse();
        resp->setStatusCode(drogon::k200OK);
        resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
        resp->setBody(response.dump());
        callback(resp);
    } catch (const std::exception& ex) {
        spdlog::error("Failed to list hackathons for organizer {}: {}", organizer_id, ex.what());
        callback(bad_request("Unable to list hackathons"));
    }
}

void HackathonController::update_hackathon(const drogon::HttpRequestPtr& req, std::function<void (const drogon::HttpResponsePtr &)> &&callback, long long id) {
    if (!repo_) {
        callback(internal_error());
        return;
    }

    try {
        auto existing = repo_->get_by_id(id);
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

        repo_->update(id, updated);
        callback(no_content());
    } catch (const std::exception& ex) {
        spdlog::error("Failed to update hackathon {}: {}", id, ex.what());
        callback(bad_request("Invalid request payload"));
    }
}

drogon::HttpResponsePtr HackathonController::bad_request(const std::string& message) {
    json response{{"error", message}};
    auto resp = drogon::HttpResponse::newHttpResponse();
    resp->setStatusCode(drogon::k400BadRequest);
    resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
    resp->setBody(response.dump());
    return resp;
}

drogon::HttpResponsePtr HackathonController::internal_error() {
    json response{{"error", "Internal server error"}};
    auto resp = drogon::HttpResponse::newHttpResponse();
    resp->setStatusCode(drogon::k500InternalServerError);
    resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
    resp->setBody(response.dump());
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
#include <regex>
#include <unordered_set>
