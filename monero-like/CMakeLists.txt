cmake_minimum_required(VERSION 3.16)

project(monero_demo LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_executable(monero_demo
    src/main.cpp
)

target_include_directories(monero_demo PRIVATE
    /usr/include
)

target_link_libraries(monero_demo PRIVATE
    cryptopp
)
