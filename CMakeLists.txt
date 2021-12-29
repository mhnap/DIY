cmake_minimum_required(VERSION 3.20)
project(DIY)

set(CMAKE_CXX_STANDARD 20)

find_package(GTest REQUIRED)

add_subdirectory(src)