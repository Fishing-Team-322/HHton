cmake_minimum_required(VERSION 3.20)
project(hhthon_cpp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(BUILD_TESTING "Enable testing" ON)

add_subdirectory(services/hackathon-config)
if(BUILD_TESTING)
  enable_testing()
  add_subdirectory(tests)
endif()
