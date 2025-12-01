#pragma once

#include <optional>
#include <string>

struct Hackathon {
    long long id{0};
    long long organizer_id{0};
    std::string title;
    std::string description;
    std::string format;
    std::optional<std::string> location;
    std::optional<std::string> start_at;
    std::optional<std::string> end_at;
    std::string status;
};
