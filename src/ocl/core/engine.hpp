#pragma once

#include "data_types.hpp"
#include <optional>
#include <string>
#include <string_view>
#include <vector>

namespace ocl {

class Engine {
public:
  Engine(std::string_view kernelName, std::vector<size_t> globalWorkSizes);
  void setLocalWorkSizes(std::vector<size_t> localWorkSizes);
  void setData(const void* input, void* output, size_t size, DataType type);
  void addCompilerOption(std::string_view option);
  void addCompilerDefineOption(std::string_view name, std::string_view definition);
  void run();

  struct Data {
    const void* input;
    void* output;
    size_t size;
    DataType type;
  };

private:
  std::string getKernelFilePath() const;
  std::string loadKernelSource() const;

  std::string m_kernelName;
  std::vector<size_t> m_globalWorkSizes;

  std::vector<size_t> m_localWorkSizes;
  std::optional<Data> m_data;
  std::string m_compilerOptions;
};

} // namespace ocl