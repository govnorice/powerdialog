cmake_minimum_required(VERSION 3.26)
project(powerdialog)

set(CMAKE_CXX_STANDARD 17)

find_package(PkgConfig REQUIRED)

pkg_check_modules(GTK REQUIRED IMPORTED_TARGET gtk+-3.0)
pkg_check_modules(LIBSVG REQUIRED IMPORTED_TARGET librsvg-2.0)

include_directories(${GTK_INCLUDE_DIRS})
include_directories(${LIBSVG_INCLUDE_DIRS})

link_libraries(${GTK_LIBRARIES})
link_libraries(${LIBSVG_LIBRARIES})

# CSS
set(CSS_FILE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/style.css")
set(STYLE_HEX_FILE "${CMAKE_CURRENT_SOURCE_DIR}/style.hex")

# Convert to style.hex
execute_process(COMMAND xxd -i "${CSS_FILE_PATH}" "${STYLE_HEX_FILE}")

add_executable(powerdialog main.cpp
        modules/CssManager.cpp
        modules/CssManager.h
        widgets/ActionWidget.cpp
        widgets/ActionWidget.h
        images/images.h
        "${STYLE_HEX_FILE}"
)