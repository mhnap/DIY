#include "common/object.hpp"
#include "common/utils.hpp"

struct Foo1 {
  Foo1(const common::Object& object) : m_object(object) {}
  common::Object m_object;
};

struct Foo2 {
  Foo2(const common::Object& object) : m_object(object) {}
  Foo2(common::Object&& object) : m_object(std::move(object)) {}
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

common::Object getObject() {
  common::Object object("object");
  return object;
}

common::Object strangeGetObject() {
  common::Object object("object");
  return std::move(object);
}

common::Object returnObject(const common::Object& object) { return object; }

common::Object strangeReturnObject(common::Object object) { return object; }

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
    // Construct object
    common::Object object1("object");
    // Construct another from lvalue
    common::Object object2(object1);
    // Construct another from prvalue
    common::Object object3(common::Object{"prvalue"});
    // Construct another from xvalue
    common::Object object4(std::move(object1));
    common::print("object1 - ", object1);
    common::print("object2 - ", object2);
    common::print("object3 - ", object3);
    common::print("object4 - ", object4);
  }
  {
    common::Object object("object");
    common::print("Foo1 lvalue:");
    Foo1 foo1l(object);
    common::print("Foo1 rvalue:");
    Foo1 foo1r({"object"});
    common::print("Foo2 lvalue:");
    Foo2 foo2l(object);
    common::print("Foo2 rvalue:");
    Foo2 foo2r({"object"});
    common::print("Foo3 lvalue:");
    Foo3 foo3l(object);
    common::print("Foo3 rvalue:");
    Foo3 foo3r({"object"});
    common::print("Foo4 lvalue:");
    Foo4 foo4l(object);
    common::print("Foo4 rvalue:");
    Foo4 foo4r({"object"});
    common::print("Foo5 lvalue:");
    Foo5 foo5l(object);
    common::print("Foo5 rvalue:");
    Foo5 foo5r({"object"});
  }
  {
    // RVO, only constructor is called
    common::print("getObject:");
    auto object = getObject();
    // Move inside prevents copy elision, so we have additional move here
    common::print("strangeGetObject:");
    auto object1 = strangeGetObject();
    // Object is copied
    common::print("returnObject:");
    auto object2 = returnObject(object1);
    // Object is copied as function parameter and then this new copy is moved by compiler as return value
    common::print("strangeReturnObject:");
    auto object3 = strangeReturnObject(object1);
  }
  {
    // Mistake taken from https://youtu.be/ECoLo17nG5c?t=2419
    auto object1 = getObject();
    // Move won't work, because of reference collapsing rules
    common::Object object2(my_move(object1));
    // Need to use std::remove_reference to make it work as expected
    common::Object object3(correct_my_move(object1));
  }
}