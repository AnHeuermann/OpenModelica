#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "arrayIndex.h"
#include "simulation_data.h"

/**
 * @brief Test calculation of array length of vector variable.
 *
 * @return int  Return 0 on test success, 1 otherwise.
 */
int main(void)
{
  int test_success = 1;
  size_t dim1_1 = 2;
  size_t dim1_2 = 3;
  size_t dim2_1 = 4;
  size_t dim2_2 = 3;
  size_t dim2_3 = 2;
  size_t expected_length = (dim1_1 * dim1_2) + 1 + (dim2_1 * dim2_2 * dim2_3);
  size_t expected_num_arrays = 2;

  // Prepare dummy data
  DIMENSION_ATTRIBUTE dimensions_var1[] = {
      {.type = DIMENSION_BY_START,
       .start = dim1_1,
       .valueReference = -1},
      {.type = DIMENSION_BY_START,
       .start = dim1_2,
       .valueReference = -1}};
  DIMENSION_INFO dimension_info_var1 = {
      .numberOfDimensions = 2,
      .dimensions = dimensions_var1};

  DIMENSION_ATTRIBUTE dimensions_var2[] = {
      {.type = DIMENSION_BY_START,
       .start = dim2_1,
       .valueReference = -1},
      {.type = DIMENSION_BY_START,
       .start = dim2_2,
       .valueReference = -1},
      {.type = DIMENSION_BY_START,
       .start = dim2_3,
       .valueReference = -1}};
  DIMENSION_INFO dimension_info_var2 = {
      .numberOfDimensions = 3,
      .dimensions = dimensions_var2};

  STATIC_REAL_DATA realVarsData[] = {
      {.dimension = dimension_info_var1},
      {.dimension = {0}},
      {.dimension = dimension_info_var2}
  };

  // Test
  size_t actual_num_arrays = collectArrayVariableSizes(&realVarsData, T_REAL, 3);

  // Validate
  if (actual_num_arrays != expected_num_arrays)
  {
    fprintf(stderr, "Test failed: Expected '%zu' arrays, but got '%zu'.\n", expected_num_arrays, actual_num_arrays);
    test_success = 0;
  }

  if (test_success)
  {
    printf("All tests passed!\n");
    return 0;
  }
  else
  {
    printf("Some tests failed!\n");
    return 1;
  }
}
