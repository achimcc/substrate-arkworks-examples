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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.15 us` (✅ **1.00x**) | `4.15 us` (✅ **1.00x slower**) | `90.94 ns` (🚀 **45.69x faster**) | `185.55 ns` (🚀 **22.39x faster**) | `29.78 ns` (🚀 **139.53x faster**) | `19.49 ns` (🚀 **213.21x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.21 us` (✅ **1.00x**) | `4.21 us` (✅ **1.00x slower**) | `85.67 ns` (🚀 **49.09x faster**) | `171.07 ns` (🚀 **24.58x faster**) | `29.40 ns` (🚀 **143.02x faster**) | `15.26 ns` (🚀 **275.52x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.96 us` (✅ **1.00x**) | `2.96 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.94 us` (✅ **1.00x**) | `1.94 us` (✅ **1.00x faster**) | `73.02 ns` (🚀 **26.59x faster**) | `144.42 ns` (🚀 **13.44x faster**) | `21.54 ns` (🚀 **90.14x faster**)  | `7.49 ns` (🚀 **259.33x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.50 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `71.49 ns` (❌ *4.07x slower*)    | `125.29 ns` (❌ *7.14x slower*)    | `24.48 ns` (❌ *1.39x slower*)     | `17.55 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.17 us` (❌ *31.23x slower*)    | `6.91 us` (❌ *99.53x slower*)     | `270.47 ns` (❌ *3.90x slower*)    | `69.41 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.57 us` (❌ *26.49x slower*)    | `4.87 us` (❌ *82.29x slower*)     | `215.89 ns` (❌ *3.65x slower*)    | `59.17 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `50.48 us` (❌ *3.69x slower*)    | `58.28 us` (❌ *4.26x slower*)     | `46.95 us` (❌ *3.43x slower*)     | `13.68 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.46 us` (❌ *42.12x slower*)    | `14.07 us` (❌ *132.88x slower*)   | `401.09 ns` (❌ *3.79x slower*)    | `105.92 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.41 us` (❌ *28.09x slower*)    | `13.97 us` (❌ *89.07x slower*)    | `569.01 ns` (❌ *3.63x slower*)    | `156.87 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.84 ns` (✅ **1.00x**)  | `15.62 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**) | `21.21 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)  | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `467.35 ns` (✅ **1.00x**) | `467.76 ns` (✅ **1.00x slower**) | `50.33 ns` (🚀 **9.29x faster**)     | `156.73 ns` (🚀 **2.98x faster**)    | `466.45 ns` (✅ **1.00x faster**)    | `985.44 ns` (❌ *2.11x slower*)    |
| **`serialize_uncompressed`**             | `629.48 ns` (✅ **1.00x**) | `629.01 ns` (✅ **1.00x faster**) | `50.71 ns` (🚀 **12.41x faster**)    | `157.04 ns` (🚀 **4.01x faster**)    | `466.50 ns` (✅ **1.35x faster**)    | `985.75 ns` (❌ *1.57x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)   | `1.37 ms` (✅ **1.00x faster**)   | `93.46 ns` (🚀 **14637.05x faster**) | `303.80 ns` (🚀 **4503.13x faster**) | `945.92 ns` (🚀 **1446.26x faster**) | `1.90 us` (🚀 **720.18x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.74 us` (✅ **1.00x**) | `252.70 us` (✅ **1.00x faster**) | `93.47 ns` (🚀 **2703.86x faster**)  | `303.81 ns` (🚀 **831.88x faster**)  | `941.52 ns` (🚀 **268.44x faster**)  | `1.90 us` (🚀 **133.01x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x faster**)   | `93.71 ns` (🚀 **11889.95x faster**) | `303.81 ns` (🚀 **3667.33x faster**) | `943.59 ns` (🚀 **1180.76x faster**) | `1.90 us` (🚀 **586.21x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `694.18 ns` (✅ **1.00x**) | `695.98 ns` (✅ **1.00x slower**) | `93.70 ns` (🚀 **7.41x faster**)     | `305.29 ns` (🚀 **2.27x faster**)    | `943.59 ns` (❌ *1.36x slower*)      | `1.90 us` (❌ *2.74x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `11.15 s` (✅ **1.00x**) | `11.18 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `65.32 us` (✅ **1.00x**) | `251.44 us` (❌ *3.85x slower*)   | `6.00 ms` (❌ *91.84x slower*)     |
| **`legendre_for_qr`**    | `29.57 us` (✅ **1.00x**) | `252.75 us` (❌ *8.55x slower*)   | `257.23 us` (❌ *8.70x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `109.14 ns` (✅ **1.00x**) | `210.56 ns` (❌ *1.93x slower*)    |
| **`from_big-endian_bits`**    | `109.18 ns` (✅ **1.00x**) | `210.49 ns` (❌ *1.93x slower*)    |
| **`comparison`**              | `4.23 ns` (✅ **1.00x**)   | `4.20 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)   | `5.04 ns` (✅ **1.08x slower**)    |
| **`is_zero`**                 | `4.01 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.41 ns` (✅ **1.00x**) | `278.66 ns` (❌ *3.51x slower*)    |
| **`into_bigint`** | `41.56 ns` (✅ **1.00x**) | `141.43 ns` (❌ *3.40x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

