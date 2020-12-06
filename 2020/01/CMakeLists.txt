get_filename_component(day ${CMAKE_CURRENT_LIST_DIR} NAME)

add_executable(puzzle_${day} puzzle.cpp)
target_link_libraries(puzzle_${day} PUBLIC fmt::fmt)