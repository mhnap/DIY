#include "trade_matching_engine.hpp"

int main() {
  trade_matching::Engine engine;
  engine.run(std::cin);
  std::cout << engine.getSerializedTrades();
}