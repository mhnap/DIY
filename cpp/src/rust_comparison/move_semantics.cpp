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
    // Move don't work on const objects
    const std::string str("42");
    std::vector<std::string> vec = getVecWithString(std::move(str));
    std::cout << "vec:" << vec[0] << std::endl;
    std::cout << "str:" << str << std::endl;
  }

  {
    // Object can be moved also using reference
    std::string str("42");
    std::string& str_ref = str;
    std::string new_str = std::move(str_ref);
    std::cout << "str:" << str << std::endl;
    std::cout << "str_ref:" << str_ref << std::endl;
    std::cout << "new_str:" << new_str << std::endl;
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
    // std::cout << "a:" << *a << "; b:" << *b << std::endl;
    // Process finished with exit code 139 (interrupted by signal 11: SIGSEGV)
  }

  {
    // Can move the owned value if there is still a valid reference to it
    std::vector<int> v = {1, 2};
    int& r = v[0];
    std::vector<int> nv = std::move(v);
    nv = {1, 2, 3, 4};
    std::cout << "r:" << r << std::endl;
    // ==210877==ERROR: AddressSanitizer: heap-use-after-free on address 0x602000000010 at pc 0x55b7e5f886c3 bp 0x7ffcfe3b95d0 sp 0x7ffcfe3b95c0
    //READ of size 4 at 0x602000000010 thread T0
    //\    #0 0x55b7e5f886c2 in main /home/mhnap/projects/DIY/src/rust_comparison/move_semantics.cpp:83
    //    #1 0x7fce00429d8f in __libc_start_call_main ../sysdeps/nptl/libc_start_call_main.h:58
    //    #2 0x7fce00429e3f in __libc_start_main_impl ../csu/libc-start.c:392
    //    #3 0x55b7e5f875c4 in _start (/home/mhnap/projects/DIY/build/debug/src/rust_comparison/rust_comp_move_semantics+0x35c4)
  }
}

// Cons:
// - move is implemented with rvalue overloading, thus need to implement custom functions (move ctor, assignment, etc.) to "move" data
//                                                not so simple logic (rvalue references, casts, etc.)
// - need to explicitly use std::move to turn on move semantics
// - non-destructive move, thus destructors are run for moved-from object, but there are no need for this
//                         thus need to make correct destructor for types that support move
//                         thus types need to handle empty but valid states (unique_ptr can have nullptr)
//                         thus need to remember not to use moved-from object (or add static check for this)
// - move don't work on const objects
// - object can be moved also using reference
// - implicit deep copy
//
// Many of cons can be fixed with "destructive move", but not all
