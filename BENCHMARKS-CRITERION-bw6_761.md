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
|        | `1.69 ms` (✅ **1.00x**)          | `1.68 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.16 us` (✅ **1.00x**) | `4.16 us` (✅ **1.00x slower**) | `90.63 ns` (🚀 **45.87x faster**) | `181.44 ns` (🚀 **22.91x faster**) | `30.24 ns` (🚀 **137.46x faster**) | `19.01 ns` (🚀 **218.71x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.20 us` (✅ **1.00x**) | `4.20 us` (✅ **1.00x slower**) | `85.14 ns` (🚀 **49.31x faster**) | `168.43 ns` (🚀 **24.93x faster**) | `28.86 ns` (🚀 **145.46x faster**) | `15.40 ns` (🚀 **272.65x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.95 us` (✅ **1.00x**) | `2.95 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.93 us` (✅ **1.00x**) | `1.93 us` (✅ **1.00x faster**) | `67.58 ns` (🚀 **28.62x faster**) | `138.66 ns` (🚀 **13.95x faster**) | `21.08 ns` (🚀 **91.74x faster**)  | `7.48 ns` (🚀 **258.58x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.49 ms` (✅ **1.00x**) | `1.49 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `70.08 ns` (❌ *4.04x slower*)    | `123.31 ns` (❌ *7.11x slower*)    | `24.33 ns` (❌ *1.40x slower*)     | `17.33 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.16 us` (❌ *31.19x slower*)    | `6.89 us` (❌ *99.32x slower*)     | `270.31 ns` (❌ *3.90x slower*)    | `69.37 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.56 us` (❌ *26.32x slower*)    | `4.86 us` (❌ *82.13x slower*)     | `215.96 ns` (❌ *3.65x slower*)    | `59.16 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `50.98 us` (❌ *3.75x slower*)    | `58.78 us` (❌ *4.32x slower*)     | `47.49 us` (❌ *3.49x slower*)     | `13.61 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.46 us` (❌ *42.10x slower*)    | `14.07 us` (❌ *132.83x slower*)   | `401.23 ns` (❌ *3.79x slower*)    | `105.89 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.41 us` (❌ *28.06x slower*)    | `13.96 us` (❌ *88.92x slower*)    | `567.30 ns` (❌ *3.61x slower*)    | `157.02 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)  | `15.62 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.74 ns` (✅ **1.00x**) | `21.00 ns` (❌ *1.96x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)  | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `469.18 ns` (✅ **1.00x**) | `468.76 ns` (✅ **1.00x faster**) | `50.32 ns` (🚀 **9.32x faster**)     | `156.68 ns` (🚀 **2.99x faster**)    | `464.37 ns` (✅ **1.01x faster**)    | `989.91 ns` (❌ *2.11x slower*)    |
| **`serialize_uncompressed`**             | `629.59 ns` (✅ **1.00x**) | `630.31 ns` (✅ **1.00x slower**) | `50.09 ns` (🚀 **12.57x faster**)    | `157.01 ns` (🚀 **4.01x faster**)    | `464.33 ns` (✅ **1.36x faster**)    | `989.93 ns` (❌ *1.57x slower*)    |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)   | `1.36 ms` (✅ **1.00x faster**)   | `95.32 ns` (🚀 **14272.39x faster**) | `304.72 ns` (🚀 **4464.67x faster**) | `943.48 ns` (🚀 **1441.97x faster**) | `1.91 us` (🚀 **711.07x faster**)  |
| **`deserialize_compressed_unchecked`**   | `251.87 us` (✅ **1.00x**) | `251.87 us` (✅ **1.00x slower**) | `93.75 ns` (🚀 **2686.56x faster**)  | `305.90 ns` (🚀 **823.36x faster**)  | `947.68 ns` (🚀 **265.77x faster**)  | `1.90 us` (🚀 **132.26x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x slower**)   | `94.99 ns` (🚀 **11675.16x faster**) | `305.99 ns` (🚀 **3624.50x faster**) | `947.81 ns` (🚀 **1170.13x faster**) | `1.90 us` (🚀 **582.41x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `700.21 ns` (✅ **1.00x**) | `701.32 ns` (✅ **1.00x slower**) | `93.68 ns` (🚀 **7.47x faster**)     | `306.00 ns` (🚀 **2.29x faster**)    | `947.87 ns` (❌ *1.35x slower*)      | `1.91 us` (❌ *2.73x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `11.03 s` (✅ **1.00x**) | `10.99 s` (✅ **1.00x faster**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.86 us` (✅ **1.00x**) | `250.60 us` (❌ *3.86x slower*)   | `5.98 ms` (❌ *92.26x slower*)     |
| **`legendre_for_qr`**    | `29.46 us` (✅ **1.00x**) | `250.95 us` (❌ *8.52x slower*)   | `256.87 us` (❌ *8.72x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `107.37 ns` (✅ **1.00x**) | `211.31 ns` (❌ *1.97x slower*)    |
| **`from_big-endian_bits`**    | `107.53 ns` (✅ **1.00x**) | `210.59 ns` (❌ *1.96x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)   | `4.19 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.66 ns` (✅ **1.00x**)   | `5.33 ns` (❌ *1.15x slower*)      |
| **`is_zero`**                 | `4.01 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.29 ns` (✅ **1.00x**) | `278.63 ns` (❌ *3.51x slower*)    |
| **`into_bigint`** | `41.48 ns` (✅ **1.00x**) | `143.89 ns` (❌ *3.47x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

