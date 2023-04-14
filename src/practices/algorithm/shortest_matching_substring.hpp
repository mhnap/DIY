#include <cstring>
#include <list>
#include <queue>
#include <string>
#include <string_view>

namespace algorithm::shortest_matching_substring {

struct Substring {
  size_t begin;
  size_t end;
  bool operator<(const Substring& other) const { return end - begin > other.end - other.begin; }
};

struct PartialSubstring {
  size_t start_index;
  std::string chars_to_check;
};

// Given two strings s and t find the shortest substring of s that contains all characters of t.
std::string v1(std::string_view base_string, std::string_view chars_to_check) {
  // Contains final substrings
  std::priority_queue<Substring> final_substrings;
  // Contains partial substrings that are still need to check
  std::list<PartialSubstring> partial_substrings;

  // Iterate through each base char
  for (auto base_string_index = 0U; base_string_index < base_string.size(); ++base_string_index) {
    const auto base_char = base_string[base_string_index];

    // Check already existed partial substrings
    for (auto partial_substrings_it = partial_substrings.begin();
         partial_substrings_it != partial_substrings.end(); ++partial_substrings_it) {
      // Try to find base char in current partial_substring chars to check
      const auto base_char_pos = partial_substrings_it->chars_to_check.find(base_char);
      if (base_char_pos != std::string::npos) {
        // Found new char, remove it from chars_to_check
        partial_substrings_it->chars_to_check.erase(base_char_pos, 1);
        // Check whether we can move current substring from partial to final
        if (partial_substrings_it->chars_to_check.empty()) {
          // Add current substring to final substrings
          final_substrings.emplace(partial_substrings_it->start_index, base_string_index);
          // Remove current substring from partial substrings
          partial_substrings.erase(partial_substrings_it--);
        }
      }
    }

    // Check for new partial substring
    const auto base_char_pos = chars_to_check.find(base_char);
    if (base_char_pos != std::string::npos) {
      const auto new_chars_to_check = std::string(chars_to_check).erase(base_char_pos, 1);
      if (!new_chars_to_check.empty()) {
        partial_substrings.emplace_back(base_string_index, new_chars_to_check);
      } else {
        final_substrings.emplace(base_string_index, base_string_index);
      }
    }
  }

  // Check whether we really found anything
  if (final_substrings.empty()) {
    return {};
  }

  // Get the shortest substring
  const auto shortest_substring = final_substrings.top();
  // Construct string from it
  std::string shortest(base_string.begin() + shortest_substring.begin,
                       base_string.begin() + shortest_substring.end + 1);

  return shortest;
}

// From web
#define MAX_CHARS 256
std::string v2(std::string_view s, std::string_view t) {
  if (t.empty() || s.empty()) {
    return {};
  }

  int sLen = strlen(s.data()), tLen = strlen(t.data());
  int sFreq[MAX_CHARS]{};
  int tFreq[MAX_CHARS]{};

  for (int i = 0; i < tLen; i++) {
    tFreq[t[i]]++;
  }
  int start = 0, minStart = 0, minLen = sLen + 1, count = 0;
  for (int end = 0; end < sLen; end++) {
    if (tFreq[s[end]] > 0) {
      sFreq[s[end]]++;
      if (sFreq[s[end]] <= tFreq[s[end]]) {
        count++;
      }
    }
    while (count == tLen) {
      if (end - start + 1 < minLen) {
        minStart = start;
        minLen = end - start + 1;
      }
      if (tFreq[s[start]] > 0) {
        sFreq[s[start]]--;
        if (sFreq[s[start]] < tFreq[s[start]]) {
          count--;
        }
      }
      start++;
    }
  }
  char* result = NULL;
  if (minLen <= sLen) {
    result = (char*)malloc((minLen + 1) * sizeof(char));
    memcpy(result, s.data() + minStart, minLen);
    result[minLen] = '\0';
  }
  return result ? std::string(result) : std::string{};
}

} // namespace algorithm::shortest_matching_substring
