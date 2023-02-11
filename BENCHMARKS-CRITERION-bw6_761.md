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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.15 us` (✅ **1.00x**) | `4.15 us` (✅ **1.00x faster**) | `90.45 ns` (🚀 **45.88x faster**) | `181.15 ns` (🚀 **22.91x faster**) | `30.26 ns` (🚀 **137.12x faster**) | `19.41 ns` (🚀 **213.84x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.21 us` (✅ **1.00x**) | `4.20 us` (✅ **1.00x faster**) | `85.00 ns` (🚀 **49.48x faster**) | `169.15 ns` (🚀 **24.87x faster**) | `28.79 ns` (🚀 **146.09x faster**) | `15.24 ns` (🚀 **275.92x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.96 us` (✅ **1.00x**) | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.93 us` (✅ **1.00x**) | `1.93 us` (✅ **1.00x faster**) | `70.44 ns` (🚀 **27.46x faster**) | `143.30 ns` (🚀 **13.50x faster**) | `22.31 ns` (🚀 **86.69x faster**)  | `7.48 ns` (🚀 **258.48x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.50 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `69.82 ns` (❌ *4.16x slower*)    | `124.66 ns` (❌ *7.43x slower*)    | `24.00 ns` (❌ *1.43x slower*)     | `16.77 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.16 us` (❌ *31.20x slower*)    | `6.89 us` (❌ *99.34x slower*)     | `270.54 ns` (❌ *3.90x slower*)    | `69.38 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.56 us` (❌ *26.32x slower*)    | `4.87 us` (❌ *82.36x slower*)     | `216.05 ns` (❌ *3.65x slower*)    | `59.16 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `50.46 us` (❌ *3.72x slower*)    | `58.21 us` (❌ *4.29x slower*)     | `46.92 us` (❌ *3.46x slower*)     | `13.57 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.46 us` (❌ *42.11x slower*)    | `14.08 us` (❌ *132.96x slower*)   | `400.89 ns` (❌ *3.79x slower*)    | `105.92 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.40 us` (❌ *27.79x slower*)    | `13.95 us` (❌ *88.14x slower*)    | `567.11 ns` (❌ *3.58x slower*)    | `158.31 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)  | `15.67 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.63 ns` (✅ **1.00x**) | `20.91 ns` (❌ *1.97x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)  | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `470.89 ns` (✅ **1.00x**) | `469.55 ns` (✅ **1.00x faster**) | `50.43 ns` (🚀 **9.34x faster**)     | `156.41 ns` (🚀 **3.01x faster**)    | `463.39 ns` (✅ **1.02x faster**)    | `990.26 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `630.03 ns` (✅ **1.00x**) | `630.34 ns` (✅ **1.00x slower**) | `50.19 ns` (🚀 **12.55x faster**)    | `156.00 ns` (🚀 **4.04x faster**)    | `465.04 ns` (✅ **1.35x faster**)    | `990.30 ns` (❌ *1.57x slower*)    |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)   | `1.36 ms` (✅ **1.00x faster**)   | `93.30 ns` (🚀 **14628.84x faster**) | `306.52 ns` (🚀 **4452.81x faster**) | `945.98 ns` (🚀 **1442.81x faster**) | `1.91 us` (🚀 **714.08x faster**)  |
| **`deserialize_compressed_unchecked`**   | `251.59 us` (✅ **1.00x**) | `251.63 us` (✅ **1.00x slower**) | `93.31 ns` (🚀 **2696.16x faster**)  | `306.49 ns` (🚀 **820.88x faster**)  | `941.50 ns` (🚀 **267.22x faster**)  | `1.90 us` (🚀 **132.22x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x faster**)   | `93.31 ns` (🚀 **11935.42x faster**) | `306.77 ns` (🚀 **3630.25x faster**) | `943.38 ns` (🚀 **1180.49x faster**) | `1.90 us` (🚀 **585.48x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `694.77 ns` (✅ **1.00x**) | `696.49 ns` (✅ **1.00x slower**) | `93.96 ns` (🚀 **7.39x faster**)     | `305.25 ns` (🚀 **2.28x faster**)    | `943.38 ns` (❌ *1.36x slower*)      | `1.91 us` (❌ *2.75x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `10.99 s` (✅ **1.00x**) | `11.01 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.78 us` (✅ **1.00x**) | `250.33 us` (❌ *3.86x slower*)   | `5.98 ms` (❌ *92.39x slower*)     |
| **`legendre_for_qr`**    | `29.46 us` (✅ **1.00x**) | `251.40 us` (❌ *8.53x slower*)   | `257.45 us` (❌ *8.74x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `108.11 ns` (✅ **1.00x**) | `216.39 ns` (❌ *2.00x slower*)    |
| **`from_big-endian_bits`**    | `108.21 ns` (✅ **1.00x**) | `216.02 ns` (❌ *2.00x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)   | `4.19 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.71 ns` (✅ **1.00x**)   | `5.51 ns` (❌ *1.17x slower*)      |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `78.48 ns` (✅ **1.00x**) | `280.01 ns` (❌ *3.57x slower*)    |
| **`into_bigint`** | `41.56 ns` (✅ **1.00x**) | `143.86 ns` (❌ *3.46x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

