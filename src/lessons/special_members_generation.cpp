#include "common/object.hpp"
#include "common/utils.hpp"

struct GeneratedMember {
  struct DefCtor;
  struct CopyCtor;
  struct MoveCtor;
  struct CopyAssign;
  struct MoveAssign;
};

struct WithoutAny {
  common::Object object;
} instance;

struct WithDefaultConstructor : WithoutAny {
  WithDefaultConstructor() = default;
};

struct WithUserDefinedConstructor : WithoutAny {
  WithUserDefinedConstructor(int){};
};

struct WithCopyConstructor : WithoutAny {
  WithCopyConstructor(const WithCopyConstructor&) = default;
};

struct WithMoveConstructor : WithoutAny {
  WithMoveConstructor(WithMoveConstructor&&) = default;
};

struct WithCopyAssignment : WithoutAny {
  WithCopyAssignment& operator=(const WithCopyAssignment&) = default;
};

struct WithMoveAssignment : WithoutAny {
  WithMoveAssignment& operator=(WithMoveAssignment&&) = default;
};

struct WithDestructor : WithoutAny {
  ~WithDestructor() = default;
};

template <typename T, typename M>
void testGeneratedMember() {
  if constexpr (std::is_same_v<M, GeneratedMember::DefCtor>) {
    T t; // Default ctor
  } else if constexpr (std::is_same_v<M, GeneratedMember::CopyCtor>) {
    T t(static_cast<T&>(instance)); // Copy ctor
  } else if constexpr (std::is_same_v<M, GeneratedMember::MoveCtor>) {
    T t(std::move(static_cast<T&>(instance))); // Move ctor
  } else if constexpr (std::is_same_v<M, GeneratedMember::CopyAssign>) {
    static_cast<T&>(instance) = static_cast<T&>(instance); // Copy assignment
  } else if constexpr (std::is_same_v<M, GeneratedMember::MoveAssign>) {
    static_cast<T&>(instance) = std::move(static_cast<T&>(instance)); // Move assignment
  }
}

int main() {
  common::print("WithoutAny case:");
  testGeneratedMember<WithoutAny, GeneratedMember::DefCtor>();
  testGeneratedMember<WithoutAny, GeneratedMember::CopyCtor>();
  testGeneratedMember<WithoutAny, GeneratedMember::MoveCtor>();
  testGeneratedMember<WithoutAny, GeneratedMember::CopyAssign>();
  testGeneratedMember<WithoutAny, GeneratedMember::MoveAssign>();

  common::print("WithDefaultConstructor case:");
  testGeneratedMember<WithDefaultConstructor, GeneratedMember::DefCtor>();
  testGeneratedMember<WithDefaultConstructor, GeneratedMember::CopyCtor>();
  testGeneratedMember<WithDefaultConstructor, GeneratedMember::MoveCtor>();
  testGeneratedMember<WithDefaultConstructor, GeneratedMember::CopyAssign>();
  testGeneratedMember<WithDefaultConstructor, GeneratedMember::MoveAssign>();

  common::print("WithUserDefinedConstructor case:");
  //  testGeneratedMember<WithUserDefinedConstructor, GeneratedMember::DefCtor>();
  testGeneratedMember<WithUserDefinedConstructor, GeneratedMember::CopyCtor>();
  testGeneratedMember<WithUserDefinedConstructor, GeneratedMember::MoveCtor>();
  testGeneratedMember<WithUserDefinedConstructor, GeneratedMember::CopyAssign>();
  testGeneratedMember<WithUserDefinedConstructor, GeneratedMember::MoveAssign>();

  common::print("WithCopyConstructor case:");
  //  testGeneratedMember<WithCopyConstructor, GeneratedMember::DefCtor>();
  testGeneratedMember<WithCopyConstructor, GeneratedMember::CopyCtor>();
  testGeneratedMember<WithCopyConstructor, GeneratedMember::MoveCtor>(); // used copy instead
  testGeneratedMember<WithCopyConstructor, GeneratedMember::CopyAssign>();
  testGeneratedMember<WithCopyConstructor, GeneratedMember::MoveAssign>(); // used copy instead

  common::print("WithMoveConstructor case:");
  //  testGeneratedMember<WithMoveConstructor, GeneratedMember::DefCtor>();
  //  testGeneratedMember<WithMoveConstructor, GeneratedMember::CopyCtor>();
  testGeneratedMember<WithMoveConstructor, GeneratedMember::MoveCtor>();
  //  testGeneratedMember<WithMoveConstructor, GeneratedMember::CopyAssign>();
  //  testGeneratedMember<WithMoveConstructor, GeneratedMember::MoveAssign>();

  common::print("WithCopyAssignment case:");
  testGeneratedMember<WithCopyAssignment, GeneratedMember::DefCtor>();
  testGeneratedMember<WithCopyAssignment, GeneratedMember::CopyCtor>();
  testGeneratedMember<WithCopyAssignment, GeneratedMember::MoveCtor>(); // used copy instead
  testGeneratedMember<WithCopyAssignment, GeneratedMember::CopyAssign>();
  testGeneratedMember<WithCopyAssignment, GeneratedMember::MoveAssign>(); // used copy instead

  common::print("WithMoveAssignment case:");
  testGeneratedMember<WithMoveAssignment, GeneratedMember::DefCtor>();
  //  testGeneratedMember<WithMoveAssignment, GeneratedMember::CopyCtor>();
  //  testGeneratedMember<WithMoveAssignment, GeneratedMember::MoveCtor>();
  //  testGeneratedMember<WithMoveAssignment, GeneratedMember::CopyAssign>();
  testGeneratedMember<WithMoveAssignment, GeneratedMember::MoveAssign>();

  common::print("WithDestructor case:");
  testGeneratedMember<WithDestructor, GeneratedMember::DefCtor>();
  testGeneratedMember<WithDestructor, GeneratedMember::CopyCtor>();
  testGeneratedMember<WithDestructor, GeneratedMember::MoveCtor>(); // used copy instead
  testGeneratedMember<WithDestructor, GeneratedMember::CopyAssign>();
  testGeneratedMember<WithDestructor, GeneratedMember::MoveAssign>(); // used copy instead

  // Nice table can be found here - https://youtu.be/ECoLo17nG5c?t=1713
}