// https://en.cppreference.com/w/cpp/language/ub

#include <iostream>

int main() {
  {
    // Multiple unsequenced modifications
    // Can be seen by "-Werror=sequence-point"(gcc) or "-Wunsequenced"(clang)
    // https://en.cppreference.com/w/cpp/language/eval_order
    int i = 0;
    i = ++i + 2; // well-defined
    i = i++ + 2; // UB until C++17
    i = ++i + i++; // UB
    i = ++i + i; // UB
    std::cout << i;
  }

  {
    // Signed integer arithmetic operation overflow
    // https://en.cppreference.com/w/cpp/language/operator_arithmetic
    int i = 0;
    i = i + 1; // can be UB due to signed overflow
  }
}
