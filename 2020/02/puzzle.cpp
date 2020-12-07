#include <algorithm>
#include <array>
#include <charconv>
#include <filesystem>
#include <fmt/format.h>
#include <fstream>
#include <string_view>
#include <vector>

namespace ranges = std::ranges;
namespace fs = std::filesystem;

struct password_policy {
    int first_number;
    int second_number;
    char letter;
};

bool validate_old_policies(password_policy policy, std::string_view password) {
    auto letter_amount = ranges::count(password, policy.letter);
    return (letter_amount >= policy.first_number) && (letter_amount <= policy.second_number);
}

bool validate_toboggan_policies(password_policy policy, std::string_view password) {
    auto contains_letter_at = [&](std::size_t position) {
        return password.at(position - 1) == policy.letter;
    };

    return contains_letter_at(policy.first_number) != contains_letter_at(policy.second_number);
}

constexpr std::array<std::pair<password_policy, std::string_view>, 3> test_input{{
    {{1, 3, 'a'}, "abcde"},
    {{1, 3, 'b'}, "cdefg"},
    {{2, 9, 'c'}, "ccccccccc"},
}};

bool test1() {
    return ranges::count_if(test_input, [](auto entry) {
               return validate_old_policies(entry.first, entry.second);
           }) == 2;
}

bool test2() {
    return ranges::count_if(test_input, [](auto entry) {
               return validate_toboggan_policies(entry.first, entry.second);
           }) == 1;
}

auto parse_input(fs::path file) {
    std::ifstream f(file);
    if (!f) {
        fmt::print("File not found\n");
        std::abort();
    }

    std::vector<std::pair<password_policy, std::string>> entries;

    for (std::string line; std::getline(f, line);) {
        password_policy policy{};

        auto begin = line.c_str();
        auto end = begin + line.length();

        if (auto [p, ec] = std::from_chars(begin, end, policy.first_number); ec == std::errc{}) {
            begin = p + 1;
        }

        if (auto [p, ec] = std::from_chars(begin, end, policy.second_number); ec == std::errc{}) {
            begin = p + 1;
        }

        policy.letter = *begin;
        begin += 3;

        entries.emplace_back(policy, std::string(begin, end));
    }

    return entries;
}

int main() {
    auto const input = parse_input("./input");

    fmt::print("*** Puzzle 1\n");

    if (test1()) {
        fmt::print("Test passed!\n");
    } else {
        fmt::print("Test not passed\n");
        std::exit(-1);
    }

    fmt::print("Number of valid passwords: {}\n", ranges::count_if(input, [](auto entry) {
                   return validate_old_policies(entry.first, entry.second);
               }));

    fmt::print("\n*** Puzzle 2\n");

    if (test2()) {
        fmt::print("Test passed!\n");
    } else {
        fmt::print("Test not passed\n");
        std::exit(-1);
    }

    fmt::print("Number of valid passwords: {}\n", ranges::count_if(input, [](auto entry) {
                   return validate_toboggan_policies(entry.first, entry.second);
               }));
}
