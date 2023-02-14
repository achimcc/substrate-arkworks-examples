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
|        | `1.70 ms` (✅ **1.00x**)          | `1.69 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.16 us` (✅ **1.00x**) | `4.15 us` (✅ **1.00x faster**) | `91.05 ns` (🚀 **45.65x faster**) | `182.26 ns` (🚀 **22.80x faster**) | `30.13 ns` (🚀 **137.92x faster**) | `19.51 ns` (🚀 **213.06x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.20 us` (✅ **1.00x**) | `4.21 us` (✅ **1.00x slower**) | `85.32 ns` (🚀 **49.27x faster**) | `168.39 ns` (🚀 **24.97x faster**) | `28.58 ns` (🚀 **147.08x faster**) | `15.00 ns` (🚀 **280.35x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.96 us` (✅ **1.00x**) | `2.96 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.94 us` (✅ **1.00x**) | `1.94 us` (✅ **1.00x faster**) | `71.17 ns` (🚀 **27.30x faster**) | `144.45 ns` (🚀 **13.45x faster**) | `21.79 ns` (🚀 **89.15x faster**)  | `7.50 ns` (🚀 **259.18x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.50 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `69.52 ns` (❌ *3.96x slower*)    | `123.87 ns` (❌ *7.06x slower*)    | `24.32 ns` (❌ *1.39x slower*)     | `17.55 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.17 us` (❌ *31.43x slower*)    | `6.90 us` (❌ *100.15x slower*)    | `271.54 ns` (❌ *3.94x slower*)    | `68.92 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.57 us` (❌ *26.60x slower*)    | `4.87 us` (❌ *82.78x slower*)     | `216.84 ns` (❌ *3.68x slower*)    | `58.85 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `50.46 us` (❌ *3.68x slower*)    | `58.23 us` (❌ *4.25x slower*)     | `46.93 us` (❌ *3.43x slower*)     | `13.70 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.46 us` (❌ *42.09x slower*)    | `14.07 us` (❌ *132.86x slower*)   | `401.07 ns` (❌ *3.79x slower*)    | `105.93 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.41 us` (❌ *28.15x slower*)    | `13.96 us` (❌ *89.20x slower*)    | `569.93 ns` (❌ *3.64x slower*)    | `156.51 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)  | `15.68 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**) | `21.21 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)  | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `468.91 ns` (✅ **1.00x**) | `468.93 ns` (✅ **1.00x slower**) | `50.31 ns` (🚀 **9.32x faster**)     | `157.20 ns` (🚀 **2.98x faster**)    | `465.48 ns` (✅ **1.01x faster**)    | `986.54 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `629.24 ns` (✅ **1.00x**) | `630.31 ns` (✅ **1.00x slower**) | `50.92 ns` (🚀 **12.36x faster**)    | `157.85 ns` (🚀 **3.99x faster**)    | `465.54 ns` (✅ **1.35x faster**)    | `986.18 ns` (❌ *1.57x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)   | `1.37 ms` (✅ **1.00x faster**)   | `93.74 ns` (🚀 **14592.27x faster**) | `303.96 ns` (🚀 **4499.95x faster**) | `945.50 ns` (🚀 **1446.67x faster**) | `1.90 us` (🚀 **719.95x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.61 us` (✅ **1.00x**) | `252.60 us` (✅ **1.00x faster**) | `93.74 ns` (🚀 **2694.69x faster**)  | `303.97 ns` (🚀 **831.03x faster**)  | `942.13 ns` (🚀 **268.13x faster**)  | `1.90 us` (🚀 **132.97x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x faster**)   | `93.55 ns` (🚀 **11908.26x faster**) | `303.82 ns` (🚀 **3666.71x faster**) | `946.93 ns` (🚀 **1176.46x faster**) | `1.90 us` (🚀 **586.36x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `694.01 ns` (✅ **1.00x**) | `694.96 ns` (✅ **1.00x slower**) | `93.89 ns` (🚀 **7.39x faster**)     | `305.24 ns` (🚀 **2.27x faster**)    | `941.21 ns` (❌ *1.36x slower*)      | `1.90 us` (❌ *2.74x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `11.10 s` (✅ **1.00x**) | `11.15 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `65.46 us` (✅ **1.00x**) | `251.35 us` (❌ *3.84x slower*)   | `6.00 ms` (❌ *91.61x slower*)     |
| **`legendre_for_qr`**    | `29.28 us` (✅ **1.00x**) | `252.91 us` (❌ *8.64x slower*)   | `257.27 us` (❌ *8.79x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `109.22 ns` (✅ **1.00x**) | `210.59 ns` (❌ *1.93x slower*)    |
| **`from_big-endian_bits`**    | `109.21 ns` (✅ **1.00x**) | `210.49 ns` (❌ *1.93x slower*)    |
| **`comparison`**              | `4.23 ns` (✅ **1.00x**)   | `4.20 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `4.66 ns` (✅ **1.00x**)   | `4.63 ns` (✅ **1.01x faster**)    |
| **`is_zero`**                 | `4.01 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `78.46 ns` (✅ **1.00x**) | `278.15 ns` (❌ *3.55x slower*)    |
| **`into_bigint`** | `41.53 ns` (✅ **1.00x**) | `141.65 ns` (❌ *3.41x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

