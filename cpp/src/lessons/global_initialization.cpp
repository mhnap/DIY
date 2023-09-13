// https://stackoverflow.com/questions/8708539/order-of-initialization

// If uncomment this, std::cout will be initialized before usage
// #include <iostream>

struct Foo {
  Foo();
} foo;

// In this case, std::cout won't be initialized and program will catch SIGSEGV
#include <iostream>

Foo::Foo() { std::cout << "Hello World"; }

int main() {}