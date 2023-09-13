#include "common/object.hpp"
#include "common/utils.hpp"

common::Object RVO() {
  return common::Object(); // RVO
}

common::Object correctNRVO() {
  common::Object obj = common::Object(); // prvalue, copy elision
  return obj; // NRVO
}

common::Object wrongNRVO() {
  common::Object obj = common::Object(); // prvalue, copy elision
  return std::move(obj); // cannot do NRVO
}

common::Object getFromParameter(common::Object obj) { return obj; }
common::Object getFromRefParameter(const common::Object& obj) { return obj; }

void printObjectAddress(common::Object obj) { common::println("&obj=", &obj); }

int main() {
  {
    common::println("----- RVO -----");
    common::Object obj = RVO(); // prvalue, copy elision
  }
  {
    common::println("----- Correct NRVO -----");
    common::Object obj = correctNRVO(); // prvalue, copy elision
  }
  {
    common::println("----- Wrong NRVO -----");
    common::Object obj = wrongNRVO(); // prvalue, copy elision
  }
  {
    common::println("----- Initialization copy elision -----");
    printObjectAddress(RVO());
  }
  {
    common::println("----- Get from parameter -----");
    common::Object obj;
    // Object is copied as function parameter and then this new copy is moved by compiler as return value
    common::println("lvalue:");
    getFromParameter(obj);
    // Object is moved as function parameter and then this new copy is moved by compiler as return value
    common::println("xvalue:");
    getFromParameter(std::move(obj));
    // Copy elision for function parameter and then object is moved by compiler as return value
    common::println("prvalue:");
    getFromParameter(common::Object());
  }
  {
    common::println("----- Get from ref parameter -----");
    common::Object obj;
    // Object is copied as return value
    common::println("lvalue:");
    getFromRefParameter(obj);
    // Object is copied as return value
    common::println("xvalue:");
    getFromRefParameter(std::move(obj));
    // Object is copied as return value
    common::println("prvalue:");
    getFromRefParameter(common::Object());
  }
}
