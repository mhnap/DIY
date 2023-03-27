#pragma once

#include <algorithm>
#include <tuple>
#include <vector>

#ifndef NDEBUG
#include <iostream>
#endif

// Helper macro to handle pair type sorting
#define CALL_SORT(sort_func)                                                                                 \
  if constexpr (std::is_same_v<T, Data>) {                                                                   \
    sort_func(result.begin(), result.end());                                                                 \
  } else {                                                                                                   \
    sort_func(result.begin(), result.end(), [](auto a, auto b) { return a.second < b.second; });             \
  }

namespace {

const auto default_comp = [](auto a, auto b) { return a < b; };

#ifndef NDEBUG
const auto debug_str = [](auto comp_count, auto swap_count) {
  return "Comparison count: " + std::to_string(comp_count) + "; Swap count: " + std::to_string(swap_count);
};
#endif

void bubble_sort_v1(const auto first, const auto last, const auto comp) {
  if (first == last) {
    return;
  }
#ifndef NDEBUG
  size_t comp_count = 0;
  size_t swap_count = 0;
#endif
  for (auto i = first; i != last; ++i) {
    for (auto current = std::next(first); current != last; ++current) {
      auto prev = std::prev(current);
      if (comp(*current, *prev)) {
        std::swap(*current, *prev);
#ifndef NDEBUG
        ++swap_count;
      }
      ++comp_count;
#else
      }
#endif
    }
  }
#ifndef NDEBUG
  std::cout << "bubble_sort_v1: " << debug_str(comp_count, swap_count) << std::endl;
#endif
}

void bubble_sort_v1(const auto first, const auto last) { bubble_sort_v1(first, last, default_comp); }

void bubble_sort_v2(const auto first, const auto last, const auto comp) {
  if (first == last) {
    return;
  }
#ifndef NDEBUG
  size_t comp_count = 0;
  size_t swap_count = 0;
#endif
  bool swapped = true;
  while (swapped) {
    swapped = false;
    for (auto current = std::next(first); current != last; ++current) {
      auto prev = std::prev(current);
      if (comp(*current, *prev)) {
        std::swap(*current, *prev);
        swapped = true;
#ifndef NDEBUG
        ++swap_count;
      }
      ++comp_count;
#else
      }
#endif
    }
  }
#ifndef NDEBUG
  std::cout << "bubble_sort_v2: " << debug_str(comp_count, swap_count) << std::endl;
#endif
}

void bubble_sort_v2(const auto first, const auto last) { bubble_sort_v2(first, last, default_comp); }

void bubble_sort_v3(const auto first, auto last, const auto comp) {
  if (first == last) {
    return;
  }
#ifndef NDEBUG
  size_t comp_count = 0;
  size_t swap_count = 0;
#endif
  bool swapped = true;
  while (swapped) {
    swapped = false;
    for (auto current = std::next(first); current != last; ++current) {
      auto prev = std::prev(current);
      if (comp(*current, *prev)) {
        std::swap(*current, *prev);
        swapped = true;
#ifndef NDEBUG
        ++swap_count;
      }
      ++comp_count;
#else
      }
#endif
    }
    --last;
  }
#ifndef NDEBUG
  std::cout << "bubble_sort_v3: " << debug_str(comp_count, swap_count) << std::endl;
#endif
}

void bubble_sort_v3(const auto first, auto last) { bubble_sort_v3(first, last, default_comp); }

void bubble_sort_v4(const auto first, auto last, const auto comp) {
  if (first == last) {
    return;
  }
#ifndef NDEBUG
  size_t comp_count = 0;
  size_t swap_count = 0;
#endif
  while (std::distance(first, last) > 1) {
    auto new_last = first;
    for (auto current = std::next(first); current != last; ++current) {
      auto prev = std::prev(current);
      if (comp(*current, *prev)) {
        std::swap(*current, *prev);
        new_last = current;
#ifndef NDEBUG
        ++swap_count;
      }
      ++comp_count;
#else
      }
#endif
    }
    last = new_last;
  }
#ifndef NDEBUG
  std::cout << "bubble_sort_v4: " << debug_str(comp_count, swap_count) << std::endl;
#endif
}

void bubble_sort_v4(const auto first, auto last) { bubble_sort_v4(first, last, default_comp); }

} // namespace

namespace algorithm::sort {

template <typename T, typename Data = T>
[[nodiscard]] auto v1(const std::vector<T>& vec) {
  auto result = vec;

  CALL_SORT(std::sort)

  return result;
}

template <typename T, typename Data = T>
[[nodiscard]] auto v2(const std::vector<T>& vec) {
  auto result = vec;

  CALL_SORT(std::stable_sort)

  return result;
}

template <typename T, typename Data = T>
[[nodiscard]] auto v3(const std::vector<T>& vec) {
  auto result = vec;

  CALL_SORT(bubble_sort_v1)

  return result;
}

template <typename T, typename Data = T>
[[nodiscard]] auto v4(const std::vector<T>& vec) {
  auto result = vec;

  CALL_SORT(bubble_sort_v2)

  return result;
}

template <typename T, typename Data = T>
[[nodiscard]] auto v5(const std::vector<T>& vec) {
  auto result = vec;

  CALL_SORT(bubble_sort_v3)

  return result;
}

template <typename T, typename Data = T>
[[nodiscard]] auto v6(const std::vector<T>& vec) {
  auto result = vec;

  CALL_SORT(bubble_sort_v4)

  return result;
}

} // namespace algorithm::sort
