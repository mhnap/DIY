#include "common/object.hpp"
#include "common/utils.hpp"
#include <string>

class Person1 : public common::Object {
public:
  template <typename T>
  explicit Person1(T&& n) : Object(std::forward<T>(n)) {
    common::print("Person1 user ctor");
  }
  Person1(const Person1& p) : Object(p) { common::print("Person1 copy ctor"); }
};

class Person2 : public common::Object {
public:
  template <typename T, typename = std::enable_if_t<!std::is_same_v<Person2, T>>>
  explicit Person2(T&& n) : Object(std::forward<T>(n)) {
    common::print("Person2 user ctor");
  }
  Person2(const Person2& p) : Object(p) { common::print("Person2 copy ctor"); }
};

class Person3 : public common::Object {
public:
  template <typename T, typename = std::enable_if_t<!std::is_same_v<Person3, std::decay_t<T>>>>
  explicit Person3(T&& n) : Object(std::forward<T>(n)) {
    common::print("Person3 user ctor");
  }
  Person3(const Person3& p) : Object(p) { common::print("Person3 copy ctor"); }
};

class Person4 : public common::Object {
public:
  template <typename T, typename = std::enable_if_t<!std::is_base_of_v<Person4, std::decay_t<T>>>>
  explicit Person4(T&& n) : Object(std::forward<T>(n)) {
    common::print("Person4 user ctor");
  }
  Person4(const Person4& p) : Object(p) { common::print("Person4 copy ctor"); }
};

class DerivedPerson1 : public Person3 {
public:
  explicit DerivedPerson1(std::string s) : Person3(s) { common::print("DerivedPerson1 user ctor"); }
  DerivedPerson1(const DerivedPerson1& d) : Person3(d) { common::print("DerivedPerson1 copy ctor"); }
};

class DerivedPerson2 : public Person4 {
public:
  explicit DerivedPerson2(std::string s) : Person4(s) { common::print("DerivedPerson2 user ctor"); }
  DerivedPerson2(const DerivedPerson2& d) : Person4(d) { common::print("DerivedPerson2 copy ctor"); }
};

int main() {
  {
    // Person1 user ctor is called as expected
    Person1 p1("person");
    // Person1 copy ctor is not called, instead Person1 user ctor is also called here
    // This is because Person1 user ctor is better match for "Person1&" type than copy ctor
    Person1 p2(p1);
  }
  {
    // Person2 user ctor is called as expected
    Person2 p1("person");
    // Person2 copy ctor is not called, instead Person2 user ctor is also called here
    // This is because Person2 user ctor enabled only if is the same as "Person2" type
    // but parameter type is "Person2&", so it's not the same type
    Person2 p2(p1);
  }
  {
    // Person3 user ctor is called as expected
    Person3 p1("person");
    // Person3 copy ctor is also called as expected
    // We check whether a type is the same as Person3, but decay first
    // so we will ignore references and cv-qualifiers also
    Person3 p2(p1);
  }
  {
    // DerivedPerson1 user ctor and Person3 user ctor are called as expected
    DerivedPerson1 d1("derived person");
    // DerivedPerson1 copy ctor is also called as expected,
    // but inside Person3 user ctor is called instead of Person3 copy ctor
    // This is because "const DerivedPerson1&" type is not the same as "Person3"
    DerivedPerson1 d2(d1);
  }
  {
    // DerivedPerson2 user ctor and Person4 user ctor are called as expected
    DerivedPerson2 d1("derived person");
    // DerivedPerson2 copy ctor and Person4 copy ctor are also called as expected
    // This is because we use "is_base_of_v" instead of "is_same_v" inside Person4 user ctor
    DerivedPerson2 d2(d1);
  }
}