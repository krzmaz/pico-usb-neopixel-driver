#pragma once

#ifdef DEBUG_LOGS
template <typename T>
std::ostream &operator<<(std::ostream &output, std::vector<T> const &values) {
  output << "DUMP: ";
  for (auto const &value : values) {
    output << (int)value << ", ";
  }
  output << std::endl;
  return output;
}

#define LOG(stuff) std::cout << stuff << std::endl
#else
#define LOG(stuff) (void)0
#endif