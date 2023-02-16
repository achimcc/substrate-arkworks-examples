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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.16 us` (✅ **1.00x**) | `4.15 us` (✅ **1.00x faster**) | `90.27 ns` (🚀 **46.05x faster**) | `183.47 ns` (🚀 **22.66x faster**) | `30.01 ns` (🚀 **138.52x faster**) | `19.45 ns` (🚀 **213.70x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.20 us` (✅ **1.00x**) | `4.20 us` (✅ **1.00x faster**) | `85.10 ns` (🚀 **49.35x faster**) | `168.64 ns` (🚀 **24.90x faster**) | `28.25 ns` (🚀 **148.68x faster**) | `15.10 ns` (🚀 **278.18x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.96 us` (✅ **1.00x**) | `2.97 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.94 us` (✅ **1.00x**) | `1.94 us` (✅ **1.00x faster**) | `70.93 ns` (🚀 **27.38x faster**) | `144.04 ns` (🚀 **13.48x faster**) | `22.08 ns` (🚀 **87.94x faster**)  | `7.48 ns` (🚀 **259.64x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.50 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `68.59 ns` (❌ *4.09x slower*)    | `123.29 ns` (❌ *7.36x slower*)    | `23.98 ns` (❌ *1.43x slower*)     | `16.76 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.17 us` (❌ *31.22x slower*)    | `6.91 us` (❌ *99.59x slower*)     | `270.38 ns` (❌ *3.90x slower*)    | `69.37 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.56 us` (❌ *26.49x slower*)    | `4.85 us` (❌ *82.19x slower*)     | `215.83 ns` (❌ *3.65x slower*)    | `59.06 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `50.54 us` (❌ *3.69x slower*)    | `58.28 us` (❌ *4.26x slower*)     | `46.96 us` (❌ *3.43x slower*)     | `13.70 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.47 us` (❌ *42.13x slower*)    | `14.08 us` (❌ *132.81x slower*)   | `400.83 ns` (❌ *3.78x slower*)    | `105.98 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.40 us` (❌ *28.09x slower*)    | `13.97 us` (❌ *89.08x slower*)    | `568.71 ns` (❌ *3.63x slower*)    | `156.78 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)  | `15.69 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.63 ns` (✅ **1.00x**) | `21.19 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.10 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)  | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `467.33 ns` (✅ **1.00x**) | `467.53 ns` (✅ **1.00x slower**) | `50.31 ns` (🚀 **9.29x faster**)     | `156.52 ns` (🚀 **2.99x faster**)    | `465.56 ns` (✅ **1.00x faster**)    | `984.58 ns` (❌ *2.11x slower*)    |
| **`serialize_uncompressed`**             | `629.14 ns` (✅ **1.00x**) | `629.15 ns` (✅ **1.00x slower**) | `50.69 ns` (🚀 **12.41x faster**)    | `156.88 ns` (🚀 **4.01x faster**)    | `465.48 ns` (✅ **1.35x faster**)    | `984.70 ns` (❌ *1.57x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)   | `1.37 ms` (✅ **1.00x faster**)   | `93.99 ns` (🚀 **14549.13x faster**) | `303.49 ns` (🚀 **4505.65x faster**) | `946.45 ns` (🚀 **1444.78x faster**) | `1.90 us` (🚀 **719.04x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.68 us` (✅ **1.00x**) | `252.62 us` (✅ **1.00x faster**) | `93.97 ns` (🚀 **2689.03x faster**)  | `303.52 ns` (🚀 **832.52x faster**)  | `942.22 ns` (🚀 **268.18x faster**)  | `1.90 us` (🚀 **132.77x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x faster**)   | `93.93 ns` (🚀 **11859.72x faster**) | `305.34 ns` (🚀 **3648.26x faster**) | `942.24 ns` (🚀 **1182.23x faster**) | `1.90 us` (🚀 **585.92x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `698.66 ns` (✅ **1.00x**) | `700.49 ns` (✅ **1.00x slower**) | `94.73 ns` (🚀 **7.38x faster**)     | `303.86 ns` (🚀 **2.30x faster**)    | `942.06 ns` (❌ *1.35x slower*)      | `1.90 us` (❌ *2.72x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `11.00 s` (✅ **1.00x**) | `11.01 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `65.34 us` (✅ **1.00x**) | `251.36 us` (❌ *3.85x slower*)   | `6.00 ms` (❌ *91.91x slower*)     |
| **`legendre_for_qr`**    | `29.29 us` (✅ **1.00x**) | `252.72 us` (❌ *8.63x slower*)   | `257.41 us` (❌ *8.79x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `109.04 ns` (✅ **1.00x**) | `210.68 ns` (❌ *1.93x slower*)    |
| **`from_big-endian_bits`**    | `109.01 ns` (✅ **1.00x**) | `210.29 ns` (❌ *1.93x slower*)    |
| **`comparison`**              | `4.22 ns` (✅ **1.00x**)   | `4.20 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)   | `5.07 ns` (✅ **1.09x slower**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.69 ns` (✅ **1.00x**) | `280.33 ns` (❌ *3.52x slower*)    |
| **`into_bigint`** | `41.52 ns` (✅ **1.00x**) | `141.57 ns` (❌ *3.41x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

