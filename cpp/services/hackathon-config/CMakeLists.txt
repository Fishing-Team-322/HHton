cmake_minimum_required(VERSION 3.20)

project(hackathon_config_service LANGUAGES CXX)

# Собираем все .cpp из src/
file(GLOB_RECURSE SRC_FILES
    src/*.cpp
)

add_executable(hackathon_config ${SRC_FILES})

# --- ИМЕННО ЗДЕСЬ мы подтягиваем библиотеки из vcpkg ---
# vcpkg ставит нам CONFIG-пакеты, которые объявляют эти targets:
#   Drogon::Drogon
#   nlohmann_json::nlohmann_json
#   spdlog::spdlog
#   pqxx::pqxx

find_package(Drogon CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(libpqxx CONFIG REQUIRED)

target_link_libraries(hackathon_config
    PRIVATE
        Drogon::Drogon
        nlohmann_json::nlohmann_json
        spdlog::spdlog
        pqxx::pqxx
)

target_include_directories(hackathon_config PRIVATE src)
set_target_properties(hackathon_config PROPERTIES
    CXX_STANDARD 20
    CXX_STANDARD_REQUIRED YES
)

