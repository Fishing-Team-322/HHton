#include "config.h"
#include "controllers/HackathonController.h"
#include "db_pool.h"
#include "repositories/HackathonRepository.h"

#include <drogon/drogon.h>
#include <spdlog/spdlog.h>

int main() {
    auto config = load_config_from_env();
    spdlog::info("Starting hackathon-config service on port {}", config.http_port);
    spdlog::info("Using DB connection string from {}: {}", config.db_conn_source, config.db_conn_log_hint);

    DbPool pool(config.db_conn_str);
    auto repository = std::make_shared<HackathonRepository>(pool);
    HackathonController::set_repository(repository);

    constexpr auto cors_origin = "http://localhost:8081";
    auto &app = drogon::app();

    app.registerBeginningAdvice([cors_origin](const drogon::HttpRequestPtr &req,
                                              drogon::AdviceCallback &&cb,
                                              drogon::AdviceChainCallback &&chain_cb) {
        if (req->path().rfind("/api/", 0) == 0 && req->method() == drogon::Options) {
            auto resp = drogon::HttpResponse::newHttpResponse();
            resp->setStatusCode(drogon::k204NoContent);
            resp->addHeader("Access-Control-Allow-Origin", cors_origin);
            resp->addHeader("Access-Control-Allow-Methods", "GET,POST,PUT,DELETE,OPTIONS");
            resp->addHeader("Access-Control-Allow-Headers", "Content-Type, Authorization");
            cb(resp);
            return;
        }
        chain_cb();
    });

    app.registerPostHandlingAdvice(
        [cors_origin](const drogon::HttpRequestPtr &req, const drogon::HttpResponsePtr &resp) {
            if (req->path().rfind("/api/", 0) == 0) {
                resp->addHeader("Access-Control-Allow-Origin", cors_origin);
            }
        });

    app.addListener("0.0.0.0", static_cast<uint16_t>(config.http_port))
        .run();

    return 0;
}
