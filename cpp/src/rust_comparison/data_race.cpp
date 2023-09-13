// https://medium.com/codex/eda-needs-to-be-using-rust-pt-2-59d2263ebb03

#include <iostream>
#include <random>
#include <string>
#include <thread>

void maybeDoSomeWork() {
  std::srand(std::time(nullptr));
  if (std::rand() % 2) {
    std::this_thread::sleep_for(std::chrono::milliseconds(1));
  }
}

int main() {
  std::string msg = "Hello";
  auto t = std::thread([&]() { std::cout << msg << std::endl; });
  maybeDoSomeWork();
  msg += ", world!";
  t.join();
  return 0;
}
