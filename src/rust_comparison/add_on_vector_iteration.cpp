#include <iostream>
#include <string>
#include <vector>

int main() {
  std::vector<std::string> vec = {"1", "2", "3"};
  // vec.reserve(6); // Will fix this issue
  for (auto str : vec) {
    vec.emplace_back(str);
  }

  for (const auto& str : vec) {
    std::cout << str;
  }
}

// =================================================================
//==1916637==ERROR: AddressSanitizer: heap-use-after-free on address 0x608000000040 at pc 0x55d4a53ab3b7 bp 0x7fff0f2250a0 sp 0x7fff0f225090
//READ of size 8 at 0x608000000040 thread T0
//    #0 0x55d4a53ab3b6 in std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::_M_data() const /usr/include/c++/12/bits/basic_string.h:235
//    #1 0x55d4a53ab0e8 in std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::basic_string(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&) /usr/include/c++/12/bits/basic_string.h:544
//    #2 0x55d4a53a998f in main /home/mhnap/projects/DIY/src/rust_comparison/add_on_vector_iteration.cpp:7
//    #3 0x7f8461629d8f in __libc_start_call_main ../sysdeps/nptl/libc_start_call_main.h:58
//    #4 0x7f8461629e3f in __libc_start_main_impl ../csu/libc-start.c:392
//    #5 0x55d4a53a94e4 in _start (/home/mhnap/projects/DIY/build/debug/src/rust_comparison/rust_comp_add_on_vector_iteration+0x34e4)
