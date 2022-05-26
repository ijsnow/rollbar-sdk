cmake_minimum_required(VERSION 3.19)

project(rollbar)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.2.1 # Optionally specify a commit hash, version tag or branch here
)

FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH rollbar/Cargo.toml)

find_program(CBINDGEN cbindgen)

include_directories(${CLIENT_BUILD_DIR})

add_subdirectory(examples/cpp)
