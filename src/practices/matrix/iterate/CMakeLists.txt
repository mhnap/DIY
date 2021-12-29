add_executable(diagonal.test diagonal.test.cpp)
add_executable(spiral.test spiral.test.cpp)

target_link_libraries(diagonal.test GTest::gtest GTest::gtest_main)
target_link_libraries(spiral.test GTest::gtest GTest::gtest_main)