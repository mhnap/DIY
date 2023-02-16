#include "ocl/core/engine.hpp"
#include <iomanip>
#include <iostream>
#include <numeric>
#include <span>
#include <vector>

namespace {

#ifdef NDEBUG
constexpr bool IS_PROFILING = true;
#else
constexpr bool IS_PROFILING = false;
#endif

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

void printResults(const auto& results, auto rowSize, auto colSize) {
  for (const auto& result : results) {
    if (IS_PROFILING) {
      printExecutionTime(result.name, results.front().engine.getExecutionTime(),
                         result.engine.getExecutionTime());
    } else {
      printAsMatrix(result.name, result.data, rowSize, colSize);
    }
  }
}

} // namespace

int main() {
  constexpr auto ROW_SIZE = IS_PROFILING ? 1024 * 16U : 8U;
  constexpr auto COLUMN_SIZE = IS_PROFILING ? 1024 * 16U : 8U;
  constexpr auto TOTAL_SIZE = ROW_SIZE * COLUMN_SIZE;
  constexpr auto TILE_SIZE = IS_PROFILING ? 16U : 4U;

  using DATA_TYPE = int;
  const auto OCL_DATA_TYPE = ocl::dataTypeFromType<DATA_TYPE>();
  const std::vector<DATA_TYPE> data = [] {
    auto vec = decltype(data)(TOTAL_SIZE);
    std::iota(vec.begin(), vec.end(), 0);
    return vec;
  }();

  struct Result {
    std::string_view name;
    std::span<DATA_TYPE> data;
    ocl::Engine& engine;
  };

  std::vector<Result> results;

  //
  // COPY NAIVE
  //
  auto resultCopyNaive = decltype(data)(TOTAL_SIZE);
  ocl::Engine engineCopyNaive("copy_naive", {TOTAL_SIZE});
  engineCopyNaive.setData(data.data(), resultCopyNaive.data(), TOTAL_SIZE, OCL_DATA_TYPE);
  if (IS_PROFILING) {
    engineCopyNaive.enableProfiling();
  }
  engineCopyNaive.run();
  results.push_back({"copy naive", resultCopyNaive, engineCopyNaive});

  //
  // COPY VECTORED
  //
  auto resultCopyVectored = decltype(data)(TOTAL_SIZE);
  ocl::Engine engineCopyVectored("copy_vectored", {TOTAL_SIZE / TILE_SIZE});
  engineCopyVectored.setData(data.data(), resultCopyVectored.data(), TOTAL_SIZE, OCL_DATA_TYPE);
  engineCopyVectored.addCompilerOptionDefine("VEC_SIZE", TILE_SIZE);
  if (IS_PROFILING) {
    engineCopyVectored.enableProfiling();
  }
  engineCopyVectored.run();
  results.push_back({"copy vectored", resultCopyVectored, engineCopyVectored});

  //
  // MATRIX TRANSPOSE NAIVE
  //
  auto resultNaive = decltype(data)(TOTAL_SIZE);
  ocl::Engine engineNaive("matrix_transpose_naive", {ROW_SIZE, COLUMN_SIZE});
  engineNaive.setData(data.data(), resultNaive.data(), TOTAL_SIZE, OCL_DATA_TYPE);
  engineNaive.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
  engineNaive.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
  if (IS_PROFILING) {
    engineNaive.setLocalWorkSizes({TILE_SIZE, TILE_SIZE});
    engineNaive.enableProfiling();
  }
  engineNaive.run();
  results.push_back({"transpose naive", resultNaive, engineNaive});

  //
  // MATRIX TRANSPOSE TILED ON READ
  //
  auto resultTiledOnRead = decltype(data)(TOTAL_SIZE);
  ocl::Engine engineTiledOnRead("matrix_transpose_tiled", {ROW_SIZE, COLUMN_SIZE / TILE_SIZE});
  engineTiledOnRead.setData(data.data(), resultTiledOnRead.data(), TOTAL_SIZE, OCL_DATA_TYPE);
  engineTiledOnRead.setLocalWorkSizes({TILE_SIZE, 1});
  engineTiledOnRead.addCompilerOptionDefine("TILE_SIZE", TILE_SIZE);
  engineTiledOnRead.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
  engineTiledOnRead.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
  if (IS_PROFILING) {
    engineTiledOnRead.enableProfiling();
  }
  engineTiledOnRead.run();
  results.push_back({"transpose tiled on read", resultTiledOnRead, engineTiledOnRead});

  //
  // MATRIX TRANSPOSE TILED ON WRITE
  //
  auto resultTiledOnWrite = decltype(data)(TOTAL_SIZE);
  ocl::Engine engineTiledOnWrite("matrix_transpose_tiled", {ROW_SIZE, COLUMN_SIZE / TILE_SIZE});
  engineTiledOnWrite.setData(data.data(), resultTiledOnWrite.data(), TOTAL_SIZE, OCL_DATA_TYPE);
  engineTiledOnWrite.setLocalWorkSizes({TILE_SIZE, 1});
  engineTiledOnWrite.addCompilerOptionDefine("TILE_SIZE", TILE_SIZE);
  engineTiledOnWrite.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
  engineTiledOnWrite.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
  engineTiledOnWrite.addCompilerOptionDefine("TRANSPOSE_ON_TILE_WRITE");
  if (IS_PROFILING) {
    engineTiledOnWrite.enableProfiling();
  }
  engineTiledOnWrite.run();
  results.push_back({"transpose tiled on write", resultTiledOnWrite, engineTiledOnWrite});

  //
  // MATRIX TRANSPOSE TILED VECTORED
  //
  auto resultTiledVectored = decltype(data)(TOTAL_SIZE);
  ocl::Engine engineTiledVectored("matrix_transpose_tiled_vectored", {ROW_SIZE, COLUMN_SIZE / TILE_SIZE});
  engineTiledVectored.setData(data.data(), resultTiledVectored.data(), TOTAL_SIZE, OCL_DATA_TYPE);
  engineTiledVectored.setLocalWorkSizes({TILE_SIZE, 1});
  engineTiledVectored.addCompilerOptionDefine("TILE_SIZE", TILE_SIZE);
  engineTiledVectored.addCompilerOptionDefine("VEC_SIZE", TILE_SIZE);
  engineTiledVectored.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
  engineTiledVectored.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
  if (IS_PROFILING) {
    engineTiledVectored.enableProfiling();
  }
  engineTiledVectored.run();
  results.push_back({"transpose tiled vectored", resultTiledVectored, engineTiledVectored});

  //
  // FINAL RESULTS
  //
  printResults(results, ROW_SIZE, COLUMN_SIZE);

  const bool isAllCopyMatricesAreEqual = (data == resultCopyNaive and data == resultCopyVectored);
  if (not IS_PROFILING or not isAllCopyMatricesAreEqual) {
    std::cout << "\nCopy matrices are equal: " << std::boolalpha << isAllCopyMatricesAreEqual;
  }
  const bool isAllTransposedMatricesAreEqual =
      (resultNaive == resultTiledOnRead and resultNaive == resultTiledOnWrite and
       resultNaive == resultTiledVectored);
  if (not IS_PROFILING or not isAllTransposedMatricesAreEqual) {
    std::cout << "\nTransposed matrices are equal: " << std::boolalpha << isAllTransposedMatricesAreEqual;
  }
}
