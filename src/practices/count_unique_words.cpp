#include <filesystem>
#include <fstream>
#include <iostream>
#include <ranges>
#include <string_view>
#include <thread>
#include <unordered_set>
#include <vector>

std::size_t countUniqueWords(const std::string& content) {
  std::atomic<std::size_t> totalCount = 0;
  {
    auto threadCount = std::thread::hardware_concurrency();
    std::vector<std::jthread> threads;
    threads.reserve(threadCount);
    for (unsigned int i = 0; i < threadCount; ++i) {
      threads.emplace_back([&, i] {
        std::unordered_set<std::string_view> set;
        set.reserve(content.size() / threadCount);
        for (const auto& word : std::views::split(content, ' ')) {
          if ((word.front() % threadCount) == i) {
            // Replace with next line when gcc will fix this bug
            // set.emplace(word.begin(), word.end());
            set.emplace(&*word.begin(), std::ranges::distance(word));
          }
        }
        totalCount += set.size();
      });
    }
  }
  return totalCount.load();
}

int main(int argc, char* argv[]) {
  // We consider that second argument should be filename
  auto filename = argv[1];
  if (!filename) {
    std::cerr << "Second argument should be specified as filename" << std::endl;
    return 1;
  }

  // Check whether such file exist
  if (!std::filesystem::is_regular_file(filename)) {
    std::cerr << "Specified file does not exist" << std::endl;
    return 1;
  }

  // Open file
  std::ifstream file(filename, std::ios::in);
  if (!file.is_open()) {
    std::cerr << "Cannot open specified file" << std::endl;
    return 1;
  }

  // Load file content into string
  auto filesize = std::filesystem::file_size(filename);
  std::string content(filesize, '\0');
  file.read(content.data(), filesize);

  // Count unique words
  auto count = countUniqueWords(content);
  std::cout << "Unique words count: " << count << std::endl;

  return 0;
}