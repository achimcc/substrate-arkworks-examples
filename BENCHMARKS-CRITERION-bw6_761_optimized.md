# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bw6_761_optimized](#sample_bw6_761_optimized)
    - [arithmetic_for_bw6_761_optimized](#arithmetic_for_bw6_761_optimized)
    - [serialization_for_bw6_761_optimized](#serialization_for_bw6_761_optimized)
    - [msm_for_bw6_761_optimized](#msm_for_bw6_761_optimized)
    - [squareroot_for_bw6_761_optimized](#squareroot_for_bw6_761_optimized)
    - [bitwise_operations_for_bw6_761_optimized](#bitwise_operations_for_bw6_761_optimized)
    - [conversions_for_bw6_761_optimized](#conversions_for_bw6_761_optimized)
    - [pairing_for_bw6_761optimized](#pairing_for_bw6_761optimized)

## Benchmark Results

### sample_bw6_761_optimized

|        | `g1projectivebw6_761_elements`          | `g2projectivebw6_761_elements`           |
|:-------|:----------------------------------------|:---------------------------------------- |
|        | `1.75 ms` (✅ **1.00x**)                 | `1.74 ms` (✅ **1.01x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.14 us` (✅ **1.00x**)        | `4.14 us` (✅ **1.00x slower**) | `89.86 ns` (🚀 **46.10x faster**) | `181.07 ns` (🚀 **22.88x faster**) | `30.30 ns` (🚀 **136.73x faster**) | `19.03 ns` (🚀 **217.63x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.20 us` (✅ **1.00x**)        | `4.21 us` (✅ **1.00x slower**) | `84.76 ns` (🚀 **49.60x faster**) | `168.41 ns` (🚀 **24.96x faster**) | `27.98 ns` (🚀 **150.24x faster**) | `15.38 ns` (🚀 **273.40x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `2.92 us` (✅ **1.00x**)        | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.96 us` (✅ **1.00x**)        | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.94 us` (✅ **1.00x**)        | `1.94 us` (✅ **1.00x faster**) | `70.37 ns` (🚀 **27.55x faster**) | `144.72 ns` (🚀 **13.40x faster**) | `22.30 ns` (🚀 **86.94x faster**)  | `7.48 ns` (🚀 **259.13x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.55 ms` (✅ **1.00x**)        | `1.55 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `69.53 ns` (❌ *4.06x slower*)    | `123.48 ns` (❌ *7.21x slower*)    | `24.32 ns` (❌ *1.42x slower*)     | `17.12 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.17 us` (❌ *31.23x slower*)    | `6.92 us` (❌ *99.75x slower*)     | `271.64 ns` (❌ *3.91x slower*)    | `69.39 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.56 us` (❌ *26.40x slower*)    | `4.87 us` (❌ *82.42x slower*)     | `218.29 ns` (❌ *3.69x slower*)    | `59.15 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `50.77 us` (❌ *3.90x slower*)    | `58.55 us` (❌ *4.49x slower*)     | `47.22 us` (❌ *3.62x slower*)     | `13.03 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.46 us` (❌ *42.09x slower*)    | `14.08 us` (❌ *132.99x slower*)   | `401.45 ns` (❌ *3.79x slower*)    | `105.87 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.41 us` (❌ *28.06x slower*)    | `13.97 us` (❌ *88.98x slower*)    | `568.89 ns` (❌ *3.62x slower*)    | `157.05 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)        | `15.67 ns` (❌ *2.00x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**)       | `21.24 ns` (❌ *2.00x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `468.56 ns` (✅ **1.00x**)      | `468.96 ns` (✅ **1.00x slower**) | `50.37 ns` (🚀 **9.30x faster**)     | `157.23 ns` (🚀 **2.98x faster**)    | `466.50 ns` (✅ **1.00x faster**)    | `984.00 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `629.78 ns` (✅ **1.00x**)      | `629.26 ns` (✅ **1.00x faster**) | `50.19 ns` (🚀 **12.55x faster**)    | `157.31 ns` (🚀 **4.00x faster**)    | `466.47 ns` (✅ **1.35x faster**)    | `984.20 ns` (❌ *1.56x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)        | `1.37 ms` (✅ **1.00x slower**)   | `93.93 ns` (🚀 **14570.72x faster**) | `304.45 ns` (🚀 **4495.56x faster**) | `941.76 ns` (🚀 **1453.30x faster**) | `1.89 us` (🚀 **723.59x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.69 us` (✅ **1.00x**)      | `252.76 us` (✅ **1.00x slower**) | `93.92 ns` (🚀 **2690.41x faster**)  | `304.25 ns` (🚀 **830.53x faster**)  | `942.02 ns` (🚀 **268.24x faster**)  | `1.89 us` (🚀 **133.59x faster**)  |
| **`deserialize_uncompressed`**           | `1.12 ms` (✅ **1.00x**)        | `1.12 ms` (✅ **1.00x slower**)   | `93.91 ns` (🚀 **11891.95x faster**) | `304.35 ns` (🚀 **3669.33x faster**) | `941.79 ns` (🚀 **1185.80x faster**) | `1.89 us` (🚀 **590.94x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `692.72 ns` (✅ **1.00x**)      | `696.18 ns` (✅ **1.01x slower**) | `93.88 ns` (🚀 **7.38x faster**)     | `304.39 ns` (🚀 **2.28x faster**)    | `941.98 ns` (❌ *1.36x slower*)      | `1.89 us` (❌ *2.73x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `11.06 s` (✅ **1.00x**)        | `11.08 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.75 us` (✅ **1.00x**) | `251.52 us` (❌ *3.88x slower*)   | `6.00 ms` (❌ *92.69x slower*)     |
| **`legendre_for_qr`**    | `29.07 us` (✅ **1.00x**) | `251.57 us` (❌ *8.65x slower*)   | `257.19 us` (❌ *8.85x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)        | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `107.79 ns` (✅ **1.00x**)      | `212.43 ns` (❌ *1.97x slower*)    |
| **`from_big-endian_bits`**    | `107.95 ns` (✅ **1.00x**)      | `212.01 ns` (❌ *1.96x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)        | `4.74 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `4.01 ns` (✅ **1.00x**)        | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.34 ns` (✅ **1.00x**) | `278.83 ns` (❌ *3.51x slower*)    |
| **`into_bigint`** | `41.53 ns` (✅ **1.00x**) | `142.20 ns` (❌ *3.42x slower*)    |

### pairing_for_bw6_761optimized

|        | `g1_preparation_for_bw6_761optimized`          | `g2_preparation_for_bw6_761optimized`          | `miller_loop_for_bw6_761optimized`          | `final_exponentiation_for_bw6_761optimized`          | `full_pairing_for_bw6_761optimized`           |
|:-------|:-----------------------------------------------|:-----------------------------------------------|:--------------------------------------------|:-----------------------------------------------------|:--------------------------------------------- |
|        | `16.10 ns` (✅ **1.00x**)                       | `10.66 ns` (✅ **1.51x faster**)                | `3.89 ms` (❌ *241472.48x slower*)           | `3.69 ms` (❌ *228850.75x slower*)                    | `7.59 ms` (❌ *471120.67x slower*)             |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

