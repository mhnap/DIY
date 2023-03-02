#include "common/object.hpp"
#include "common/utils.hpp"
#include <functional>
#include <iostream>
#include <vector>

class FooByRef {
public:
  FooByRef(int value) : m_value(value) {}
  void addValue(std::vector<std::function<void()>>& closures) const {
    // We capture by reference, and thus we can have dangling reference to "m_value" inside
    closures.emplace_back([&] { std::cout << "FooByRef: " << m_value << std::endl; });
  }

private:
  int m_value;
};

class FooByValue {
public:
  FooByValue(int value) : m_value(value) {}
  void addValue(std::vector<std::function<void()>>& closures) const {
    // We capture by value, but we still can have dangling reference to "m_value" inside
    // This is because lambda captures apply only to non-static local variables
    // (including parameters) visible in the scope where the lambda is created
    // But "m_value" is data member of class and thus truly means "this->m_value"
    // so turns out that really "this" is captured by value, not "m_value"
    closures.emplace_back([=] { std::cout << "FooByValue: " << m_value << std::endl; });
  }

private:
  int m_value;
};

class FooByCopy {
public:
  FooByCopy(int value) : m_value(value) {}
  void addValue(std::vector<std::function<void()>>& closures) const {
    // We capture copy of "m_value", so no problems at all
    closures.emplace_back([value = m_value] { std::cout << "FooByCopy: " << value << std::endl; });
  }

private:
  int m_value;
};

int main() {
  {
    // We have dangling reference to "a" inside lambda
    std::function<void()> danglingLambda;
    [&] {
      int a = 10;
      danglingLambda = [&] { std::cout << "a: " << a << std::endl; };
    }();
    danglingLambda();
    // We copy "a" value to lambda, so no problem
    std::function<void()> lambda;
    [&] {
      int a = 10;
      lambda = [=] { std::cout << "a: " << a << std::endl; };
    }();
    lambda();
  }
  {
    std::vector<std::function<void()>> closures;
    {
      FooByRef foo(10);
      foo.addValue(closures);
    }
    {
      FooByValue foo(10);
      foo.addValue(closures);
    }
    {
      FooByCopy foo(10);
      foo.addValue(closures);
    }
    for (auto& c : closures) {
      c();
    }
  }
  {
    common::Object object("lvalue");
    common::println("nonGenericLambda:");
    auto nonGenericLambda = [](auto x) {
      common::Object x2(x);
      common::println(x2);
    };
    nonGenericLambda(object);
    nonGenericLambda(common::Object("rvalue"));
    // We need to use auto&& and std::forward<decltype(x)> together
    common::println("genericLambda:");
    auto genericLambda = [](auto&& x) {
      common::Object x2(std::forward<decltype(x)>(x));
      common::println(x2);
    };
    genericLambda(object);
    genericLambda(common::Object("rvalue"));
  }
}