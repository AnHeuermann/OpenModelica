# Test 1
add_executable(test_array_length_scalar
  01_array_length_scalar.c
)
target_link_libraries(test_array_length_scalar PRIVATE SimulationRuntimeC)
add_test(NAME test_array_length_scalar COMMAND test_array_length_scalar)

# Test 2
add_executable(test_array_length_vector
  02_array_length_vector.c
)
target_link_libraries(test_array_length_vector PRIVATE SimulationRuntimeC)
add_test(NAME test_array_length_vector COMMAND test_array_length_vector)

# Test 3
add_executable(test_array_length_matrix
  03_array_length_matrix.c
)
target_link_libraries(test_array_length_matrix PRIVATE SimulationRuntimeC)
add_test(NAME test_array_length_matrix COMMAND test_array_length_matrix)

# Test 4
add_executable(test_collect_array_sizes
  04_collect_array_sizes.c
)
target_link_libraries(test_collect_array_sizes PRIVATE SimulationRuntimeC)
add_test(NAME test_collect_array_sizes COMMAND test_collect_array_sizes)

add_custom_target(ctestsuite-arrayIndex-unit DEPENDS
  test_array_length_scalar
  test_array_length_vector
  test_array_length_matrix
  test_collect_array_sizes
)
