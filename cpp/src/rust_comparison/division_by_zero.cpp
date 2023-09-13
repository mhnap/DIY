#include <cmath>
#include <iostream>

int get_zero() { return 0; }

int main() {
  {
    // Floating point division by zero will produce an infinity value.
    float a = 1;
    float b = 0;
    float c = a / b;
    std::cout << "c=" << c << std::endl;
  }

  {
    // Or will produce NAN if `a` is NAN.
    float a = NAN;
    float b = 0;
    float c = a / b;
    std::cout << "c=" << c << std::endl;
  }

  {
    // Or will produce INFINITY if `a` is INFINITY.
    float a = INFINITY;
    float b = 0;
    float c = a / b;
    std::cout << "c=" << c << std::endl;
  }

  {
    // Or will produce -INFINITY if `a` is -INFINITY.
    float a = -INFINITY;
    float b = 0;
    float c = a / b;
    std::cout << "c=" << c << std::endl;
  }

  //

  {
      // Compiler can detect integer literals division by zero.
      // int a = 1 / 0;
      // error: division by zero [-Werror=div-by-zero]
      // std::cout << "a=" << a << std::endl;
  }

  {
      // Compiler can detect constexpr integer variables division by zero.
      // constexpr int a = 1;
      // constexpr int b = 0;
      // error: division by zero [-Werror=div-by-zero]
      // constexpr int c = a / b;
      // std::cout << "c=" << c << std::endl;
  }

  {
    // There is no checks for runtime integer division by zero.
    int a = 1;
    int b = get_zero();
    int c = a / b;
    std::cout << "c=" << c << std::endl;
    // Process finished with exit code 136 (interrupted by signal 8: SIGFPE)
  }
}
