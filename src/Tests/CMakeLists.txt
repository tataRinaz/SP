add_executable(Tests
               Tokenizer.cpp
)

target_include_directories(Tests
    PRIVATE ..
    PRIVATE ${PROJECT_SOURCE_DIR}/src/Tokenizer
    PRIVATE ${CONAN_INCLUDE_DIRS_CATCH2}
)

target_link_libraries(Tests
    Tokenizer
)

add_test(NAME Tests COMMAND Tests)