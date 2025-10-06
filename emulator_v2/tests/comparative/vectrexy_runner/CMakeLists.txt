cmake_minimum_required(VERSION 3.15)
project(VectrexyRunner)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Ruta a Vectrexy
set(VECTREXY_ROOT "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/vectrexy")

# Incluir directorios de Vectrexy
include_directories(
    ${VECTREXY_ROOT}/libs/emulator/include
    ${VECTREXY_ROOT}/libs/core/include
    ${CMAKE_SOURCE_DIR}/external
)

# Ruta a librería precompilada
link_directories(${CMAKE_SOURCE_DIR}/lib)

# Ejecutable
add_executable(vectrexy_runner main.cpp)

# Linkear contra librerías de Vectrexy (precompiladas localmente)
target_link_libraries(vectrexy_runner
    vectrexy_emulator
    vectrexy_core
)

# Copiar BIOS al directorio de salida
add_custom_command(TARGET vectrexy_runner POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
    "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin"
    "$<TARGET_FILE_DIR:vectrexy_runner>/bios.bin"
)
