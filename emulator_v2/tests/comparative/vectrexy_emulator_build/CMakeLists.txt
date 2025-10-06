# CMakeLists.txt minimalista para compilar SOLO libvectrexy_emulator + ErrorHandler

cmake_minimum_required(VERSION 3.15)
project(vectrexy_emulator_only CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Disable asserts para evitar FAIL() en Release
add_definitions(-DNDEBUG)

set(VECTREXY_ROOT "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/vectrexy")

# SOLO ErrorHandler de CORE (mínimo necesario)
set(CORE_SOURCES "${VECTREXY_ROOT}/libs/core/src/ErrorHandler.cpp")

# Source files EMULATOR
file(GLOB EMULATOR_SOURCES "${VECTREXY_ROOT}/libs/emulator/src/*.cpp")

# Include directories
include_directories(
    "${VECTREXY_ROOT}/libs/emulator/include"
    "${VECTREXY_ROOT}/libs/core/include"
)

# Static libraries
add_library(vectrexy_core STATIC ${CORE_SOURCES})
add_library(vectrexy_emulator STATIC ${EMULATOR_SOURCES})

# Install (copy to location expected by vectrexy_runner)
install(TARGETS vectrexy_core vectrexy_emulator
    ARCHIVE DESTINATION "${CMAKE_SOURCE_DIR}/lib"
)
