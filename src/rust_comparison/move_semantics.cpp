#include <iostream>
#include <memory>
#include <string>
#include <vector>

std::vector<std::string> getVecWithString(std::string str) {
  std::vector<std::string> vec;
  vec.push_back(std::move(str));
  return vec;
}

int main() {
  {
    int a = 42;
    // Copy
    int b = a;
    std::cout << "a:" << a << "; b:" << b << std::endl;
  }

  {
    int a = 42;
    // Still copy, no move for trivially-copyable types
    int b = std::move(a);
    std::cout << "a:" << a << "; b:" << b << std::endl;
  }

  {
    std::string a = "42";
    // Copy (or better to say deep copy as copy constructor is called)
    std::string b = a;
    std::cout << "a:" << a << "; b:" << b << std::endl;
  }

  {
    std::string a = "42";
    // Move (or better to say memcpy as move constructor is called)
    std::string b = std::move(a);
    std::cout << "a:" << a << "; b:" << b << std::endl;
  }

  {
    std::string str("42");
    std::vector<std::string> vec = getVecWithString(std::move(str));
    std::cout << "vec:" << vec[0] << std::endl;
    std::cout << "str:" << str << std::endl;
  }

  {
    std::unique_ptr<std::string> a = std::make_unique<std::string>("42");
    // unique_ptr cannot be copied (deep copied)
    // C++ use copy by default, so it would be confusing to implicitly make a deep copy when copying a pointer
    // std::unique_ptr<std::string> b = a;
    // Call to deleted constructor of 'std::unique_ptr<std::string>' (aka 'unique_ptr<basic_string<char>>')
    std::cout << "a:" << *a << "; b:"
              << "cannot copy" << std::endl;
  }

  {
    std::unique_ptr<std::string> a = std::make_unique<std::string>("42");
    // unique_ptr can be moved (memcpy)
    std::unique_ptr<std::string> b = std::move(a);
    std::cout << "a:" << *a << "; b:" << *b << std::endl;
    // Process finished with exit code 139 (interrupted by signal 11: SIGSEGV)
  }
}

// Cons:
// - need to explicitly use std::move to turn on move semantics
// - need to remember not to use moved-from object (or add static check for this)
// - need to implement custom functions (move ctor, assignment, etc.) to "move" data
// - types need to handle empty but valid states (unique_ptr can have nullptr)
// - need to make correct destructor for types that support move
// - destructors are run for moved-from object, but there are no need for this
//
// Seems that almost all of cons can be fixed with "destructive move"
