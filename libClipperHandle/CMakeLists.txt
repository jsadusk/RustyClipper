cmake_minimum_required(VERSION 3.0)
project(ClipperHandle)

add_library(ClipperHandle STATIC ClipperHandle.cpp)

install(TARGETS ClipperHandle DESTINATION .)
