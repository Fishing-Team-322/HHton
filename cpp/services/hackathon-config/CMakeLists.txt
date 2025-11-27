cmake_minimum_required(VERSION 3.20)

add_executable(hackathon_config
  src/main.cpp
  src/config.cpp
  src/db_pool.cpp
  src/repositories/HackathonRepository.cpp
  src/controllers/HackathonController.cpp
)

target_include_directories(hackathon_config
  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

target_link_libraries(hackathon_config
  PRIVATE
    Drogon::Drogon
    nlohmann_json::nlohmann_json
    spdlog::spdlog
    pqxx::pqxx
)

target_compile_features(hackathon_config PRIVATE cxx_std_20)
