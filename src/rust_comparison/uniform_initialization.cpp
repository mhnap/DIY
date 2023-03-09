// https://en.wikipedia.org/wiki/Criticism_of_C%2B%2B

#include <iostream>
#include <vector>

int main() {
  int integer1{10}; // int
  int integer2(10); // int
  std::vector<int> vector1{10, 0}; // std::initializer_list
  std::vector<int> vector2(10, 0); // std::size_t, int

  std::cout << "Will print 10\n" << integer1 << '\n';
  std::cout << "Will print 10\n" << integer2 << '\n';

  std::cout << "Will print 10,0,\n";

  for (const auto& item : vector1) {
    std::cout << item << ',';
  }

  std::cout << "\nWill print 0,0,0,0,0,0,0,0,0,0,\n";

  for (const auto& item : vector2) {
    std::cout << item << ',';
  }
}
