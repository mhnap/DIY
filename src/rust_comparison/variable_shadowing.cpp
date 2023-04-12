#include <iostream>

int main() {
  auto shadowed_binding = 1.5;
  {
    std::cout << "before being shadowed: " << shadowed_binding << std::endl;

    // Can shadow in different block
    auto shadowed_binding = "abc";
    std::cout << "shadowed in inner block: " << shadowed_binding << std::endl;
  }
  std::cout << "outside inner block: " << shadowed_binding << std::endl;

  // Cannot shadow in the same block
  // auto shadowed_binding = 1; // Error
}

// Differences:
// - cannot shadow variable in the same scope
//
// Similarities:
// - can shadow variables in different scopes
