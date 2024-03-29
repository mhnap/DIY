#include <iostream>
#include <variant>

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

    // Bad, UB, got SIGSEGV, need to remember that union value we set previously
    un.n = 43;
    // std::cout << "s: " << un.s << std::endl;
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

  // C++ safe version of tagged union - std::variant
  {
    std::variant<int, std::string> v;
    v = "Hello";
    std::cout << "v size: " << sizeof(v) << " " << v.index() << std::endl;
    v = 42;
    std::cout << "v size: " << sizeof(v) << " " << v.index() << std::endl;
    // Good
    if (std::holds_alternative<int>(v)) {
      std::cout << get<int>(v) << std::endl;
    }
    // Good
    struct Visitor {
      void operator()(int i) const { std::cout << "int: " << i << std::endl; }
      void operator()(const std::string& s) const { std::cout << "string: " << s << std::endl; }
    };
    std::visit(Visitor{}, v);
    v = "Hello";
    std::visit(Visitor{}, v);
    // Almost good
    std::cout << get<int>(v);
  }
}

// https://www.youtube.com/watch?v=Lu1WsdQOi0E&ab_channel=C%2B%2BWeeklyWithJasonTurner
