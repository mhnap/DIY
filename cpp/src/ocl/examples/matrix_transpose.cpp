#include "ocl/core/engine.hpp"
#include <iomanip>
#include <iostream>
#include <numeric>
#include <vector>

namespace {

#ifdef NDEBUG
constexpr bool IS_PROFILING = true;
#else
constexpr bool IS_PROFILING = false;
#endif

using DATA_TYPE = int;
using TIME_TYPE = std::result_of<decltype (&ocl::Engine::getExecutionTime)(ocl::Engine)>::type;

enum class IterationType { ROW_WISE, COLUMN_WISE };
enum class TransposeType { ON_TILE_WRITE, ON_TILE_READ };

struct Result {
  std::string name;
  std::vector<DATA_TYPE> data;
  bool transpose = true;
  TIME_TYPE executionTime;
};

constexpr auto ROW_SIZE = IS_PROFILING ? 1024 * 8U : 8U;
constexpr auto COLUMN_SIZE = IS_PROFILING ? 1024 * 8U : 8U;
constexpr auto TOTAL_SIZE = ROW_SIZE * COLUMN_SIZE;
constexpr auto TILE_SIZE = IS_PROFILING ? 16U : 4U;
constexpr auto VEC_SIZE = TILE_SIZE;
const auto OCL_DATA_TYPE = ocl::dataTypeFromType<DATA_TYPE>();

void printAsMatrix(auto name, auto array, auto rowSize, auto colSize) {
  const auto maxNumAsStringSize = std::to_string(array.back()).size();
  std::cout << "\nData " << name << ":\n";
  for (auto i = 0U; i < rowSize; ++i) {
    for (auto j = 0U; j < colSize; ++j) {
      std::cout << std::setw(maxNumAsStringSize) << array[i * colSize + j] << ' ';
    }
    std::cout << '\n';
  }
}

void printExecutionTime(auto name, auto targetTime, auto executionTime) {
  std::cout << "Total " << name << " execution time: " << executionTime << ' '
            << targetTime * 100 / executionTime << "%\n";
}

void printResults(const auto& results, auto rowSize, auto colSize, bool isProfiling) {
  for (const auto& result : results) {
    if (isProfiling) {
      printExecutionTime(result.name, results.front().executionTime, result.executionTime);
    } else {
      printAsMatrix(result.name, result.data, rowSize, colSize);
    }
  }
}

void compareResults(auto& results) {
  bool isAllTransposedMatricesAreEqual = true;
  std::vector<DATA_TYPE>* tmpData = nullptr;
  for (auto& result : results) {
    if (result.transpose) {
      if (tmpData == nullptr) {
        tmpData = &result.data;
      } else if (*tmpData != result.data) {
        isAllTransposedMatricesAreEqual = false;
        break;
      }
    }
  }
  if (not IS_PROFILING or not isAllTransposedMatricesAreEqual) {
    std::cout << "\nTransposed matrices are equal: " << std::boolalpha << isAllTransposedMatricesAreEqual;
  }
}

void runKernels(auto& results, const auto& input, auto kernelName, std::vector<size_t>&& gws,
                std::vector<size_t>&& lws, bool isTiled = true, bool isVectored = false) {
  std::vector transposeTypes{TransposeType::ON_TILE_WRITE, TransposeType::ON_TILE_READ};
  std::vector iterationTypes{IterationType::ROW_WISE, IterationType::COLUMN_WISE};
  if (not isTiled) {
    transposeTypes = {TransposeType::ON_TILE_WRITE};
    iterationTypes = {IterationType::ROW_WISE};
  }
  if (isVectored) {
    iterationTypes = {IterationType::ROW_WISE};
  }
  for (auto iterationType : iterationTypes) {
    for (auto transposeType : transposeTypes) {
      std::string prettyName = kernelName;
      std::vector<DATA_TYPE> output(TOTAL_SIZE);
      ocl::Engine engine(kernelName, gws);
      engine.setData(input.data(), output.data(), TOTAL_SIZE, OCL_DATA_TYPE);
      engine.setLocalWorkSizes(lws);
      engine.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
      engine.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
      if (isVectored) {
        engine.addCompilerOptionDefine("VEC_SIZE", VEC_SIZE);
      }
      if (isTiled) {
        engine.addCompilerOptionDefine("TILE_SIZE", TILE_SIZE);
        if (transposeType == TransposeType::ON_TILE_WRITE) {
          engine.addCompilerOptionDefine("TRANSPOSE_ON_TILE_WRITE");
          prettyName += " on write";
        } else {
          prettyName += " on read";
        }
      }
      if (iterationType == IterationType::ROW_WISE) {
        engine.addCompilerOptionDefine("ROW_WISE");
        prettyName += " row-wise";
      } else {
        prettyName += " column-wise";
      }
      if (IS_PROFILING) {
        engine.enableProfiling();
      }
      engine.run();
      results.emplace_back(prettyName, std::move(output));
      if (IS_PROFILING) {
        results.back().executionTime = engine.getExecutionTime();
      }
    }
  }
}

} // namespace

