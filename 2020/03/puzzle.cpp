#define CATCH_CONFIG_RUNNER
#include <catch2/catch.hpp>

#include <algorithm>
#include <filesystem>
#include <fmt/format.h>
#include <fstream>
#include <functional>
#include <numeric>
#include <vector>

namespace ranges = std::ranges;
namespace views = std::views;
namespace fs = std::filesystem;

enum class map_tile : bool { open_square, tree };

class toboggan_map {
  public:
    explicit toboggan_map(fs::path file) {
        std::ifstream f(file);
        if (!f) {
            fmt::print("Could not open file {}\n", file.c_str());
            std::exit(-1);
        }

        for (std::string line; std::getline(f, line);) {
            auto v = views::all(line) | views::transform([](char c) {
                         return c == '#' ? map_tile::tree : map_tile::open_square;
                     });
            data_.emplace_back(v.begin(), v.end());
        }
    }

    auto width() const { return data_.front().size(); }
    auto height() const { return data_.size(); }

    auto operator()(std::size_t x, std::size_t y) const {
        assert(y < height());
        return data_[y][x % width()];
    }

  private:
    std::vector<std::vector<map_tile>> data_;
};

struct position {
    std::size_t x{};
    std::size_t y{};
};

struct slope {
    int x{};
    int y{};
};

auto count_trees(toboggan_map const& map, slope step) {
    position pos;

    std::size_t count = 0;
    while (pos.y < map.height()) {
        if (map(pos.x, pos.y) == map_tile::tree) {
            count++;
        }
        pos.x = (pos.x + step.x) % map.width();
        pos.y += step.y;
    }

    return count;
}

TEST_CASE("reading input") {
    auto const map = toboggan_map("./test_input");

    CHECK(map.height() == 11);
    CHECK(map.width() == 11);
}

TEST_CASE("counting trees") {
    auto const map = toboggan_map("./test_input");
    CHECK(count_trees(map, slope{.x = 1, .y = 1}) == 2);
    CHECK(count_trees(map, slope{.x = 3, .y = 1}) == 7);
    CHECK(count_trees(map, slope{.x = 5, .y = 1}) == 3);
    CHECK(count_trees(map, slope{.x = 7, .y = 1}) == 4);
    CHECK(count_trees(map, slope{.x = 1, .y = 2}) == 2);
}

int main(int argc, char** argv) {
    int test_result = Catch::Session().run(argc, argv);

    if (test_result != 0) {
        fmt::print("Test Failed\n");
        return test_result;
    }

    fmt::print("***Puzzle 1\n");
    auto const puzzle_input = toboggan_map("./input");
    auto const tree_count = count_trees(puzzle_input, slope{.x = 3, .y = 1});
    fmt::print("Number of trees with slope (3;1): {}\n", tree_count);

    fmt::print("***Puzzle 2\n");
    std::vector<slope> slopes{
        {.x = 1, .y = 1}, //
        {.x = 3, .y = 1}, //
        {.x = 5, .y = 1}, //
        {.x = 7, .y = 1}, //
        {.x = 1, .y = 2},
    };

    std::vector<std::size_t> tree_counts;
    std::transform(slopes.begin(), slopes.end(), std::back_inserter(tree_counts),
                   [&](slope step) { return count_trees(puzzle_input, step); });

    fmt::print("Tree counts: {}\n", fmt::join(tree_counts, ", "));

    auto const product_of_tree_counts =
        std::accumulate(tree_counts.begin(), tree_counts.end(), std::size_t{1}, std::multiplies{});

    fmt::print("Product of tree counts: {}\n", product_of_tree_counts);
}
