#include "common/object.hpp"
#include "common/utils.hpp"
#include <utility>

namespace {
common::Object g_object("object");
}

struct Foo1 {
  Foo1(const common::Object& object) : m_object(object) {}
  common::Object m_object;
};

struct Foo2 {
  Foo2(const common::Object& object) : m_object(object) {}
  Foo2(common::Object&& object)
      : m_object(std::move(object)) {
  } // `object` expression here is still lvalue, even that `object` object is a rvalue reference, so need move
  common::Object m_object;
};

struct Foo3 {
  template <typename T>
  Foo3(T&& object) : m_object(std::forward<T>(object)) {}
  common::Object m_object;
};

struct Foo4 {
  Foo4(common::Object object) : m_object(std::move(object)) {}
  common::Object m_object;
};

struct Foo5 {
  Foo5(common::Object object) { m_object = object; }
  common::Object m_object;
};

template <typename T>
void testConstructor() {
  common::println("---------- ", typeid(T).name(), " ----------");
  {
    common::println("lvalue:");
    T foo(g_object);
  }
  {
    common::println("xvalue:");
    T foo(std::move(g_object));
  }
  {
    common::println("prvalue:");
    T foo(common::Object{"object"});
  }
}

template <typename T>
T&& my_move(T&& x) {
  return static_cast<T&&>(x);
}

template <typename T>
std::remove_reference_t<T>&& correct_my_move(T&& x) {
  return static_cast<std::remove_reference_t<T>&&>(x);
}

int main() {
  {
    common::println("---------- Construction ----------");
    // Construct object
    common::Object object1("object");
    // Construct another from lvalue
    common::Object object2(object1);
    // Construct another from prvalue
    common::Object object3(common::Object{"prvalue"});
    // Construct another from xvalue
    common::Object object4(std::move(object1));
    common::println("object1 - ", object1);
    common::println("object2 - ", object2);
    common::println("object3 - ", object3);
    common::println("object4 - ", object4);
  }

  testConstructor<Foo1>();
  testConstructor<Foo2>();
  testConstructor<Foo3>();
  testConstructor<Foo4>();
  testConstructor<Foo5>();

  {
    common::println("---------- Move implementation ----------");
    // Mistake taken from https://youtu.be/ECoLo17nG5c?t=2419
    common::Object object1;
    // Move won't work, because of reference collapsing rules
    common::println("my_move:");
    common::Object object2(my_move(object1));
    // Need to use std::remove_reference to make it work as expected
    common::println("correct_my_move:");
    common::Object object3(correct_my_move(object1));
  }
}
