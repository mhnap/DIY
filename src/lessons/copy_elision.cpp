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

void printObjectAddress(common::Object obj) { common::println("&obj=", &obj); }

int main() {
  {
    common::println("----- RVO -----");
    common::Object obj = RVO(); // prvalue, copy elision
    printObjectAddress(std::move(obj)); // xvalue, no copy elision
    printObjectAddress(RVO()); // prvalue, copy elision
  }
  {
    common::println("----- Correct NRVO -----");
    common::Object obj = correctNRVO(); // prvalue, copy elision
    printObjectAddress(std::move(obj)); // xvalue, no copy elision
    printObjectAddress(correctNRVO()); // prvalue, copy elision
  }
  {
    common::println("----- Wrong NRVO -----");
    common::Object obj = wrongNRVO(); // prvalue, copy elision
    printObjectAddress(std::move(obj)); // xvalue, no copy elision
    printObjectAddress(wrongNRVO()); // prvalue, copy elision
  }
}
