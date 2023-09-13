#include <array>
#include <iostream>
#include <string_view>
#include <thread>
#include <utility>

#define RUN_FUNC(func)                                                                                       \
  std::cout << #func << ": ";                                                                                \
  std::cout << std::flush;                                                                                   \
  func();                                                                                                    \
  std::cout << std::endl;

constexpr int seconds_to_write = 10;
constexpr std::string_view str_to_write = "something... ";

void check_whether_standard_streams_are_tied() {
  std::array<std::pair<std::basic_ios<char>*, std::string_view>, 4> standard_streams{
      std::pair{&std::cin, "cin"},
      std::pair{&std::cout, "cout"},
      std::pair{&std::cerr, "cerr"},
      std::pair{&std::clog, "clog"},
  };
  for (auto [stream, name] : standard_streams) {
    auto* tied = stream->tie();
    if (tied == nullptr) {
      std::cout << name << " - untied; ";
    } else {
      std::cout << name << " - tied; ";
    }
  }
}

void writing_to_cout() {
  for (auto i = 0; i < seconds_to_write; ++i) {
    std::cout << str_to_write;
    // std::cout is buffered thus meaning it may not write immediately to standard output,
    // but wait till buffer is full or some other circumstances
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
}

void writing_to_cout_with_flush() {
  for (auto i = 0; i < seconds_to_write; ++i) {
    std::cout << str_to_write;
    // Flush std::cout manually, so it will write to standard output
    std::cout.flush();
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
}

void writing_to_cout_with_unitbuf() {
  // Set std::unitbuf to flush the output after each output operation
  std::cout << std::unitbuf;
  for (auto i = 0; i < seconds_to_write; ++i) {
    std::cout << str_to_write;
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
  // Remember to set it back
  std::cout << std::nounitbuf;
}

void writing_to_cout_with_endl() {
  for (auto i = 0; i < seconds_to_write; ++i) {
    // std::endl cause flushing of std::cout
    std::cout << str_to_write << std::endl;
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
}

void writing_to_cout_with_newline() {
  for (auto i = 0; i < seconds_to_write; ++i) {
    // Newline also can cause flushing of std::cout
    // Explained here - https://asitdhal.medium.com/difference-between-std-endl-and-n-b847529a0401
    std::cout << str_to_write << '\n';
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
}

void writing_to_cout_with_cerr_usage() {
  for (auto i = 0; i < seconds_to_write; ++i) {
    std::cout << str_to_write;
    // std::cerr is tied with std::cout, so each write to std::cerr cause flush on std::cout
    std::cerr << 'e';
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
}

void writing_to_cout_with_untied_cerr_usage() {
  // Untied std::cerr won't cause flush on std::cout
  std::cerr.tie(nullptr);
  for (auto i = 0; i < seconds_to_write; ++i) {
    std::cout << str_to_write;
    std::cerr << 'e';
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
  // Remember to tie it back
  std::cerr.tie(&std::cout);
}

void writing_to_cout_with_clog_usage() {
  for (auto i = 0; i < seconds_to_write; ++i) {
    std::cout << str_to_write;
    // std::clog is not tied with std::cout by default
    std::clog << 'l';
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
}

void writing_to_cout_with_tied_clog_usage() {
  // std::clog is not tied with std::cout by default, but can be tied
  std::clog.tie(&std::cout);
  for (auto i = 0; i < seconds_to_write; ++i) {
    std::cout << str_to_write;
    std::clog << 'l';
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
  // Remember to untie it back
  std::clog.tie(nullptr);
}

void writing_to_cout_with_cin_usage() {
  for (auto i = 0; i < seconds_to_write; ++i) {
    std::cout << str_to_write;
    // std::cin is tied with std::cout, so each read from std::cin cause flush on std::cout
    std::cin >> i;
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
}

void writing_to_cout_with_untied_cin_usage() {
  // Untied std::cin won't cause flush on std::cout
  std::cin.tie(nullptr);
  // But to fully untie std::cout from std::cin also this is needed
  std::ostream::sync_with_stdio(false);

  for (auto i = 0; i < seconds_to_write; ++i) {
    std::cout << str_to_write;
    std::cin >> i;
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
  // Remember to tie and sync it back
  std::cin.tie(&std::cout);
  std::ostream::sync_with_stdio(true);
}

int main() {
  RUN_FUNC(check_whether_standard_streams_are_tied)
  RUN_FUNC(writing_to_cout)
  RUN_FUNC(writing_to_cout_with_flush)
  RUN_FUNC(writing_to_cout_with_unitbuf)
  RUN_FUNC(writing_to_cout_with_endl)
  RUN_FUNC(writing_to_cout_with_newline)
  RUN_FUNC(writing_to_cout_with_cerr_usage)
  RUN_FUNC(writing_to_cout_with_untied_cerr_usage)
  RUN_FUNC(writing_to_cout_with_clog_usage)
  RUN_FUNC(writing_to_cout_with_tied_clog_usage)
  RUN_FUNC(writing_to_cout_with_cin_usage)
  RUN_FUNC(writing_to_cout_with_untied_cin_usage)
}
