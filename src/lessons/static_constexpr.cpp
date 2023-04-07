// https://stackoverflow.com/a/58328980

#include <cassert>
#include <iostream>
#include <sstream>
#include <string>

const short const_short = 0;
constexpr short constexpr_short = 0;

// print only last 3 address value numbers
const short addr_offset = 3;

// This function will print name, value and address for given parameter
void print_properties(std::string ref_name, const short* param, short offset) {
  // determine initial size of strings
  std::string title = "value \\ address of ";
  const size_t ref_size = ref_name.size();
  const size_t title_size = title.size();
  assert(title_size > ref_size);

  // create title (resize)
  title.append(ref_name);
  title.append(" is ");
  title.append(title_size - ref_size, ' ');

  // extract last 'offset' values from address
  std::stringstream addr;
  addr << param;
  const std::string addr_str = addr.str();
  const size_t addr_size = addr_str.size();
  assert(addr_size - offset > 0);

  // print title / ref value / address at offset
  std::cout << title << *param << " " << addr_str.substr(addr_size - offset) << std::endl;
}

// here we test initialization of const variable (runtime)
void const_value(const short counter) {
  static short temp = const_short;
  const short const_var = ++temp;
  print_properties("const", &const_var, addr_offset);

  if (counter)
    const_value(counter - 1);
}

// here we test initialization of static variable (runtime)
void static_value(const short counter) {
  static short temp = const_short;
  static short static_var = ++temp;
  print_properties("static", &static_var, addr_offset);

  if (counter)
    static_value(counter - 1);
}

// here we test initialization of static const variable (runtime)
void static_const_value(const short counter) {
  static short temp = const_short;
  static const short static_var = ++temp;
  print_properties("static const", &static_var, addr_offset);

  if (counter)
    static_const_value(counter - 1);
}

// here we test initialization of constexpr variable (compile time)
void constexpr_value(const short counter) {
  constexpr short constexpr_var = constexpr_short;
  print_properties("constexpr", &constexpr_var, addr_offset);

  if (counter)
    constexpr_value(counter - 1);
}

// here we test initialization of static constexpr variable (compile time)
void static_constexpr_value(const short counter) {
  static constexpr short static_constexpr_var = constexpr_short;
  print_properties("static constexpr", &static_constexpr_var, addr_offset);

  if (counter)
    static_constexpr_value(counter - 1);
}

// final test call this method from main()
void test_static_const() {
  constexpr short counter = 2;

  const_value(counter);
  std::cout << std::endl;

  static_value(counter);
  std::cout << std::endl;

  static_const_value(counter);
  std::cout << std::endl;

  constexpr_value(counter);
  std::cout << std::endl;

  static_constexpr_value(counter);
  std::cout << std::endl;
}

int main() { test_static_const(); }
