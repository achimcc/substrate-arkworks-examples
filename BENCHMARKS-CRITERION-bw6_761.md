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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.15 us` (✅ **1.00x**) | `4.15 us` (✅ **1.00x faster**) | `89.91 ns` (🚀 **46.21x faster**) | `180.63 ns` (🚀 **23.00x faster**) | `30.36 ns` (🚀 **136.85x faster**) | `19.48 ns` (🚀 **213.31x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.20 us` (✅ **1.00x**) | `4.21 us` (✅ **1.00x slower**) | `84.96 ns` (🚀 **49.50x faster**) | `167.69 ns` (🚀 **25.08x faster**) | `28.65 ns` (🚀 **146.79x faster**) | `15.00 ns` (🚀 **280.35x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.93 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.96 us` (✅ **1.00x**) | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.94 us` (✅ **1.00x**) | `1.94 us` (✅ **1.00x faster**) | `71.11 ns` (🚀 **27.32x faster**) | `143.25 ns` (🚀 **13.56x faster**) | `21.36 ns` (🚀 **90.93x faster**)  | `7.49 ns` (🚀 **259.51x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.50 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `70.14 ns` (❌ *4.00x slower*)    | `123.10 ns` (❌ *7.01x slower*)    | `24.60 ns` (❌ *1.40x slower*)     | `17.55 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.17 us` (❌ *31.26x slower*)    | `6.91 us` (❌ *99.63x slower*)     | `271.28 ns` (❌ *3.91x slower*)    | `69.37 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.56 us` (❌ *26.50x slower*)    | `4.88 us` (❌ *82.68x slower*)     | `215.86 ns` (❌ *3.66x slower*)    | `59.03 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `50.45 us` (❌ *3.68x slower*)    | `58.26 us` (❌ *4.25x slower*)     | `46.96 us` (❌ *3.42x slower*)     | `13.71 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.46 us` (❌ *42.14x slower*)    | `14.12 us` (❌ *133.36x slower*)   | `401.32 ns` (❌ *3.79x slower*)    | `105.91 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.41 us` (❌ *28.15x slower*)    | `13.99 us` (❌ *89.28x slower*)    | `568.83 ns` (❌ *3.63x slower*)    | `156.66 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.84 ns` (✅ **1.00x**)  | `15.75 ns` (❌ *2.01x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**) | `21.16 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)  | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `467.35 ns` (✅ **1.00x**) | `468.98 ns` (✅ **1.00x slower**) | `50.31 ns` (🚀 **9.29x faster**)     | `156.61 ns` (🚀 **2.98x faster**)    | `465.62 ns` (✅ **1.00x faster**)    | `985.27 ns` (❌ *2.11x slower*)    |
| **`serialize_uncompressed`**             | `630.26 ns` (✅ **1.00x**) | `630.46 ns` (✅ **1.00x slower**) | `50.67 ns` (🚀 **12.44x faster**)    | `157.00 ns` (🚀 **4.01x faster**)    | `465.55 ns` (✅ **1.35x faster**)    | `985.38 ns` (❌ *1.56x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)   | `1.37 ms` (✅ **1.00x slower**)   | `94.61 ns` (🚀 **14478.13x faster**) | `305.80 ns` (🚀 **4479.13x faster**) | `941.46 ns` (🚀 **1454.88x faster**) | `1.90 us` (🚀 **721.55x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.67 us` (✅ **1.00x**) | `252.67 us` (✅ **1.00x slower**) | `94.62 ns` (🚀 **2670.28x faster**)  | `304.38 ns` (🚀 **830.11x faster**)  | `941.46 ns` (🚀 **268.38x faster**)  | `1.90 us` (🚀 **133.10x faster**)  |
| **`deserialize_uncompressed`**           | `1.12 ms` (✅ **1.00x**)   | `1.12 ms` (✅ **1.00x slower**)   | `94.66 ns` (🚀 **11781.60x faster**) | `304.60 ns` (🚀 **3661.32x faster**) | `945.83 ns` (🚀 **1179.11x faster**) | `1.90 us` (🚀 **586.97x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `696.14 ns` (✅ **1.00x**) | `697.57 ns` (✅ **1.00x slower**) | `94.09 ns` (🚀 **7.40x faster**)     | `304.62 ns` (🚀 **2.29x faster**)    | `946.20 ns` (❌ *1.36x slower*)      | `1.91 us` (❌ *2.74x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `11.08 s` (✅ **1.00x**) | `11.03 s` (✅ **1.00x faster**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `65.45 us` (✅ **1.00x**) | `251.42 us` (❌ *3.84x slower*)   | `6.00 ms` (❌ *91.73x slower*)     |
| **`legendre_for_qr`**    | `29.26 us` (✅ **1.00x**) | `252.98 us` (❌ *8.65x slower*)   | `257.17 us` (❌ *8.79x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `109.52 ns` (✅ **1.00x**) | `211.06 ns` (❌ *1.93x slower*)    |
| **`from_big-endian_bits`**    | `109.42 ns` (✅ **1.00x**) | `210.61 ns` (❌ *1.92x slower*)    |
| **`comparison`**              | `4.23 ns` (✅ **1.00x**)   | `4.20 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)   | `5.06 ns` (✅ **1.09x slower**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `78.43 ns` (✅ **1.00x**) | `279.61 ns` (❌ *3.56x slower*)    |
| **`into_bigint`** | `41.54 ns` (✅ **1.00x**) | `141.60 ns` (❌ *3.41x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

