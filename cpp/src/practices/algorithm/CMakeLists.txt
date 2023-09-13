add_executable(deduplicate.test deduplicate.test.cpp)
target_link_libraries(deduplicate.test GTest::gtest GTest::gtest_main common)

add_executable(unique.test unique.test.cpp)
target_link_libraries(unique.test GTest::gtest GTest::gtest_main common)

add_executable(sort.test sort.test.cpp)
target_link_libraries(sort.test GTest::gtest GTest::gtest_main common)

add_executable(shortest_matching_substring.test shortest_matching_substring.test.cpp)
target_link_libraries(shortest_matching_substring.test GTest::gtest GTest::gtest_main common)
