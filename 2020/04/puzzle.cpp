#define CATCH_CONFIG_RUNNER
#include <catch2/catch.hpp>

#include <algorithm>
#include <filesystem>
#include <fmt/format.h>
#include <fstream>
#include <map>
#include <string_view>
#include <vector>

namespace ranges = std::ranges;
namespace views = std::views;
namespace fs = std::filesystem;

using passport = std::map<std::string, std::string>;

auto read_passports(fs::path file) {
    std::ifstream f(file);

    std::vector<passport> passports;

    while (!f.eof()) {
        passport p;
        for (std::string line; std::getline(f, line) && !line.empty();) {
            for (std::istringstream s(line); !s.eof();) {
                std::string key, value;
                std::getline(s, key, ':');
                std::getline(s, value, ' ');
                p[key] = value;
            }
        }
        passports.emplace_back(std::move(p));
    }

    return passports;
}

bool valid(passport const& p) {
    static std::array<std::string, 7> const required_fields = {
        {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}};
    return ranges::all_of(required_fields, [&](auto key) { return p.contains(key); });
}

TEST_CASE("reading input") {
    auto const passports = read_passports("./test_input");
    CHECK(passports.size() == 4);
    auto const& first = passports.front();
    CHECK(first.size() == 8);
    CHECK(first.at("ecl") == "gry");
    CHECK(first.at("pid") == "860033327");
    CHECK(first.at("eyr") == "2020");
    CHECK(first.at("hcl") == "#fffffd");
    CHECK(first.at("byr") == "1937");
    CHECK(first.at("iyr") == "2017");
    CHECK(first.at("cid") == "147");
    CHECK(first.at("hgt") == "183cm");
}

TEST_CASE("passport validation") {
    auto const passports = read_passports("./test_input");

    CHECK(valid(passports[0]));
    CHECK(!valid(passports[1]));
    CHECK(valid(passports[2]));
    CHECK(!valid(passports[3]));
}

int main(int argc, char** argv) {
    int test_result = Catch::Session().run(argc, argv);

    if (test_result != 0) {
        fmt::print("Test Failed\n");
        return test_result;
    }

    fmt::print("***Puzzle 1\n");
    auto const passports = read_passports("./input");
    fmt::print("Number of valid passports: {}\n",
               ranges::count_if(passports, [](auto p) { return valid(p); }));
}
