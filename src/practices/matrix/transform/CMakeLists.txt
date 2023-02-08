add_executable(rotate.test rotate.test.cpp)
add_executable(transpose.test transpose.test.cpp)

target_link_libraries(rotate.test GTest::gtest GTest::gtest_main ocl)
target_link_libraries(transpose.test GTest::gtest GTest::gtest_main ocl)
