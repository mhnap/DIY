#pragma once

#include "data_types.hpp"
#include <array>
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
  void addCompilerOptionDefine(std::string_view name);
  void addCompilerOptionDefine(std::string_view name, std::string_view definition);
  void addCompilerOptionIncludeDirectory(std::string_view dir);
  void run();

  struct Data {
    const void* input;
    void* output;
    size_t size;
    DataType type;
  };

private:
  [[nodiscard]] std::string getKernelFilePath() const;
  [[nodiscard]] std::string loadKernelSource() const;
  void addCompilerOptionDefaultIncludeDirectories();

  std::string m_kernelName;
  std::vector<size_t> m_globalWorkSizes;
  std::vector<size_t> m_localWorkSizes;
  std::optional<Data> m_data;
  std::string m_compilerOptions;

  [[nodiscard]] static std::string getKernelsDirPath();
  static constexpr std::array s_defaultIncludeDirectories = {"include"};
};

} // namespace ocl