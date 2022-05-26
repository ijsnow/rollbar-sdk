set(CMAKE_CXX_STANDARD 14)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(ROLLBAR_BUILD_DIR ${CMAKE_BINARY_DIR}/rollbar)
include_directories(${ROLLBAR_BUILD_DIR})

set(SOURCE main.cpp)
add_executable(example-cpp ${SOURCE})
get_target_property(CLIENT_DIR rollbar LOCATION)
target_link_libraries(example-cpp ${CLIENT_DIR}/librollbar.a)
add_dependencies(example-cpp rollbar)
