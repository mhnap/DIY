#include <functional>
#include <iostream>
#include <string>
#include <vector>

int main() {
  {
    // References are implicitly created
    std::string a = "42";
    std::string& b = a;
    std::cout << "a:" << a << "; b:" << b << std::endl;
  }

  {
    // It is not possible to change what a reference refers to after initialization
    std::string a = "42";
    std::string& b = a;
    std::string c = "41";
    b = c;
    std::cout << "a:" << a << "; b:" << b << "; c:" << c << std::endl;
  }

  {
    // But, C++ has std::reference_wrapper for such case
    std::string a = "42";
    std::reference_wrapper<std::string> b = a;
    std::string c = "41";
    b = c;
    std::cout << "a:" << a << "; b:" << b.get() << "; c:" << c << std::endl;
  }

  {
    // Can create immutable reference what refer to mutable data
    int a = 42;
    const int& b = a;
    int c = 41;
    // b = c;
    // error: assignment of read-only reference ‘b’
    //   35 |     b = c;
    //      |     ~~^~~
    std::cout << "a:" << a << "; b:" << b << "; c:" << c << std::endl;
  }

  {
    // Cannot create mutable reference what refer to immutable data
    const int a = 42;
    // int& b = a;
    // error: binding reference of type ‘int&’ to ‘const int’ discards qualifiers
    //   46 |     int& b = a;
    //      |              ^
    std::cout << "a:" << a << std::endl;
  }

  {
    // Cannot have vector with regular references
    int a = 42;
    // std::vector<int&> vec;
    // Some crazy errors..
    std::vector<std::reference_wrapper<int>> vec;
    vec.emplace_back(a);
    std::cout << "a:" << vec[0] << std::endl;
  }

  {
    // No need to dereference a reference to change referred value
    int a = 42;
    {
      int& b = a;
      b = 41;
    }
    std::cout << "a:" << a << std::endl;
  }

  {
    // Reference member makes a class non-assignable
    struct S {
      int& r;
    };
    int a = 42;
    int b = 41;
    S sa{a};
    S sb{b};
    // sa = sb;
    // Object of type 'S' cannot be assigned because its copy assignment operator is implicitly deleted copy
    // assignment operator of 'S' is implicitly deleted because field 'r' is of reference type 'int &'
    std::cout << "sa:" << sa.r << "; sb:" << sb.r << std::endl;
  }
}

// Differences:
// - cannot change what reference refers
// - cannot have vector with regular references
// - no dereference operator for reference, thus no need to dereference a reference to change referred value
// - reference member makes a class non-assignable
//
// Similarities:
// - can create immutable reference what refer to mutable data
// - cannot create mutable reference what refer to immutable data
//
// Cons:
// - references are implicitly created
//
// Notes:
// - Rust references are more like std::reference_wrapper
