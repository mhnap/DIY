#include "data_types.hpp"
#include <stdexcept>

namespace ocl {

std::string dataTypeToString(DataType dataType) {
  std::string dataTypeStr;
  if (dataType == DataType::Char) {
    dataTypeStr = "char";
  } else if (dataType == DataType::Short) {
    dataTypeStr = "short";
  } else if (dataType == DataType::Int) {
    dataTypeStr = "int";
  } else if (dataType == DataType::Long) {
    dataTypeStr = "long";
  } else if (dataType == DataType::Float) {
    dataTypeStr = "float";
  } else {
    throw std::runtime_error("Unknown DataType value.");
  }
  return dataTypeStr;
}

size_t dataTypeToSize(DataType dataType) {
  size_t dataTypeSize;
  if (dataType == DataType::Char) {
    dataTypeSize = sizeof(char);
  } else if (dataType == DataType::Short) {
    dataTypeSize = sizeof(short);
  } else if (dataType == DataType::Int) {
    dataTypeSize = sizeof(int);
  } else if (dataType == DataType::Long) {
    dataTypeSize = sizeof(long);
  } else if (dataType == DataType::Float) {
    dataTypeSize = sizeof(float);
  } else {
    throw std::runtime_error("Unknown DataType value.");
  }
  return dataTypeSize;
}

template <>
DataType dataTypeFromType<char>() {
  return DataType::Char;
}

template <>
DataType dataTypeFromType<short>() {
  return DataType::Short;
}

template <>
DataType dataTypeFromType<int>() {
  return DataType::Int;
}

template <>
DataType dataTypeFromType<long>() {
  return DataType::Long;
}

template <>
DataType dataTypeFromType<float>() {
  return DataType::Float;
}

} // namespace ocl