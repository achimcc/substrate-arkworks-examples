# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bw6_761](#sample_bw6_761)
    - [arithmetic_for_bw6_761](#arithmetic_for_bw6_761)
    - [serialization_for_bw6_761](#serialization_for_bw6_761)
    - [msm_for_bw6_761](#msm_for_bw6_761)
    - [squareroot_for_bw6_761](#squareroot_for_bw6_761)
    - [bitwise_operations_for_bw6_761](#bitwise_operations_for_bw6_761)
    - [conversions_for_bw6_761](#conversions_for_bw6_761)

## Benchmark Results

### sample_bw6_761

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `1.98 ms` (✅ **1.00x**)          | `1.96 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.83 us` (✅ **1.00x**) | `4.83 us` (✅ **1.00x slower**) | `79.94 ns` (🚀 **60.46x faster**) | `160.12 ns` (🚀 **30.18x faster**) | `27.69 ns` (🚀 **174.51x faster**) | `12.66 ns` (🚀 **381.71x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.88 us` (✅ **1.00x**) | `4.89 us` (✅ **1.00x slower**) | `79.47 ns` (🚀 **61.47x faster**) | `152.74 ns` (🚀 **31.98x faster**) | `25.91 ns` (🚀 **188.55x faster**) | `13.27 ns` (🚀 **368.02x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.37 us` (✅ **1.00x**) | `3.37 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.39 us` (✅ **1.00x**) | `3.39 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.24 us` (✅ **1.00x**) | `2.24 us` (✅ **1.00x slower**) | `56.19 ns` (🚀 **39.79x faster**) | `117.09 ns` (🚀 **19.09x faster**) | `19.13 ns` (🚀 **116.90x faster**) | `7.14 ns` (🚀 **313.34x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.75 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `70.90 ns` (❌ *3.93x slower*)    | `120.16 ns` (❌ *6.65x slower*)    | `22.22 ns` (❌ *1.23x slower*)     | `18.06 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.50 us` (❌ *32.81x slower*)    | `7.96 us` (❌ *104.59x slower*)    | `304.79 ns` (❌ *4.01x slower*)    | `76.08 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.80 us` (❌ *26.94x slower*)    | `5.54 us` (❌ *82.95x slower*)     | `243.22 ns` (❌ *3.64x slower*)    | `66.75 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.78 us` (❌ *3.63x slower*)    | `60.93 us` (❌ *4.28x slower*)     | `47.54 us` (❌ *3.34x slower*)     | `14.25 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.15 us` (❌ *43.92x slower*)    | `16.22 us` (❌ *138.23x slower*)   | `420.47 ns` (❌ *3.58x slower*)    | `117.37 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.09 us` (❌ *31.05x slower*)    | `16.13 us` (❌ *98.47x slower*)    | `649.38 ns` (❌ *3.96x slower*)    | `163.82 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)  | `17.20 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.34 ns` (✅ **1.00x**) | `21.86 ns` (❌ *2.11x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**)  | `4.53 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `515.80 ns` (✅ **1.00x**) | `516.27 ns` (✅ **1.00x slower**) | `56.40 ns` (🚀 **9.14x faster**)     | `168.88 ns` (🚀 **3.05x faster**)    | `510.74 ns` (✅ **1.01x faster**)  | `1.08 us` (❌ *2.10x slower*)      |
| **`serialize_uncompressed`**             | `699.66 ns` (✅ **1.00x**) | `699.72 ns` (✅ **1.00x slower**) | `57.52 ns` (🚀 **12.16x faster**)    | `169.97 ns` (🚀 **4.12x faster**)    | `510.76 ns` (✅ **1.37x faster**)  | `1.08 us` (❌ *1.55x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `94.95 ns` (🚀 **16774.08x faster**) | `341.62 ns` (🚀 **4662.46x faster**) | `1.04 us` (🚀 **1527.89x faster**) | `2.12 us` (🚀 **750.15x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.94 us` (✅ **1.00x**) | `291.95 us` (✅ **1.00x slower**) | `94.92 ns` (🚀 **3075.61x faster**)  | `341.60 ns` (🚀 **854.63x faster**)  | `1.04 us` (🚀 **280.08x faster**)  | `2.12 us` (🚀 **137.50x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x faster**)   | `94.93 ns` (🚀 **13702.29x faster**) | `341.67 ns` (🚀 **3807.10x faster**) | `1.04 us` (🚀 **1247.84x faster**) | `2.12 us` (🚀 **612.71x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `765.32 ns` (✅ **1.00x**) | `771.52 ns` (✅ **1.01x slower**) | `94.86 ns` (🚀 **8.07x faster**)     | `341.62 ns` (🚀 **2.24x faster**)    | `1.04 us` (❌ *1.36x slower*)      | `2.12 us` (❌ *2.77x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.58 s` (✅ **1.00x**) | `12.62 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.28 us` (✅ **1.00x**) | `290.33 us` (❌ *4.32x slower*)   | `7.00 ms` (❌ *104.10x slower*)    |
| **`legendre_for_qr`**    | `31.73 us` (✅ **1.00x**) | `293.16 us` (❌ *9.24x slower*)   | `297.32 us` (❌ *9.37x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `89.04 ns` (✅ **1.00x**) | `166.56 ns` (❌ *1.87x slower*)    |
| **`from_big-endian_bits`**    | `89.03 ns` (✅ **1.00x**) | `169.54 ns` (❌ *1.90x slower*)    |
| **`comparison`**              | `5.14 ns` (✅ **1.00x**)  | `5.10 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.76 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.36 ns` (✅ **1.00x**) | `315.06 ns` (❌ *4.18x slower*)    |
| **`into_bigint`** | `46.97 ns` (✅ **1.00x**) | `155.83 ns` (❌ *3.32x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

