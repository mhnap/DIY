add_library(ocl engine.hpp engine.cpp data_types.hpp data_types.cpp utils.hpp error.hpp)
if (APPLE)
    target_link_libraries(ocl PRIVATE "-framework OpenCL")
else ()
    target_link_libraries(ocl PRIVATE OpenCL::OpenCL)
endif ()
