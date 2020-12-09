get_filename_component(day ${CMAKE_CURRENT_LIST_DIR} NAME)

find_package(Catch2 CONFIG REQUIRED)

add_executable(puzzle_${day} puzzle.cpp)
target_link_libraries(puzzle_${day} PUBLIC fmt::fmt Catch2::Catch2)
