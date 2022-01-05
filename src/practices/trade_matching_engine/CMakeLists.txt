add_executable(trade_matching_engine.test trade_matching_engine.test.cpp)
target_link_libraries(trade_matching_engine.test GTest::gtest GTest::gtest_main)

add_executable(main main.cpp)