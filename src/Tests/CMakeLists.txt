add_executable(Tests
               Tokenizer.cpp
               Parser.cpp
)

target_include_directories(Tests
    PRIVATE ..
    PRIVATE ${PROJECT_SOURCE_DIR}/src/Parser
    PRIVATE ${CONAN_INCLUDE_DIRS_CATCH2}
)

target_link_libraries(Tests
    Parser
)

add_test(NAME Tests COMMAND Tests)