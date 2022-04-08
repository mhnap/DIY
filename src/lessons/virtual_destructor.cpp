#include "common/utils.hpp"
#include "memory"

struct NonVirtualDestructor {
  NonVirtualDestructor() { common::print("NonVirtualDestructor default ctor"); }
  ~NonVirtualDestructor() { common::print("NonVirtualDestructor dtor"); }
};

struct FromNonVirtualDestructor : NonVirtualDestructor {
  FromNonVirtualDestructor() { common::print("FromNonVirtualDestructor default ctor"); }
  ~FromNonVirtualDestructor() { common::print("FromNonVirtualDestructor dtor"); }
};

struct VirtualDestructor {
  VirtualDestructor() { common::print("VirtualDestructor default ctor"); }
  virtual ~VirtualDestructor() { common::print("VirtualDestructor dtor"); }
};

struct FromVirtualDestructor : VirtualDestructor {
  FromVirtualDestructor() { common::print("FromVirtualDestructor default ctor"); }
  ~FromVirtualDestructor() override { common::print("FromVirtualDestructor dtor"); }
};

int main() {
  common::print("Non virtual destructor case:");
  {
    common::print("a) Construct using raw ptr and new");
    {
      NonVirtualDestructor* fromNonVirtualDestructor(new FromNonVirtualDestructor());
      delete fromNonVirtualDestructor;
      // No call of ~FromNonVirtualDestructor(), because our class is derived
      // from base that does not have destructor marked as virtual, BAD
    }
    common::print("b) Construct using shared ptr");
    {
      std::shared_ptr<NonVirtualDestructor> fromNonVirtualDestructor(new FromNonVirtualDestructor());
      // There is call of ~FromNonVirtualDestructor(), because internal pointer type in shared_ptr
      // (in control block) is type that was passed to constructor - derived type (FromNonVirtualDestructor)
      // and thus ~FromNonVirtualDestructor (and ~NonVirtualDestructor) are called, GOOD
    }
    common::print("c) Construct using unique ptr");
    {
      std::unique_ptr<NonVirtualDestructor> fromNonVirtualDestructor(new FromNonVirtualDestructor());
      // No call of ~FromNonVirtualDestructor(), because internal pointer type in unique_ptr
      // is base type (NonVirtualDestructor) and only base destructor is called, BAD
    }
    common::print("d) Construct using shared ptr and previously created object ");
    {
      NonVirtualDestructor* fromNonVirtualDestructor(new FromNonVirtualDestructor());
      std::shared_ptr<NonVirtualDestructor> p(fromNonVirtualDestructor);
      // No call of ~FromNonVirtualDestructor(), because internal pointer type in shared_ptr
      // (in control block) is type that was passed to constructor - base type (NonVirtualDestructor)
      // and only base destructor is called, BAD
    }
  }
  common::print();
  common::print("Virtual destructor case:");
  {
    common::print("a) Construct using raw ptr and new");
    {
      VirtualDestructor* fromVirtualDestructor(new FromVirtualDestructor());
      delete fromVirtualDestructor;
    }
    common::print("b) Construct using shared ptr");
    {
      std::shared_ptr<VirtualDestructor> fromVirtualDestructor(new FromVirtualDestructor());
    }
    common::print("c) Construct using unique ptr");
    {
      std::unique_ptr<VirtualDestructor> fromVirtualDestructor(new FromVirtualDestructor());
    }
    common::print("d) Construct using shared ptr and previously created object ");
    {
      VirtualDestructor* fromVirtualDestructor(new FromVirtualDestructor());
      std::shared_ptr<VirtualDestructor> p(fromVirtualDestructor);
    }
    // All cases are "GOOD", if we agree that we really need vtables at all
  }
}