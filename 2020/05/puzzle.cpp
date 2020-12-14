#define CATCH_CONFIG_RUNNER
#include <catch2/catch.hpp>

#include <bitset>
#include <cassert>
#include <filesystem>
#include <fmt/format.h>
#include <fstream>
#include <optional>
#include <ranges>
#include <string_view>
#include <vector>

namespace ranges = std::ranges;
namespace views = std::views;
namespace fs = std::filesystem;

auto to_seat_id(std::string_view boarding_pass) {
    return std::bitset<7>(boarding_pass.data(), 7, 'F', 'B').to_ulong() * 8 +
           std::bitset<3>(boarding_pass.data() + 7, 3, 'L', 'R').to_ulong();
}

auto row_number(unsigned long seat_id) { return seat_id / 8; }
auto column_number(unsigned long seat_id) { return seat_id % 8; }

auto read_boarding_passes(fs::path file) {
    std::ifstream f(file);
    if (!f) {
        fmt::print("Could not open file");
        std::exit(-1);
    }

    std::vector<unsigned long> seat_ids;
    for (std::string line; std::getline(f, line);) {
        seat_ids.push_back(to_seat_id(line));
    }
    return seat_ids;
}

TEST_CASE("boarding pass decoding") {
    CHECK(to_seat_id("FBFBBFFRLR") == 357);
    CHECK(row_number(357) == 44);
    CHECK(column_number(357) == 5);

    CHECK(to_seat_id("BFFFBBFRRR") == 567);
    CHECK(row_number(567) == 70);
    CHECK(column_number(567) == 7);

    CHECK(to_seat_id("FFFBBBFRRR") == 119);
    CHECK(row_number(119) == 14);
    CHECK(column_number(119) == 7);

    CHECK(to_seat_id("BBFFBBFRLL") == 820);
    CHECK(row_number(820) == 102);
    CHECK(column_number(820) == 4);
}

TEST_CASE("input reading") {
    auto seat_ids = read_boarding_passes("./test_input");
    CHECK(seat_ids == std::vector<unsigned long>{567, 119, 820});
}

int main(int argc, char** argv) {
    int test_result = Catch::Session().run(argc, argv);

    if (test_result != 0) {
        fmt::print("Test Failed\n");
        return test_result;
    }

    fmt::print("***Puzzle 1\n");
    auto seat_ids = read_boarding_passes("./input");
    fmt::print("Highest seat ID: {}\n", *ranges::max_element(seat_ids));

    fmt::print("\n***Puzzle 2\n");
    ranges::sort(seat_ids);
    if (auto found = ranges::adjacent_find(
            seat_ids, [](auto first, auto second) { return (second - first) > 1; });
        found != seat_ids.end()) {
        fmt::print("found: {} -> {}, my seat id = {}\n", *found, *(found + 1), *found + 1);
    } else {
        fmt::print("no seat id gap found!\n");
    }
}
