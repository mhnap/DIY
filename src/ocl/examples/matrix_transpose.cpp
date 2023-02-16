#include "ocl/core/engine.hpp"
#include <iomanip>
#include <iostream>
#include <numeric>
#include <vector>

#ifdef NDEBUG
constexpr bool isProfiling = true;
#else
constexpr bool isProfiling = false;
#endif

namespace {
std::result_of<decltype (&ocl::Engine::getExecutionTime)(ocl::Engine)>::type targetTime;
}

void printAsMatrix(auto name, const auto& array, auto rowSize, auto colSize) {
  const auto maxNumAsStringSize = std::to_string(array.back()).size();
  std::cout << "\nMatrix transposed " << name << ":\n";
  for (auto i = 0U; i < rowSize; ++i) {
    for (auto j = 0U; j < colSize; ++j) {
      std::cout << std::setw(maxNumAsStringSize) << array[i * colSize + j] << ' ';
    }
    std::cout << '\n';
  }
}

void printExecutionTime(auto name, auto executionTime) {
  std::cout << "Transposed " << name << " execution time: " << executionTime << ' '
            << targetTime * 100 / executionTime << "%\n";
}

int main() {
  constexpr auto ROW_SIZE = isProfiling ? 1024 * 8U : 8U;
  constexpr auto COLUMN_SIZE = isProfiling ? 1024 * 8U : 8U;
  constexpr auto TOTAL_SIZE = ROW_SIZE * COLUMN_SIZE;

  const std::vector<int> data = [&] {
    auto vec = decltype(data)(TOTAL_SIZE);
    std::iota(vec.begin(), vec.end(), 1);
    return vec;
  }();
  if (not isProfiling) {
    printAsMatrix("original", data, ROW_SIZE, COLUMN_SIZE);
  }

  const auto dataType = ocl::dataTypeFromType<decltype(data)::value_type>();
  constexpr auto TILE_SIZE = isProfiling ? 16U : 4U;

  //
  // MATRIX_TRANSPOSE_NAIVE
  //
  auto resultNaive = decltype(data)(TOTAL_SIZE);
  ocl::Engine engineNaive("matrix_transpose_naive", {ROW_SIZE, COLUMN_SIZE});
  engineNaive.setData(data.data(), resultNaive.data(), TOTAL_SIZE, dataType);
  if (isProfiling) {
    engineNaive.setLocalWorkSizes({16, 16});
  }
  engineNaive.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
  engineNaive.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
  if (isProfiling) {
    engineNaive.enableProfiling();
  }
  engineNaive.run();
  if (isProfiling) {
    targetTime = engineNaive.getExecutionTime();
    printExecutionTime("naive", targetTime);
  } else {
    printAsMatrix("naive", resultNaive, COLUMN_SIZE, ROW_SIZE);
  }

  //
  // MATRIX TRANSPOSE TILED ON READ
  //
  auto resultTiledOnRead = decltype(data)(TOTAL_SIZE);
  ocl::Engine engineTiledOnRead("matrix_transpose_tiled", {ROW_SIZE, COLUMN_SIZE / TILE_SIZE});
  engineTiledOnRead.setData(data.data(), resultTiledOnRead.data(), TOTAL_SIZE, dataType);
  engineTiledOnRead.setLocalWorkSizes({TILE_SIZE, 1});
  engineTiledOnRead.addCompilerOptionDefine("TILE_SIZE", TILE_SIZE);
  engineTiledOnRead.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
  engineTiledOnRead.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
  if (isProfiling) {
    engineTiledOnRead.enableProfiling();
  }
  engineTiledOnRead.run();
  if (isProfiling) {
    printExecutionTime("tiled on read", engineTiledOnRead.getExecutionTime());
  } else {
    printAsMatrix("tiled on read", resultTiledOnRead, COLUMN_SIZE, ROW_SIZE);
  }

  //
  // MATRIX TRANSPOSE TILED ON WRITE
  //
  auto resultTiledOnWrite = decltype(data)(TOTAL_SIZE);
  ocl::Engine engineTiledOnWrite("matrix_transpose_tiled", {ROW_SIZE, COLUMN_SIZE / TILE_SIZE});
  engineTiledOnWrite.setData(data.data(), resultTiledOnWrite.data(), TOTAL_SIZE, dataType);
  engineTiledOnWrite.setLocalWorkSizes({TILE_SIZE, 1});
  engineTiledOnWrite.addCompilerOptionDefine("TILE_SIZE", TILE_SIZE);
  engineTiledOnWrite.addCompilerOptionDefine("ROW_SIZE", ROW_SIZE);
  engineTiledOnWrite.addCompilerOptionDefine("COLUMN_SIZE", COLUMN_SIZE);
  engineTiledOnWrite.addCompilerOptionDefine("TRANSPOSE_ON_TILE_WRITE");
  if (isProfiling) {
    engineTiledOnWrite.enableProfiling();
  }
  engineTiledOnWrite.run();
  if (isProfiling) {
    printExecutionTime("tiled on write", engineTiledOnWrite.getExecutionTime());
  } else {
    printAsMatrix("tiled on write", resultTiledOnWrite, COLUMN_SIZE, ROW_SIZE);
  }

  //
  // FINAL RESULTS
  //
  const bool isAllTransposedMatricesAreEqual =
      (resultNaive == resultTiledOnRead and resultNaive == resultTiledOnWrite);
  if (not isProfiling or not isAllTransposedMatricesAreEqual) {
    std::cout << "\nTransposed matrices are equal: " << std::boolalpha << isAllTransposedMatricesAreEqual
              << '\n';
  }
}
