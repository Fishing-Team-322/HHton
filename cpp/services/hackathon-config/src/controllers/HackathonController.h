#pragma once

#include "../repositories/HackathonRepository.h"

#include <drogon/HttpController.h>
#include <memory>

class HackathonController : public drogon::HttpController<HackathonController> {
public:
    explicit HackathonController(std::shared_ptr<HackathonRepository> repo);

    METHOD_LIST_BEGIN
    ADD_METHOD_TO(HackathonController::create_hackathon, "/api/v1/hackathons", drogon::Post);
    ADD_METHOD_TO(HackathonController::get_hackathon, "/api/v1/hackathons/{1}", drogon::Get);
    ADD_METHOD_TO(HackathonController::list_hackathons, "/api/v1/organizers/{1}/hackathons", drogon::Get);
    ADD_METHOD_TO(HackathonController::update_hackathon, "/api/v1/hackathons/{1}", drogon::Put);
    METHOD_LIST_END

    void create_hackathon(const drogon::HttpRequestPtr& req, std::function<void (const drogon::HttpResponsePtr &)> &&callback);
    void get_hackathon(const drogon::HttpRequestPtr& req, std::function<void (const drogon::HttpResponsePtr &)> &&callback, long long id);
    void list_hackathons(const drogon::HttpRequestPtr& req, std::function<void (const drogon::HttpResponsePtr &)> &&callback, long long organizer_id);
    void update_hackathon(const drogon::HttpRequestPtr& req, std::function<void (const drogon::HttpResponsePtr &)> &&callback, long long id);

private:
    std::shared_ptr<HackathonRepository> repository_;

    static drogon::HttpResponsePtr bad_request(const std::string& message);
    static drogon::HttpResponsePtr not_found();
    static drogon::HttpResponsePtr no_content();
};
