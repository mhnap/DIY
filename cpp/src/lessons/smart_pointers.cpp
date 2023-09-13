#include "common/object.hpp"
#include "common/utils.hpp"
#include <memory>

struct Foo : common::Object {
  std::shared_ptr<Foo> getPtr() { return std::shared_ptr<Foo>(this); }
};

struct SharedFoo : common::Object, std::enable_shared_from_this<SharedFoo> {
  std::shared_ptr<SharedFoo> getPtr() { return shared_from_this(); }
};

struct BetterSharedFoo : common::Object, std::enable_shared_from_this<BetterSharedFoo> {
  static std::shared_ptr<BetterSharedFoo> create() {
    return std::shared_ptr<BetterSharedFoo>(new BetterSharedFoo());
  }
  std::shared_ptr<BetterSharedFoo> getPtr() { return shared_from_this(); }

private:
  BetterSharedFoo() = default;
};

struct Child;

struct Parent : common::Object {
  std::shared_ptr<Child> child;
};

struct Child : common::Object {
  std::shared_ptr<Parent> parent;
  std::weak_ptr<Parent> weakParent;
};

int main() {
  {
    common::println("-------- unique_ptr part");
    std::unique_ptr<common::Object> ptr1;
    if (ptr1) {
      common::println("Valid ptr1");
    }
    std::unique_ptr<common::Object> ptr2(new common::Object());
    if (ptr2) {
      common::println("Valid ptr2");
    }
    auto ptr3 = std::make_unique<common::Object>();
    if (ptr3) {
      common::println("Valid ptr3");
    }
    // Cannot be copied
    //    ptr3 = ptr2;
    //    auto ptr4 = ptr3;
    auto ptr4 = std::move(ptr3);
    if (ptr3) {
      common::println("Valid ptr3");
    }
    if (ptr4) {
      common::println("Valid ptr4");
    }
    ptr4.reset();
    if (ptr4) {
      common::println("Valid ptr4");
    }
    auto customDeleter = [](common::Object* object) {
      common::println("Delete ptr5");
      delete object;
    };
    std::unique_ptr<common::Object, decltype(customDeleter)> ptr5(new common::Object());
    { std::unique_ptr<common::Object[]> ptr6(new common::Object[10]); }

    common::println("unique_ptr size: ", sizeof(ptr2));
    common::println("raw ptr size: ", sizeof(ptr2.get()));
    common::println("unique_ptr with deleter size: ", sizeof(ptr5));
    common::println("unique_ptr address: ", &ptr2);
    common::println("raw ptr address: ", ptr2.get());
  }
  {
    common::println("-------- shared_ptr part");
    auto object = new common::Object();
    std::shared_ptr<common::Object> ptr1(object);
    // double free
    //    std::shared_ptr<common::Object> ptr2(object);
    std::shared_ptr<common::Object> ptr2(ptr1);

    auto ptr3 = std::make_shared<Foo>();
    // double free
    //    auto ptr4 = ptr3->getPtr();

    {
      auto ptr4 = std::make_shared<SharedFoo>();
      auto ptr5 = ptr4->getPtr();

      auto ptr6 = new SharedFoo();
      // std::bad_weak_ptr
      //    auto ptr7 = ptr6->getPtr();
    }
    // Cannot be abused
    {
      auto ptr4 = BetterSharedFoo::create();
      auto ptr5 = ptr4->getPtr();
      ptr5->clearCounts();

      // auto ptr6 = new BetterSharedFoo();
      // std::bad_weak_ptr
      //    auto ptr7 = ptr6->getPtr();
    }

    std::shared_ptr<common::Object> ptr7;
    {
      auto shared_ptr = std::make_shared<SharedFoo>();
      ptr7 = shared_ptr;
      common::println("use count: ", shared_ptr.use_count());
    }
    common::println("use count: ", ptr7.use_count());
    common::println("shared_ptr size: ", sizeof(ptr7));
  }
  {
    common::println("-------- weak_ptr part");
    {
      common::println("cycle with shared_ptr");
      auto parent = std::make_shared<Parent>();
      auto child = std::make_shared<Child>();
      parent->child = child;
      child->parent = parent;
      child->parent->clearCounts();
      // Destructors are not called
    }
    {
      common::println("cycle with weak_ptr");
      auto parent = std::make_shared<Parent>();
      auto child = std::make_shared<Child>();
      parent->child = child;
      child->weakParent = std::weak_ptr(parent);
      child->weakParent.lock()->clearCounts();
      // Destructors are called
    }
    std::weak_ptr<common::Object> wptr;
    {
      auto ptr = std::make_shared<common::Object>();
      wptr = ptr;
      std::shared_ptr<common::Object> ptr2(wptr);
      // Destructor for ptr object is called, but allocated memory is not released
      // as we used make_shared, so object and control block lives together
    }
    // Expired
    //    std::shared_ptr<common::Object> ptr(wptr);
    common::println("weak_ptr size: ", sizeof(wptr));
  }
  common::println("-------- END");
}