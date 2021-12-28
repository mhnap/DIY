add_executable(diagonal_iteration diagonal_iteration.cpp)
add_executable(circular_iteration circular_iteration.cpp)
add_executable(rotate rotate.cpp)

target_link_libraries(rotate PRIVATE common)