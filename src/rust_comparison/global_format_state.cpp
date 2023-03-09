// https://en.wikipedia.org/wiki/Criticism_of_C%2B%2B

#include <iostream>
#include <vector>

int main() {
  try {
    std::cout << std::hex << 0xFFFFFFFF << '\n';
    // std::bad_alloc will be thrown here:
    std::vector<int> vector(0xFFFFFFFFFFFFFFFFull);
    std::cout << std::dec; // Never reached
                           // (using scopes guards would have fixed that issue
                           //  and made the code more expressive)
  } catch (const std::exception& e) {
    std::cout << "Error number: " << 10 << '\n'; // Not in decimal
  }
}
