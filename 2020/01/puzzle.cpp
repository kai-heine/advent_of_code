#include <algorithm>
#include <cassert>
#include <charconv>
#include <filesystem>
#include <fmt/format.h>
#include <fstream>
#include <optional>
#include <ranges>
#include <string>
#include <tuple>
#include <vector>

namespace fs = std::filesystem;
namespace ranges = std::ranges;

auto read_list(fs::path file) {
    std::ifstream f(file);
    if (!f) {
        std::abort();
    }

    std::vector<int> entries;

    for (std::string line; std::getline(f, line);) {
        int entry = 0;
        if (auto [p, ec] = std::from_chars(line.c_str(), line.c_str() + line.length(), entry);
            ec == std::errc{}) {
            entries.push_back(entry);
        }
    }

    return entries;
}

template <ranges::forward_range Range, typename T = ranges::range_value_t<Range>>
auto find_summands(Range const& numbers, T sum, T init = T{}) -> std::optional<std::pair<T, T>> {
    assert(ranges::is_sorted(numbers));

    auto end = std::upper_bound(numbers.begin(), numbers.end(), sum - init);

    for (auto begin = numbers.begin(); begin != end; ++begin) {
        T diff = sum - (*begin + init);
        if (auto found = std::lower_bound(begin, end, diff); (found != end) && (*found == diff)) {
            return std::pair{*begin, *found};
        }
    }
    return std::nullopt;
}

template <ranges::forward_range Range, typename T = ranges::range_value_t<Range>>
auto find_three_summands(Range const& numbers, T sum) -> std::optional<std::tuple<T, T, T>> {
    assert(ranges::is_sorted(numbers));

    for (auto begin = numbers.begin(); begin != numbers.end(); ++begin) {
        if (auto found =
                find_summands(ranges::subrange{std::next(begin), numbers.end()}, sum, *begin);
            found.has_value()) {
            return std::tuple{*begin, found->first, found->second};
        }
    }
    return std::nullopt;
}

bool test1() {
    std::vector<int> test_input = {1721, 979, 366, 299, 675, 1456};
    ranges::sort(test_input);

    auto result = find_summands(test_input, 2020);

    return result.has_value() && (result.value() == std::pair{299, 1721}) &&
           (result->first * result->second == 514579);
}

bool test2() {
    std::vector<int> test_input = {1721, 979, 366, 299, 675, 1456};
    ranges::sort(test_input);

    auto result = find_three_summands(test_input, 2020);

    if (!result.has_value()) {
        return false;
    }

    if (result.value() != std::tuple{366, 675, 979}) {
        return false;
    }

    auto product = std::get<0>(*result) * std::get<1>(*result) * std::get<2>(*result);

    return product == 241861950;
}

int main() {
    fmt::print("*** Puzzle 1\n");

    if (test1()) {
        fmt::print("Test passed!\n");
    } else {
        fmt::print("Test not passed\n");
        std::exit(-1);
    }

    auto numbers = read_list("./input");
    ranges::sort(numbers);

    /* Advent of Code Day 1
     * First puzzle:
     * Find the two entries that sum to 2020;
     * what do you get if you multiply them together?
     */

    if (auto found = find_summands(numbers, 2020); found.has_value()) {
        fmt::print("Found: {} * {} = {}\n", found->first, found->second,
                   found->first * found->second);
    } else {
        fmt::print("Not found!\n");
    }

    /* Second puzzle:
     * What is the product of the three entries that sum to 2020?
     */

    fmt::print("\n*** Puzzle 2\n");
    if (test2()) {
        fmt::print("Test passed!\n");
    } else {
        fmt::print("Test not passed\n");
        std::exit(-1);
    }

    if (auto found = find_three_summands(numbers, 2020); found.has_value()) {
        fmt::print("Found: {} * {} * {} = {}\n", std::get<0>(*found), std::get<1>(*found),
                   std::get<2>(*found),
                   std::get<0>(*found) * std::get<1>(*found) * std::get<2>(*found));
    } else {
        fmt::print("Not found!\n");
    }
}
