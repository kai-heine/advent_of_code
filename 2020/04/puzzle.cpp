#define CATCH_CONFIG_RUNNER
#include <catch2/catch.hpp>

#include <algorithm>
#include <cctype>
#include <charconv>
#include <concepts>
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

bool contains_required_fields(passport const& p) {
    static std::array<std::string, 7> const required_fields = {
        {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}};
    return ranges::all_of(required_fields, [&](auto key) { return p.contains(key); });
}

template <std::integral T>
bool in_range(T value, T min, T max) {
    return (value >= min) && (value <= max);
}

bool valid_year(std::string_view yr, int min, int max) {
    int year = 0;
    if (auto [ptr, ec] = std::from_chars(yr.begin(), yr.end(), year); ec != std::errc{}) {
        fmt::print("Could not parse year\n");
        return false;
    }
    return in_range(year, min, max);
}

bool valid_birth_year(std::string_view byr) { return valid_year(byr, 1920, 2002); }
bool valid_issue_year(std::string_view iyr) { return valid_year(iyr, 2010, 2020); }
bool valid_expiration_year(std::string_view eyr) { return valid_year(eyr, 2020, 2030); }

bool valid_height(std::string_view hgt) {
    int height = 0;
    auto [ptr, ec] = std::from_chars(hgt.begin(), hgt.end(), height);
    if (ec != std::errc{}) {
        fmt::print("Could not parse height\n");
        return false;
    }
    auto unit = hgt.substr(std::distance(hgt.data(), ptr));
    if (unit == "cm") {
        return in_range(height, 150, 193);
    } else if (unit == "in") {
        return in_range(height, 59, 76);
    }
    return false;
}

bool valid_hair_color(std::string_view hcl) {
    return (hcl.length() == 7) && (hcl.front() == '#') && ranges::all_of(hcl.substr(1), [](char c) {
               return std::isxdigit(static_cast<unsigned char>(c));
           });
}

bool valid_eye_color(std::string_view ecl) {
    constexpr std::array<std::string_view, 7> valid_colors{
        {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}};
    return ranges::any_of(valid_colors, [&](auto color) { return color == ecl; });
}

bool valid_passport_id(std::string_view pid) {
    return (pid.length() == 9) &&
           ranges::all_of(pid, [](char c) { return std::isdigit(static_cast<unsigned char>(c)); });
}

bool valid_passport(passport const& p) {
    return contains_required_fields(p) && valid_birth_year(p.at("byr")) &&
           valid_issue_year(p.at("iyr")) && valid_expiration_year(p.at("eyr")) &&
           valid_height(p.at("hgt")) && valid_hair_color(p.at("hcl")) &&
           valid_eye_color(p.at("ecl")) && valid_passport_id(p.at("pid"));
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

TEST_CASE("passport validation 1") {
    auto const passports = read_passports("./test_input");

    CHECK(contains_required_fields(passports[0]));
    CHECK(!contains_required_fields(passports[1]));
    CHECK(contains_required_fields(passports[2]));
    CHECK(!contains_required_fields(passports[3]));
}

TEST_CASE("field validation 2") {
    CHECK(valid_birth_year("2002"));
    CHECK(!valid_birth_year("2003"));

    CHECK(valid_height("60in"));
    CHECK(valid_height("190cm"));
    CHECK(!valid_height("190in"));
    CHECK(!valid_height("190"));

    CHECK(valid_hair_color("#123abc"));
    CHECK(!valid_hair_color("#123abz"));
    CHECK(!valid_hair_color("123abc"));

    CHECK(valid_eye_color("brn"));
    CHECK(!valid_eye_color("wat"));

    CHECK(valid_passport_id("000000001"));
    CHECK(!valid_passport_id("0123456789"));
}

TEST_CASE("passport validation 2") {
    auto const valid_passports = read_passports("./valid_inputs");
    CHECK(ranges::all_of(valid_passports, [](passport const& p) { return valid_passport(p); }));

    auto const invalid_passports = read_passports("./invalid_inputs");
    CHECK(ranges::none_of(invalid_passports, [](passport const& p) { return valid_passport(p); }));
}

int main(int argc, char** argv) {
    int test_result = Catch::Session().run(argc, argv);

    if (test_result != 0) {
        fmt::print("Test Failed\n");
        return test_result;
    }

    fmt::print("***Puzzle 1\n");
    auto const passports = read_passports("./input");
    fmt::print("Number of passports with all required fields: {}\n",
               ranges::count_if(passports, [](auto p) { return contains_required_fields(p); }));

    fmt::print("\n***Puzzle 2\n");
    fmt::print("Number of valid passports: {}\n",
               ranges::count_if(passports, [](auto p) { return valid_passport(p); }));
}
