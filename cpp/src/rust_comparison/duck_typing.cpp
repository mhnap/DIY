#include <iostream>
#include <string>
#include <vector>

// In C++, template functions are being checked for each concrete type, not for generic type `T`.
// So, in this case, we will have two same errors for two different types - int and char.
// The error, in this case, is a function instantiation.
// This seems more like a "duck typing".
template <class T>
T largest(const std::vector<T>& list) {
  T largest = list[0];
  for (auto item : list) {
    if (item.larger(largest)) {
      largest = item;
    }
  }
  return largest;
}

//

// NOTE: Generic type does mean ANY type that compiles with.

// Work.
template <typename T>
void push_one(std::vector<T>& vec) {
  vec.push_back(1.6);
}

// Work from C++20.
//void push_one(auto& vec) {
//    vec.push_back(1);
//}

// Work from C++20.
//void push_one(std::vector<auto>& vec) {
//    vec.push_back(1);
//}

int main() {
  const std::vector<int> number_list = {34, 50, 25, 100, 65};
  const int number_result = largest(number_list);
  std::cout << "The largest number is " << number_result << std::endl;

  const std::vector<char> char_list = {'y', 'm', 'a', 'q'};
  const int char_result = largest(char_list);
  std::cout << "The largest char is " << char_result << std::endl;

  //

  // std::vector vec = {0.0}; // output is 0 1.6
  std::vector vec = {0}; // output is 0 1
  push_one(vec);
  std::cout << vec[0] << " " << vec[1] << std::endl;
}

// https://wiki.c2.com/?LatentTypesSmell
