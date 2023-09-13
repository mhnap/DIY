// https://jasonmccampbell.medium.com/eda-needs-to-be-using-rust-a6a09911da74

#include <iostream>
#include <string>

int main(int argc, const char** argv) {
  const std::string* str;
  const std::string arg0 = argv[0];
  if (argc == 1) {
    str = &arg0;
  } else {
    const auto local_var = arg0 + argv[1];
    str = &local_var;
  }
  std::cout << "Result = " << *str << std::endl;
}

// =================================================================
//==1916738==ERROR: AddressSanitizer: heap-use-after-free on address 0x60c000000040 at pc 0x7f3556e4e602 bp 0x7fffa564b040 sp 0x7fffa564a7e8
//READ of size 59 at 0x60c000000040 thread T0
//    #0 0x7f3556e4e601 in __interceptor_fwrite ../../../../src/libsanitizer/sanitizer_common/sanitizer_common_interceptors.inc:1157
//    #1 0x7f3556b3cb34 in std::basic_ostream<char, std::char_traits<char> >& std::__ostream_insert<char, std::char_traits<char> >(std::basic_ostream<char, std::char_traits<char> >&, char const*, long) (/lib/x86_64-linux-gnu/libstdc++.so.6+0x13cb34)
//    #2 0x5569f8e777eb in main /home/mhnap/projects/DIY/src/rust_comparison/use_after_free.cpp:15
//    #3 0x7f3556629d8f in __libc_start_call_main ../sysdeps/nptl/libc_start_call_main.h:58
//    #4 0x7f3556629e3f in __libc_start_main_impl ../csu/libc-start.c:392
//    #5 0x5569f8e774c4 in _start (/home/mhnap/projects/DIY/build/debug/src/rust_comparison/rust_comp_use_after_free+0x34c4)
