#include <iostream>

// C++ doesn't guarantee that there is no reference aliasing.

void add(int& a, const int& b) {
  a += b;
  a += b;
}
// Above function produces the following assembly with optimizations turned on.
// add(int&, int const&):
//        mov     eax, DWORD PTR [rsi]
//        add     eax, DWORD PTR [rdi]
//        mov     DWORD PTR [rdi], eax
//        add     eax, DWORD PTR [rsi]
//        mov     DWORD PTR [rdi], eax
//        ret
// Here, we don't know that `a` cannot alias `b`, so we read `b` twice.

// This logic can be changed by manually annotating `b` with __restrict__ type qualifier.
void add_restrict(int& a, const int& __restrict__ b) {
  a += b;
  a += b;
}
// Above function produces the following assembly with optimizations turned on.
// add(int&, int const&):
//        mov     eax, DWORD PTR [rsi]
//        add     eax, eax
//        add     DWORD PTR [rdi], eax
//        ret
// Here, we know that `a` cannot alias `b`, so we read `b` only once.

int main() {
  int a = 42;
  // Aliasing don't cause a compiler error.
  add(a, a);
  std::cout << "a=" << a << std::endl;

  int b = 1;
  // No problem because we use two different values.
  add(a, b);
  std::cout << "a=" << a << " b=" << b << std::endl;

  // Undefined behaviour
  add(a, a);
  std::cout << "a=" << a << std::endl;
}

// Cons:
// - don't gives more optimizations possibilities
