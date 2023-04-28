#include <iostream>

int main() {
  // Regular union
  {
    //    union num_or_str {
    //      int n;
    //      std::string s;
    //    } un;
    // error, cannot use type with non-trivial (copy/)constructor in a union

    union num_or_str {
      int n;
      char* s;
    } un;
    std::cout << "un size: " << sizeof(un) << std::endl;

    // Good
    un.n = 42;
    std::cout << "n: " << un.n << std::endl;

    // Good
    un.s = (char*)"Hello";
    std::cout << "s: " << un.s << std::endl;

    // Bad, got SIGSEGV, need to remember that union value we set previously
    un.n = 43;
    std::cout << "s: " << un.s << std::endl;
  }

  // Tagged union
  {
    struct tagged_num_or_str {
      short tag;
      union num_or_str {
        int n;
        char* s;
      } num_or_str;
    } un;
    std::cout << "un size: " << sizeof(un) << std::endl;

    // Good
    un.num_or_str.n = 42;
    un.tag = 0;
    std::cout << "n: " << un.num_or_str.n << std::endl;

    // Good
    un.num_or_str.s = (char*)"Hello";
    un.tag = 1;
    std::cout << "s: " << un.num_or_str.s << std::endl;

    // Good, because even if we forgot what value we set, we still can check the tag
    if (un.tag == 0) {
      std::cout << "n: " << un.num_or_str.n << std::endl;
    } else {
      std::cout << "s: " << un.num_or_str.s << std::endl;
    }
  }

  // Tagged union with enum as tag
  {
    struct tagged_num_or_str {
      enum class kind { num, str } kind;
      union num_or_str {
        int n;
        char* s;
      } num_or_str;
    } un;
    std::cout << "un size: " << sizeof(un) << std::endl;

    // Good
    un.num_or_str.n = 42;
    un.kind = tagged_num_or_str::kind::num;
    std::cout << "n: " << un.num_or_str.n << std::endl;

    // Good
    un.num_or_str.s = (char*)"Hello";
    un.kind = tagged_num_or_str::kind::str;
    std::cout << "s: " << un.num_or_str.s << std::endl;
  }
}

// https://www.youtube.com/watch?v=Lu1WsdQOi0E&ab_channel=C%2B%2BWeeklyWithJasonTurner
