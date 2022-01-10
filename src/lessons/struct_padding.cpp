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
  common::print("Size of non packed struct is ", sizeof(NonPackedStruct));
  common::print("Size of packed struct is ", sizeof(PackedStruct));
  common::print("Size of non packed right struct is ", sizeof(NonPackedRightStruct));
  common::print("Size of packed right struct is ", sizeof(PackedRightStruct));
  common::print("Size of non packed struct with big member is ", sizeof(NonPackedStructWithBigMember));
  common::print("Size of packed struct with big member is ", sizeof(PackedStructWithBigMember));
  common::print("Size of aligned struct is ", sizeof(AlignedStruct));
  common::print("Size of empty struct is ", sizeof(EmptyStruct));
}