#pragma once

#include "data_types.hpp"
#include <array>
#include <chrono>
#include <concepts>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

namespace ocl {

class Engine {
public:
  Engine(std::string_view kernelName, std::vector<size_t>&& globalWorkSizes);
  void setLocalWorkSizes(std::vector<size_t>&& localWorkSizes);
  void setData(const void* input, void* output, size_t size, DataType type);
  void addCompilerOption(std::string_view option);
  void addCompilerOptionDefine(std::string_view name);
  void addCompilerOptionDefine(std::string_view name, std::string_view definition);

  template <typename T>
    requires(not std::convertible_to<T, std::string_view>)
  void addCompilerOptionDefine(std::string_view name, T definition) {
    addCompilerOptionDefine(name, std::to_string(definition));
  }

  void addCompilerOptionIncludeDirectory(std::string_view dir);
  void enableProfiling();
  [[nodiscard]] std::chrono::nanoseconds getExecutionTime() const;
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
  bool m_isProfilingEnabled = false;
  std::chrono::nanoseconds m_executionTime;

  [[nodiscard]] static std::string getKernelsDirPath();
  static constexpr std::array s_defaultIncludeDirectories = {"include"};
};

} // namespace ocl