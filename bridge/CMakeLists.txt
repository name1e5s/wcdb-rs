
# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html

# Sets the minimum version of CMake required to build the native library.

cmake_minimum_required(VERSION 3.13)

# Declares and names the project.

project(WCDB_Rust_Bridge)

set(TARGET_NAME "WCDB")
set(WCDB_BRIDGE ON)
set(SKIP_WCONAN ON)
set(BUILD_SHARED_LIBS OFF)

add_subdirectory(../wcdb/src ${CMAKE_CURRENT_BINARY_DIR}/wcdb)

# Creates and names a library, sets it as either STATIC
# or SHARED, and provides the relative paths to its source code.
# You can define multiple libraries, and CMake builds them for you.
# Gradle automatically packages shared libraries with your APK.
set(WCDB_RUST_SRC_DIR ${CMAKE_CURRENT_LIST_DIR})

file(GLOB_RECURSE WCDB_RUST_SRC
    ${WCDB_RUST_SRC_DIR}/*.cpp
    ${WCDB_RUST_SRC_DIR}/*.c
    ${WCDB_RUST_SRC_DIR}/*.h
)

target_sources(${TARGET_NAME} PUBLIC ${WCDB_RUST_SRC})

if (APPLE)
    set_target_properties(${TARGET_NAME} PROPERTIES
        FRAMEWORK FALSE
    )
endif ()

install(
    TARGETS ${TARGET_NAME} sqlcipher zstd
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
    PUBLIC_HEADER DESTINATION include
)