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

void printAsMatrix(auto name, auto array, auto rowSize, auto colSize) {
  const auto maxNumAsStringSize = std::to_string(array.back()).size();
  std::cout << "\nMatrix " << name << ":\n";
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

} // namespace

int main() {
  constexpr auto ROW_SIZE = IS_PROFILING ? 1024 * 16U : 8U;
  constexpr auto COLUMN_SIZE = IS_PROFILING ? 1024 * 16U : 8U;
  constexpr auto TOTAL_SIZE = ROW_SIZE * COLUMN_SIZE;
  constexpr auto TILE_SIZE = IS_PROFILING ? 16U : 4U;

  const auto OCL_DATA_TYPE = ocl::dataTypeFromType<DATA_TYPE>();
  const std::vector<DATA_TYPE> data = [] {
    std::vector<DATA_TYPE> vec(TOTAL_SIZE);
    std::iota(vec.begin(), vec.end(), 0);
    return vec;
  }();

  struct Result {
    std::string name;
    std::vector<DATA_TYPE> data;
    bool transpose = true;
    TIME_TYPE executionTime;
  };

  std::vector<Result> results;

  //
  // COPY NAIVE
  //
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

  //
  // COPY VECTORED
  //
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

  //
  // MATRIX TRANSPOSE NAIVE
  //
  {
    std::vector<DATA_TYPE> result(TOTAL_SIZE);
    ocl::Engine engine("matrix_transpose_naive", {ROW_SIZE, COLUMN_SIZE});
    engine.setData(data.data(), result.data(), TOTAL_SIZE, OCL_DATA_TYPE);
    engine.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
    engine.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
    if (IS_PROFILING) {
      engine.setLocalWorkSizes({TILE_SIZE, TILE_SIZE});
      engine.enableProfiling();
    }
    engine.run();
    results.emplace_back("transpose naive", result);
    if (IS_PROFILING) {
      results.back().executionTime = engine.getExecutionTime();
    }
  }

  //
  // MATRIX TRANSPOSE TILED ON READ
  //
  {
    std::vector<DATA_TYPE> result(TOTAL_SIZE);
    ocl::Engine engine("matrix_transpose_tiled", {ROW_SIZE, COLUMN_SIZE / TILE_SIZE});
    engine.setData(data.data(), result.data(), TOTAL_SIZE, OCL_DATA_TYPE);
    engine.setLocalWorkSizes({TILE_SIZE, 1});
    engine.addCompilerOptionDefine("TILE_SIZE", TILE_SIZE);
    engine.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
    engine.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
    if (IS_PROFILING) {
      engine.enableProfiling();
    }
    engine.run();
    results.emplace_back("transpose tiled on read", result);
    if (IS_PROFILING) {
      results.back().executionTime = engine.getExecutionTime();
    }
  }

  //
  // MATRIX TRANSPOSE TILED ON WRITE
  //
  {
    std::vector<DATA_TYPE> result(TOTAL_SIZE);
    ocl::Engine engine("matrix_transpose_tiled", {ROW_SIZE, COLUMN_SIZE / TILE_SIZE});
    engine.setData(data.data(), result.data(), TOTAL_SIZE, OCL_DATA_TYPE);
    engine.setLocalWorkSizes({TILE_SIZE, 1});
    engine.addCompilerOptionDefine("TILE_SIZE", TILE_SIZE);
    engine.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
    engine.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
    engine.addCompilerOptionDefine("TRANSPOSE_ON_TILE_WRITE");
    if (IS_PROFILING) {
      engine.enableProfiling();
    }
    engine.run();
    results.emplace_back("transpose tiled on write", result);
    if (IS_PROFILING) {
      results.back().executionTime = engine.getExecutionTime();
    }
  }

  //
  // MATRIX TRANSPOSE TILED VECTORED
  //
  {
    std::vector<DATA_TYPE> result(TOTAL_SIZE);
    ocl::Engine engine("matrix_transpose_tiled_vectored", {ROW_SIZE, COLUMN_SIZE / TILE_SIZE});
    engine.setData(data.data(), result.data(), TOTAL_SIZE, OCL_DATA_TYPE);
    engine.setLocalWorkSizes({TILE_SIZE, 1});
    engine.addCompilerOptionDefine("TILE_SIZE", TILE_SIZE);
    engine.addCompilerOptionDefine("VEC_SIZE", TILE_SIZE);
    engine.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
    engine.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
    if (IS_PROFILING) {
      engine.enableProfiling();
    }
    engine.run();
    results.emplace_back("transpose tiled vectored", result);
    if (IS_PROFILING) {
      results.back().executionTime = engine.getExecutionTime();
    }
  }

  //
  // FINAL RESULTS
  //
  printResults(results, ROW_SIZE, COLUMN_SIZE, IS_PROFILING);
  compareResults(results);
}
