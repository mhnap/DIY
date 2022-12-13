add_library(ocl engine.hpp engine.cpp data_types.hpp data_types.cpp utils.hpp error.hpp)
find_library(OPENCL OpenCL)
target_link_libraries(ocl ${OPENCL})