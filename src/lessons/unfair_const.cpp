// Taken from https://www.youtube.com/watch?v=oqGxNd5MPoM&ab_channel=SwedenCpp
// https://www.youtube.com/watch?v=OupN6FMZbmA&t=3s&ab_channel=CppCon

#include <cassert>
#include <chrono>
#include <thread>

using namespace std::chrono_literals;

void fun(const std::string& s) {
  const std::string check = s;
  std::this_thread::sleep_for(100ms);
  assert(check == s);
}

void test_concurrent() {
  // good :)
  {
    std::string s = "foo";
    fun(s);
  }
  // not good :(
  {
    std::string s = "foo";
    std::thread t(fun, std::cref(s));
    std::this_thread::sleep_for(50ms);
    s = "bar";
    t.join();
  }
}

int g_i{};

void foo() { ++g_i; }

void bar(const int& i) {
  const int check = i;
  assert(check == i);
  foo();
  assert(check == i);
}

void test_global() { bar(g_i); }

int main() {
  // Problem with concurrent access
  test_concurrent();

  // The same problem with global object
  test_global();
}
