add_library(ocl engine.hpp engine.cpp data_types.hpp data_types.cpp utils.hpp)
target_link_libraries(ocl "-framework OpenCL")