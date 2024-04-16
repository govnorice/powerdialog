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

add_executable(powerdialog main.cpp
        modules/CssManager.cpp
        modules/CssManager.h
        widgets/ActionWidget.cpp
        widgets/ActionWidget.h)

# CSS
configure_file(${CMAKE_SOURCE_DIR}/style.css ${CMAKE_CURRENT_BINARY_DIR}/style.css COPYONLY)

# images
set(IMAGES_FILES
        ${CMAKE_SOURCE_DIR}/images/poweroff.svg
        ${CMAKE_SOURCE_DIR}/images/suspend.svg
        ${CMAKE_SOURCE_DIR}/images/reboot.svg
)

file(MAKE_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/images)

foreach(image ${IMAGES_FILES})
    get_filename_component(image_name ${image} NAME)
    configure_file(${image} ${CMAKE_CURRENT_BINARY_DIR}/images/${image_name} COPYONLY)
endforeach()