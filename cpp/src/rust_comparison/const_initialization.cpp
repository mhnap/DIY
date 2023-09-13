// https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Res-lambda-init
// https://www.cppstories.com/2016/11/iife-for-complex-initialization/
// https://herbsutter.com/2013/04/05/complex-initialization-for-a-const-variable/

bool condition = true;

int main() {
  // Simple example
  {
    // const int a; // cannot use const here
    int a;
    if (condition) {
      a = 1;
    } else {
      a = 2;
    }
  }

  {
    // Ternary can be used instead
    const int a = condition ? 1 : 2;
  }

  {
    // Or can use IIFE (immediately invoked function expression)
    const int a = [] {
      if (condition) {
        return 1;
      }
      return 2;
    }();
  }

  // NOTE: Notice that const is not default
}
