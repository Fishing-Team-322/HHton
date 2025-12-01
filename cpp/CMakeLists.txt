cmake_minimum_required(VERSION 3.20)
project(hhthon_cpp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Если хочешь оставить жёсткое требование на vcpkg — можно:
if(NOT DEFINED CMAKE_TOOLCHAIN_FILE)
  message(FATAL_ERROR "You must pass -DCMAKE_TOOLCHAIN_FILE=/path/to/vcpkg.cmake")
endif()

add_subdirectory(services/hackathon-config)

# Тесты подключим позже, когда всё остальное будет собираться
# add_subdirectory(tests)
