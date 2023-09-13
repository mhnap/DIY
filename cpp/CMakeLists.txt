cmake_minimum_required(VERSION 3.20)
project(DIY)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_COMPILER "g++-12")

if (EXISTS ${CMAKE_BINARY_DIR}/conan_paths.cmake)
    include(${CMAKE_BINARY_DIR}/conan_paths.cmake)
endif ()

find_package(GTest REQUIRED)
find_package(OpenCL REQUIRED)

add_subdirectory(src)