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
//==201660==ERROR: AddressSanitizer: heap-use-after-free on address 0x60d000000040 at pc 0x55c27327b77e bp
// 0x7ffd9a089cf0 sp 0x7ffd9a0894c0 READ of size 72 at 0x60d000000040 thread T0
//==201660==WARNING: invalid path to external symbolizer!
//==201660==WARNING: Failed to use and restart external symbolizer!
//    #0 0x55c27327b77d  (/home/mhnap/projects/DIY/build/debug/src/rust_comparison/use_after_free+0x3b77d)
//    (BuildId: ee3566d5097b4d4af9b1763ce6059caa09740461) #1 0x7f8d5113cb34
//    (/lib/x86_64-linux-gnu/libstdc++.so.6+0x13cb34) (BuildId: f57e02bfadacc0c923c82457d5e18e1830b5faea) #2
//    0x55c27331f8dc  (/home/mhnap/projects/DIY/build/debug/src/rust_comparison/use_after_free+0xdf8dc)
//    (BuildId: ee3566d5097b4d4af9b1763ce6059caa09740461) #3 0x7f8d50c29d8f
//    (/lib/x86_64-linux-gnu/libc.so.6+0x29d8f) (BuildId: 69389d485a9793dbe873f0ea2c93e02efaa9aa3d) #4
//    0x7f8d50c29e3f  (/lib/x86_64-linux-gnu/libc.so.6+0x29e3f) (BuildId:
//    69389d485a9793dbe873f0ea2c93e02efaa9aa3d) #5 0x55c27325f454
//    (/home/mhnap/projects/DIY/build/debug/src/rust_comparison/use_after_free+0x1f454) (BuildId:
//    ee3566d5097b4d4af9b1763ce6059caa09740461)