int main() {
  const std::vector<DATA_TYPE> data = [] {
    std::vector<DATA_TYPE> vec(TOTAL_SIZE);
    std::iota(vec.begin(), vec.end(), 0);
    return vec;
  }();
  std::vector<Result> results;

  {
    std::vector<DATA_TYPE> result(TOTAL_SIZE);
    ocl::Engine engine("copy_naive", {TOTAL_SIZE});
    engine.setData(data.data(), result.data(), TOTAL_SIZE, OCL_DATA_TYPE);
    if (IS_PROFILING) {
      engine.enableProfiling();
    }
    engine.run();
    results.emplace_back("copy naive", std::move(result), false);
    if (IS_PROFILING) {
      results.back().executionTime = engine.getExecutionTime();
    }
  }
  {
    std::vector<DATA_TYPE> result(TOTAL_SIZE);
    ocl::Engine engine("copy_vectored", {TOTAL_SIZE / TILE_SIZE});
    engine.setData(data.data(), result.data(), TOTAL_SIZE, OCL_DATA_TYPE);
    engine.addCompilerOptionDefine("VEC_SIZE", TILE_SIZE);
    if (IS_PROFILING) {
      engine.enableProfiling();
    }
    engine.run();
    results.emplace_back("copy vectored", std::move(result), false);
    if (IS_PROFILING) {
      results.back().executionTime = engine.getExecutionTime();
    }
  }

  runKernels(results, data, "matrix_transpose_naive", {ROW_SIZE, COLUMN_SIZE}, {TILE_SIZE, TILE_SIZE}, false);
  runKernels(results, data, "matrix_transpose_tiled_per_elem", {ROW_SIZE, COLUMN_SIZE},
             {TILE_SIZE, TILE_SIZE});
  runKernels(results, data, "matrix_transpose_tiled_per_row", {ROW_SIZE, COLUMN_SIZE / TILE_SIZE},
             {TILE_SIZE, 1});
  runKernels(results, data, "matrix_transpose_tiled_per_row_vectored", {ROW_SIZE, COLUMN_SIZE / TILE_SIZE},
             {TILE_SIZE, 1}, true, true);
  runKernels(results, data, "matrix_transpose_tiled_per_column", {ROW_SIZE / TILE_SIZE, COLUMN_SIZE},
             {1, TILE_SIZE});
  runKernels(results, data, "matrix_transpose_tiled_per_column_vectored", {ROW_SIZE / TILE_SIZE, COLUMN_SIZE},
             {1, TILE_SIZE}, true, true);

  // FINAL RESULTS
  printResults(results, ROW_SIZE, COLUMN_SIZE, IS_PROFILING);
  compareResults(results);
}
