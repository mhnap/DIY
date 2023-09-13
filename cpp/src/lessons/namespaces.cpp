// Unnamed namespaces are superior to the static keyword, primarily because the keyword static applies only
// to the variables declarations and functions, not to the user-defined types.

static int si = 0;
static int sf();
// error: a storage class can only be specified for objects and functions
// static struct sc {
//  int ci;
//};

namespace {
int ni = 0;
int nf();
struct nc {
  int ni;
};
} // namespace

nc NC{ni};

// Need to remember ODR (One Definition Rule)
namespace {
int i; // defines ::(unique)::i
}

void f() {
  i++; // increments ::(unique)::i
}

namespace A {
namespace {
int i; // A::(unique)::i
int j; // A::(unique)::j
} // namespace

void g() { i++; } // A::(unique)::i++
} // namespace A

using namespace A; // introduces all names from A into global namespace

int main() {
  // i++; // error: ::(unique)::i and ::A::(unique)::i are both in scope
  A::i++; // ok, increments ::A::(unique)::i
  j++; // ok, increments ::A::(unique)::j
}
