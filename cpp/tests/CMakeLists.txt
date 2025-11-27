cmake_minimum_required(VERSION 3.20)

add_executable(test_hackathon_repository
  test_hackathon_repository.cpp
)

target_include_directories(test_hackathon_repository
  PRIVATE
    ${CMAKE_CURRENT_LIST_DIR}/../services/hackathon-config/src
)

target_link_libraries(test_hackathon_repository
  PRIVATE
    GTest::gtest_main
    spdlog::spdlog
)

target_compile_features(test_hackathon_repository PRIVATE cxx_std_20)

include(GoogleTest)
gtest_discover_tests(test_hackathon_repository)
