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
    // Need to explicitly get referred object from std::reference_wrapper to use its methods
    std::string s = "42";
    std::reference_wrapper<std::string> r = s;
    // std::cout << "s:" << s.front() << " r:" << r.front(); // error
    std::cout << "s:" << s.front() << "; r:" << r.get().front() << std::endl;
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

  {
    // References cannot be uninitialized
    // int& b;
    // error: ‘b’ declared as reference but not initialized
    // Even reference_wrapper too
    // std::reference_wrapper<int> b;
    int a = 42;
    int& b = a;
    std::cout << "a:" << a << "; b:" << b << std::endl;
  }

  {
    // Can change the owned value if there is still a valid reference to it
    std::vector<int> v = {1, 2};
    int& r = v[0];
    v = {0, 1, 2, 4};
    // std::cout << "r:" << r << std::endl;
    // ==207879==ERROR: AddressSanitizer: heap-use-after-free on address 0x602000000030 at pc 0x55d7bea3baf2 bp 0x7ffc1da8aa80 sp 0x7ffc1da8aa70
    //READ of size 4 at 0x602000000030 thread T0
    //    #0 0x55d7bea3baf1 in main /home/mhnap/projects/DIY/src/rust_comparison/references.cpp:113
    //    #1 0x7fb15ec29d8f in __libc_start_call_main ../sysdeps/nptl/libc_start_call_main.h:58
    //    #2 0x7fb15ec29e3f in __libc_start_main_impl ../csu/libc-start.c:392
    //    #3 0x55d7bea3a604 in _start (/home/mhnap/projects/DIY/build/debug/src/rust_comparison/rust_comp_references+0x3604)
  }

  { std::cout << "sizeof bool:" << sizeof(bool) << "; sizeof bool&:" << sizeof(bool&) << std::endl; }
}

// Differences:
// - references cannot be uninitialized
// - cannot change what reference refers
// - cannot have vector with regular references
// - no dereference operator for reference, thus no need to dereference a reference to change referred value
// - reference member makes a class non-assignable
//
// Similarities:
// - references are non-nullable
// - can create immutable reference what refer to mutable data
// - cannot create mutable reference what refer to immutable data
//
// Cons:
// - references are implicitly created
// - no borrow checker, thus references can easily be dangling
//
// Notes:
// - Rust references are more like std::reference_wrapper
