#include <cstdio>

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

int main() {
  std::printf("Size of non packed struct is %d\n", sizeof(NonPackedStruct));
  std::printf("Size of packed struct is %d\n", sizeof(PackedStruct));
  std::printf("Size of non packed right struct is %d\n", sizeof(NonPackedRightStruct));
  std::printf("Size of packed right struct is %d\n", sizeof(PackedRightStruct));
  std::printf("Size of non packed struct with big member is %d\n", sizeof(NonPackedStructWithBigMember));
  std::printf("Size of packed struct with big member is %d\n", sizeof(PackedStructWithBigMember));
  std::printf("Size of aligned struct is %d\n", sizeof(AlignedStruct));
}