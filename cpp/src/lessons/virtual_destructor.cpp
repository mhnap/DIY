#include "common/utils.hpp"
#include "memory"

struct NonVirtualDestructor {
  NonVirtualDestructor() { common::println("NonVirtualDestructor default ctor"); }
  ~NonVirtualDestructor() { common::println("NonVirtualDestructor dtor"); }
};

struct FromNonVirtualDestructor : NonVirtualDestructor {
  FromNonVirtualDestructor() { common::println("FromNonVirtualDestructor default ctor"); }
  ~FromNonVirtualDestructor() { common::println("FromNonVirtualDestructor dtor"); }
};

struct VirtualDestructor {
  VirtualDestructor() { common::println("VirtualDestructor default ctor"); }
  virtual ~VirtualDestructor() { common::println("VirtualDestructor dtor"); }
};

struct FromVirtualDestructor : VirtualDestructor {
  FromVirtualDestructor() { common::println("FromVirtualDestructor default ctor"); }
  ~FromVirtualDestructor() override { common::println("FromVirtualDestructor dtor"); }
};

int main() {
  common::println("Non virtual destructor case:");
  {
    common::println("a) Construct using raw ptr and new");
    {
      NonVirtualDestructor* fromNonVirtualDestructor(new FromNonVirtualDestructor());
      delete fromNonVirtualDestructor;
      // No call of ~FromNonVirtualDestructor(), because our class is derived
      // from base that does not have destructor marked as virtual, BAD
    }
    common::println("b) Construct using shared ptr");
    {
      std::shared_ptr<NonVirtualDestructor> fromNonVirtualDestructor(new FromNonVirtualDestructor());
      // There is call of ~FromNonVirtualDestructor(), because internal pointer type in shared_ptr
      // (in control block) is type that was passed to constructor - derived type (FromNonVirtualDestructor)
      // and thus ~FromNonVirtualDestructor (and ~NonVirtualDestructor) are called, GOOD
    }
    common::println("c) Construct using unique ptr");
    {
      std::unique_ptr<NonVirtualDestructor> fromNonVirtualDestructor(new FromNonVirtualDestructor());
      // No call of ~FromNonVirtualDestructor(), because internal pointer type in unique_ptr
      // is base type (NonVirtualDestructor) and only base destructor is called, BAD
    }
    common::println("d) Construct using shared ptr and previously created object ");
    {
      NonVirtualDestructor* fromNonVirtualDestructor(new FromNonVirtualDestructor());
      std::shared_ptr<NonVirtualDestructor> p(fromNonVirtualDestructor);
      // No call of ~FromNonVirtualDestructor(), because internal pointer type in shared_ptr
      // (in control block) is type that was passed to constructor - base type (NonVirtualDestructor)
      // and only base destructor is called, BAD
    }
  }
  common::println();
  common::println("Virtual destructor case:");
  {
    common::println("a) Construct using raw ptr and new");
    {
      VirtualDestructor* fromVirtualDestructor(new FromVirtualDestructor());
      delete fromVirtualDestructor;
    }
    common::println("b) Construct using shared ptr");
    { std::shared_ptr<VirtualDestructor> fromVirtualDestructor(new FromVirtualDestructor()); }
    common::println("c) Construct using unique ptr");
    { std::unique_ptr<VirtualDestructor> fromVirtualDestructor(new FromVirtualDestructor()); }
    common::println("d) Construct using shared ptr and previously created object ");
    {
      VirtualDestructor* fromVirtualDestructor(new FromVirtualDestructor());
      std::shared_ptr<VirtualDestructor> p(fromVirtualDestructor);
    }
    // All cases are "GOOD", if we agree that we really need vtables at all
  }
}