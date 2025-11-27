#include "config.h"
#include "controllers/HackathonController.h"
#include "db_pool.h"
#include "repositories/HackathonRepository.h"

#include <drogon/drogon.h>
#include <spdlog/spdlog.h>

int main() {
    auto config = load_config_from_env();
    spdlog::info("Starting hackathon-config service on port {}", config.http_port);

    DbPool pool(config.db_conn_str);
    auto repository = std::make_shared<HackathonRepository>(pool);
    auto controller = std::make_shared<HackathonController>(repository);

    drogon::app()
        .registerController(controller)
        .addListener("0.0.0.0", static_cast<uint16_t>(config.http_port))
        .run();

    return 0;
}
