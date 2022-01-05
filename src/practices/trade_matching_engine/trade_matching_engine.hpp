#include <chrono>
#include <iostream>
#include <set>
#include <sstream>
#include <vector>

namespace trade_matching {

enum class Side { Buy, Sell };

struct Order {
  std::string trader;
  Side side;
  mutable std::size_t quantity;
  std::size_t price;
  std::chrono::system_clock::time_point time;

  bool operator<(const Order& o) const {
    if (price < o.price) {
      return true;
    }
    if (price > o.price) {
      return false;
    }
    // Oldest should always be first
    if (time < o.time) {
      return true;
    }
    if (time > o.time) {
      return false;
    }
    return trader < o.trader;
    // We expect that one trader can make only one order in one time point
  }

  bool operator>(const Order& o) const {
    if (price > o.price) {
      return true;
    }
    if (price < o.price) {
      return false;
    }
    // Oldest should always be first
    if (time < o.time) {
      return true;
    }
    if (time > o.time) {
      return false;
    }
    return trader > o.trader;
    // We expect that one trader can make only one order in one time point
  }
};

struct Trade {
  std::string trader;
  Side side;
  mutable std::size_t quantity;
  std::size_t price;

  bool operator<(const Trade& other) const {
    if (trader < other.trader) {
      return true;
    }
    if (trader > other.trader) {
      return false;
    }
    if (side < other.side) {
      return true;
    }
    if (side > other.side) {
      return false;
    }
    return price < other.price;
  }
};

class OrderReader {
public:
  OrderReader(std::istream& input, Order& order) : m_input(input), m_order(order) {}

  bool read() {
    std::string line;
    if (!std::getline(m_input, line)) {
      return false;
    }

    // In case if Ctrl+D doesn't work
    if (line == "EXIT") {
      return false;
    }

    std::istringstream iss(line);
    m_order = deserializeOrder(iss);
    return true;
  }

private:
  static Order deserializeOrder(std::istringstream& iss) {
    Order order;
    order.time = std::chrono::system_clock::now();

    iss >> order.trader;
    if (iss.fail()) {
      throw std::runtime_error("Cannot parse trader id.");
    }

    char side;
    iss >> side;
    if (iss.fail()) {
      throw std::runtime_error("Cannot parse side.");
    }
    switch (side) {
    case 'B':
      order.side = Side::Buy;
      break;
    case 'S':
      order.side = Side::Sell;
      break;
    default:
      throw std::runtime_error("Unknown side type.");
    }

    iss >> order.quantity;
    if (iss.fail()) {
      throw std::runtime_error("Cannot parse quantity.");
    }

    iss >> order.price;
    if (iss.fail()) {
      throw std::runtime_error("Cannot parse price.");
    }

    return order;
  }

  std::istream& m_input;
  Order& m_order;
};

class TradeWriter {
public:
  void write(const std::set<Trade>& trades) {
    if (trades.empty()) {
      return;
    }
    for (const auto& trade : trades) {
      serializeTrade(trade);
      m_output << ' ';
    }
    m_output.seekp(-1, std::ios_base::end);
    m_output << '\n';
  }

  std::string getString() const { return m_output.str(); }

private:
  void serializeTrade(const Trade& trade) {
    m_output << trade.trader;
    switch (trade.side) {
    case Side::Buy:
      m_output << '+';
      break;
    case Side::Sell:
      m_output << '-';
      break;
    default:
      throw std::runtime_error("Unknown side type.");
    }
    m_output << trade.quantity;
    m_output << '@';
    m_output << trade.price;
  }

  std::ostringstream m_output;
};

class Engine {
public:
  void run(std::istream& input) {
    Order order;
    OrderReader orderReader(input, order);
    while (orderReader.read()) {
      processAggressor(std::move(order));
    }
  }

  std::string getSerializedTrades() {
    TradeWriter tradeWriter;
    for (const auto& trades : m_tradesList) {
      // Sum up all quantities for same trader, side and price
      std::set<Trade> mergedTrades;
      for (const auto& trade : trades) {
        auto [it, inserted] = mergedTrades.insert(trade);
        if (!inserted) {
          it->quantity += trade.quantity;
        }
      }
      tradeWriter.write(mergedTrades);
    }
    return tradeWriter.getString();
  }

private:
  void processAggressor(Order&& aggressor) {
    switch (aggressor.side) {
    case Side::Buy:
      makeTrades(std::move(aggressor), m_restingSellQueue, m_restingBuyQueue);
      break;
    case Side::Sell:
      makeTrades(std::move(aggressor), m_restingBuyQueue, m_restingSellQueue);
      break;
    default:
      throw std::runtime_error("Unknown side type.");
    }
  }

  template <class FromQueue, class ToQueue>
  void makeTrades(Order&& aggressor, FromQueue& fromQueue, ToQueue& toQueue) {
    std::vector<Trade> trades;

    while (aggressor.quantity != 0) {
      if (fromQueue.empty()) {
        toQueue.insert(std::move(aggressor));
        break;
      }

      auto resting = fromQueue.begin();
      if (fromQueue.key_comp()(aggressor.price, resting->price)) {
        toQueue.insert(std::move(aggressor));
        break;
      }

      auto quantity = std::min(resting->quantity, aggressor.quantity);
      aggressor.quantity -= quantity;
      resting->quantity -= quantity;

      trades.emplace_back(aggressor.trader, aggressor.side, quantity, resting->price);
      trades.emplace_back(resting->trader, resting->side, quantity, resting->price);

      if (resting->quantity == 0) {
        fromQueue.erase(resting);
      }
    }

    if (!trades.empty()) {
      m_tradesList.push_back(std::move(trades));
    }
  }

  std::set<Order, std::greater<>> m_restingBuyQueue;
  std::set<Order, std::less<>> m_restingSellQueue;
  std::vector<std::vector<Trade>> m_tradesList;
};

} // namespace trade_matching