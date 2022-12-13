#include <initializer_list>
#include <vector>

int main() {
  // Deduced as std::initializer_list<int>
  auto i1 = {5};
  // Deduced as int
  auto i2{5};

  std::vector<bool> vec{true, true};
  // Deduced as bool
  auto first = static_cast<bool>(vec[0]);
  // Deduced as std::vector<bool>::reference
  auto second = vec[1];
}