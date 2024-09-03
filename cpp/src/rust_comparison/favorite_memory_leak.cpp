// https://www.youtube.com/watch?v=LKKmPAQFNgE

#include <vector>

struct V: std::vector<V> {};

int main() {
    V v;
    v.emplace_back();
    v.swap(v.front());
}
