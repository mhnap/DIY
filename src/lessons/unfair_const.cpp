// Taken from https://www.youtube.com/watch?v=oqGxNd5MPoM&ab_channel=SwedenCpp

#include <cassert>
#include <chrono>
#include <thread>

using namespace std::chrono_literals;

void fun(const std::string& s) {
  const std::string check = s;
  std::this_thread::sleep_for(100ms);
  assert(check == s);
}

int main() {
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
