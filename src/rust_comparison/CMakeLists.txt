add_compile_options(-Wall -Wextra -pedantic -Werror)
add_compile_options(-fsanitize=address)
add_link_options(-fsanitize=address)

add_executable(use_after_free use_after_free.cpp)
add_executable(global_format_state global_format_state.cpp)
add_executable(uniform_initialization uniform_initialization.cpp)
add_executable(const_initialization const_initialization.cpp)
add_executable(data_race data_race.cpp)
