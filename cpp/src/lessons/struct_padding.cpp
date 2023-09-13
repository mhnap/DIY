#include "common/utils.hpp"

struct NonPackedStruct {
  char c;
  int i;
  char h;
};

struct PackedStruct {
  char c;
  int i;
  char h;
} __attribute__((packed));

struct NonPackedRightStruct {
  int i;
  char c;
  char h;
};

struct PackedRightStruct {
  int i;
  char c;
  char h;
} __attribute__((packed));

struct NonPackedStructWithBigMember {
  char c;
  long long i;
  char h;
};

struct PackedStructWithBigMember {
  char c;
  long long i;
  char h;
} __attribute__((packed));

struct AlignedStruct {
  int i;
  char c;
  char h;
} __attribute__((aligned(64)));

struct EmptyStruct {};

int main() {
  common::println("Size of non packed struct is ", sizeof(NonPackedStruct));
  common::println("Size of packed struct is ", sizeof(PackedStruct));
  common::println("Size of non packed right struct is ", sizeof(NonPackedRightStruct));
  common::println("Size of packed right struct is ", sizeof(PackedRightStruct));
  common::println("Size of non packed struct with big member is ", sizeof(NonPackedStructWithBigMember));
  common::println("Size of packed struct with big member is ", sizeof(PackedStructWithBigMember));
  common::println("Size of aligned struct is ", sizeof(AlignedStruct));
  common::println("Size of empty struct is ", sizeof(EmptyStruct));
}