#include <iostream>

int main() {
  {
    // Good, expected behaviour
    struct X {
      int a;
      int b;
      int c;
    };
    X x = {1, 2, 3};
    std::cout << x.a << x.b << x.c << std::endl;
  }
  {
    // Bad, behaviour changes if we reorder struct fields
    struct X {
      int b;
      int a;
      int c;
    };
    X x = {1, 2, 3};
    std::cout << x.a << x.b << x.c << std::endl;
  }
  {
    // Almost good
    struct X {
      int b;
      int a;
      int c;
    };
    // X x = {.a = 1, .b = 2, .c = 3};
    // error: designator order for field ‘main()::X::b’ does not match declaration order in ‘main()::X’
    X x = {.b = 2, .a = 1, .c = 3};
    std::cout << x.a << x.b << x.c << std::endl;
  }
  {
    // Bad, not guaranteed to be initialized
    // Can be seen by -Wuninitialized option
    struct X {
      int a;
      int b;
      int c;
    };
    X x;
    std::cout << x.a << x.b << x.c << std::endl;
  }
  {
    // Good, guaranteed 'b' and 'c' to be initialized
    struct X {
      int a;
      int b;
      int c;
    };
    X x = {.a = 1};
    std::cout << x.a << x.b << x.c << std::endl;
  }

  {
    // Cannot have fields and methods have the same identifiers
    struct X {
      int a;
      // void a() {}
      // Redefinition of 'a' as different kind of symbol
    };
  }
}
