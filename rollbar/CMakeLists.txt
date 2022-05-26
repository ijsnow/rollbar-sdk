if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

set(ROLLBAR_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/librollbar.a")

add_custom_target(rollbar ALL
    COMMENT "Compiling rollbar module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    COMMAND cp ${ROLLBAR_LIB} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

set_target_properties(rollbar PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_test(NAME rollbar_test 
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
