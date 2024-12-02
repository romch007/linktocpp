#include <vector>
#include <numeric>
#include <cstddef>

template <typename T>
T cpp_average(const std::vector<T> numbers) {
  return std::accumulate(numbers.begin(), numbers.end(), 0.0) / numbers.size();
}

extern "C" double lib_average(const double* numbers, size_t len) {
  std::vector<double> vec_numbers(numbers, numbers + len);
  return cpp_average(vec_numbers);
}
