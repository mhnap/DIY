#include "ocl/core/engine.hpp"
#include <array>
#include <iostream>
#include <numeric>

int main() {
  constexpr auto SIZE = 64U;
  constexpr std::array<int, SIZE> data = [&] {
    auto arr = decltype(data){};
    std::iota(arr.begin(), arr.end(), 0);
    return arr;
  }();

  auto validate_results = [&](const auto& results) {
    std::cout << "Result: ";
    auto correct = 0U;
    for (auto i = 0U; i < SIZE; ++i) {
      if (results[i] == data[i]) {
        ++correct;
      }
      std::cout << results[i] << ' ';
    }
    std::cout << "\nComputed " << correct << '/' << SIZE << " correct values!\n";
  };

  {
    // Copy data by running naive OpenCL kernel
    auto results = decltype(data){};
    ocl::Engine engine("copy_naive", {SIZE});
    engine.setData(&data, &results, SIZE, ocl::dataTypeFromType<decltype(data)::value_type>());
    engine.run();
    validate_results(results);
  }

  {
    // Copy data by running vectored OpenCL kernel
    // Note, that need to correctly handle remainder using ceiling division
    constexpr auto vec_size = 16U;
    constexpr auto quotient = SIZE / vec_size;
    constexpr auto remainder = SIZE % vec_size;
    constexpr auto work_size = remainder == 0 ? quotient : quotient + 1;

    auto results = decltype(data){};
    ocl::Engine engine("copy_vectored", {work_size});
    engine.setData(&data, &results, SIZE, ocl::dataTypeFromType<decltype(data)::value_type>());
    engine.addCompilerOptionDefine("VEC_SIZE", vec_size);
    if (remainder != 0U) {
      engine.addCompilerOptionDefine("REMAINDER_SIZE", remainder);
    }
    engine.run();
    validate_results(results);
  }

  {
    // Copy data by running naive with subgroup OpenCL kernel
    auto results = decltype(data){};
    ocl::Engine engine("copy_naive_subgroup", {SIZE});
    engine.setData(&data, &results, SIZE, ocl::dataTypeFromType<decltype(data)::value_type>());
    engine.setLocalWorkSizes({32});
    engine.addCompilerOptionDefine("SUB_GROUP_SIZE", 16);
    engine.run();
    validate_results(results);
  }
}
