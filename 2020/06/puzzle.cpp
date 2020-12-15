#define CATCH_CONFIG_RUNNER
#include <catch2/catch.hpp>

#include <filesystem>
#include <fmt/format.h>
#include <fstream>
#include <ranges>
#include <set>
#include <vector>

namespace ranges = std::ranges;
namespace fs = std::filesystem;

// set_intersection, set_union

auto count_questions(std::vector<std::string> const& group_answers) {
    std::set<char> unique_questions;
    for (auto const& person_answers : group_answers) {
        unique_questions.insert(person_answers.begin(), person_answers.end());
    }
    return unique_questions.size();
}

auto read_answers(fs::path file) {
    std::ifstream f(file);

    std::vector<std::vector<std::string>> answers;

    while (!f.eof()) {
        std::vector<std::string> group_answers;
        for (std::string line; std::getline(f, line) && !line.empty();) {
            group_answers.push_back(line);
        }
        answers.emplace_back(std::move(group_answers));
    }

    return answers;
}

static std::vector<std::vector<std::string>> const test_answers{
    {"abc"}, {"a", "b", "c"}, {"ab", "ac"}, {"a", "a", "a", "a"}, {"b"}};

TEST_CASE("count unique questions of group") {
    std::vector<std::string> group_answers{"abcx", "abcy", "abcz"};
    CHECK(count_questions(group_answers) == 6);

    std::vector<std::size_t> counts;
    std::transform(test_answers.begin(), test_answers.end(), std::back_inserter(counts),
                   [](auto group) { return count_questions(group); });
    CHECK(counts == std::vector<std::size_t>{3, 3, 3, 1, 1});
    CHECK(std::accumulate(counts.begin(), counts.end(), 0) == 11);
}

TEST_CASE("input reading") {
    auto answers = read_answers("./test_input");
    CHECK(answers == test_answers);
}

int main(int argc, char** argv) {
    int test_result = Catch::Session().run(argc, argv);

    if (test_result != 0) {
        fmt::print("Test Failed\n");
        return test_result;
    }

    fmt::print("***Puzzle 1\n");
    auto const answers = read_answers("./input");
    auto sum_of_question_counts =
        std::accumulate(answers.begin(), answers.end(), 0, [](auto sum, auto group_answers) {
            return sum + count_questions(group_answers);
        });
    fmt::print("Sum of question counts: {}\n", sum_of_question_counts);
}
