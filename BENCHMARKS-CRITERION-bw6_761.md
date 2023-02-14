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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.83 us` (✅ **1.00x**) | `4.83 us` (✅ **1.00x slower**) | `77.53 ns` (🚀 **62.31x faster**) | `163.90 ns` (🚀 **29.47x faster**) | `27.87 ns` (🚀 **173.32x faster**) | `12.66 ns` (🚀 **381.60x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.89 us` (✅ **1.00x**) | `4.89 us` (✅ **1.00x slower**) | `78.11 ns` (🚀 **62.54x faster**) | `154.64 ns` (🚀 **31.59x faster**) | `25.92 ns` (🚀 **188.49x faster**) | `13.30 ns` (🚀 **367.26x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.37 us` (✅ **1.00x**) | `3.37 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.40 us` (✅ **1.00x**) | `3.40 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.23 us` (✅ **1.00x**) | `2.23 us` (✅ **1.00x slower**) | `54.21 ns` (🚀 **41.21x faster**) | `119.70 ns` (🚀 **18.66x faster**) | `19.41 ns` (🚀 **115.12x faster**) | `7.12 ns` (🚀 **313.61x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.75 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `70.32 ns` (❌ *3.90x slower*)    | `120.20 ns` (❌ *6.66x slower*)    | `22.48 ns` (❌ *1.25x slower*)     | `18.04 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.50 us` (❌ *32.86x slower*)    | `7.95 us` (❌ *104.66x slower*)    | `304.80 ns` (❌ *4.01x slower*)    | `75.96 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.80 us` (❌ *26.90x slower*)    | `5.55 us` (❌ *83.12x slower*)     | `243.07 ns` (❌ *3.64x slower*)    | `66.77 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.88 us` (❌ *3.66x slower*)    | `60.98 us` (❌ *4.30x slower*)     | `47.73 us` (❌ *3.36x slower*)     | `14.19 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.14 us` (❌ *43.84x slower*)    | `16.25 us` (❌ *138.51x slower*)   | `420.29 ns` (❌ *3.58x slower*)    | `117.34 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.08 us` (❌ *31.00x slower*)    | `16.17 us` (❌ *98.61x slower*)    | `649.23 ns` (❌ *3.96x slower*)    | `164.03 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)  | `17.18 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.35 ns` (✅ **1.00x**) | `21.86 ns` (❌ *2.11x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)  | `4.55 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `515.93 ns` (✅ **1.00x**) | `516.52 ns` (✅ **1.00x slower**) | `56.42 ns` (🚀 **9.14x faster**)     | `168.10 ns` (🚀 **3.07x faster**)    | `509.03 ns` (✅ **1.01x faster**)  | `1.08 us` (❌ *2.10x slower*)      |
| **`serialize_uncompressed`**             | `699.05 ns` (✅ **1.00x**) | `698.92 ns` (✅ **1.00x faster**) | `57.29 ns` (🚀 **12.20x faster**)    | `169.97 ns` (🚀 **4.11x faster**)    | `509.24 ns` (✅ **1.37x faster**)  | `1.08 us` (❌ *1.55x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `94.50 ns` (🚀 **16837.75x faster**) | `341.83 ns` (🚀 **4654.83x faster**) | `1.04 us` (🚀 **1529.06x faster**) | `2.13 us` (🚀 **747.34x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.84 us` (✅ **1.00x**) | `291.91 us` (✅ **1.00x slower**) | `94.37 ns` (🚀 **3092.64x faster**)  | `341.67 ns` (🚀 **854.16x faster**)  | `1.04 us` (🚀 **280.44x faster**)  | `2.13 us` (🚀 **137.08x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x slower**)   | `94.46 ns` (🚀 **13755.06x faster**) | `341.56 ns` (🚀 **3804.14x faster**) | `1.04 us` (🚀 **1248.37x faster**) | `2.14 us` (🚀 **607.13x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `765.80 ns` (✅ **1.00x**) | `771.15 ns` (✅ **1.01x slower**) | `94.42 ns` (🚀 **8.11x faster**)     | `341.48 ns` (🚀 **2.24x faster**)    | `1.04 us` (❌ *1.36x slower*)      | `2.13 us` (❌ *2.78x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.58 s` (✅ **1.00x**) | `12.61 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.48 us` (✅ **1.00x**) | `290.33 us` (❌ *4.30x slower*)   | `7.00 ms` (❌ *103.80x slower*)    |
| **`legendre_for_qr`**    | `31.71 us` (✅ **1.00x**) | `293.07 us` (❌ *9.24x slower*)   | `297.25 us` (❌ *9.37x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `89.42 ns` (✅ **1.00x**) | `168.01 ns` (❌ *1.88x slower*)    |
| **`from_big-endian_bits`**    | `89.35 ns` (✅ **1.00x**) | `165.96 ns` (❌ *1.86x slower*)    |
| **`comparison`**              | `5.14 ns` (✅ **1.00x**)  | `5.10 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.35 ns` (✅ **1.00x**) | `315.89 ns` (❌ *4.19x slower*)    |
| **`into_bigint`** | `46.96 ns` (✅ **1.00x**) | `155.70 ns` (❌ *3.32x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

