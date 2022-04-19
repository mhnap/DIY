#pragma once

#include <string>

namespace ocl {

enum class DataType { Char, Short, Int, Long, Float };

std::string dataTypeToString(DataType dataType);
size_t dataTypeToSize(DataType dataType);

template <typename T>
DataType dataTypeFromType();

} // namespace ocl