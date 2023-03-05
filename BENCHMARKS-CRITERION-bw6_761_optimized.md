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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.15 us` (✅ **1.00x**)        | `4.14 us` (✅ **1.00x faster**) | `91.52 ns` (🚀 **45.29x faster**) | `181.71 ns` (🚀 **22.81x faster**) | `30.65 ns` (🚀 **135.23x faster**) | `19.06 ns` (🚀 **217.51x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.20 us` (✅ **1.00x**)        | `4.20 us` (✅ **1.00x slower**) | `85.74 ns` (🚀 **49.02x faster**) | `168.40 ns` (🚀 **24.96x faster**) | `28.35 ns` (🚀 **148.23x faster**) | `15.39 ns` (🚀 **273.06x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `2.92 us` (✅ **1.00x**)        | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.96 us` (✅ **1.00x**)        | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.94 us` (✅ **1.00x**)        | `1.94 us` (✅ **1.00x faster**) | `71.05 ns` (🚀 **27.29x faster**) | `143.67 ns` (🚀 **13.50x faster**) | `21.88 ns` (🚀 **88.62x faster**)  | `7.48 ns` (🚀 **259.28x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.55 ms` (✅ **1.00x**)        | `1.55 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `70.14 ns` (❌ *4.10x slower*)    | `123.70 ns` (❌ *7.23x slower*)    | `24.32 ns` (❌ *1.42x slower*)     | `17.12 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.17 us` (❌ *31.27x slower*)    | `6.91 us` (❌ *99.62x slower*)     | `271.11 ns` (❌ *3.91x slower*)    | `69.38 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.56 us` (❌ *26.41x slower*)    | `4.87 us` (❌ *82.40x slower*)     | `218.29 ns` (❌ *3.69x slower*)    | `59.15 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `50.75 us` (❌ *3.89x slower*)    | `58.53 us` (❌ *4.48x slower*)     | `47.26 us` (❌ *3.62x slower*)     | `13.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.46 us` (❌ *42.11x slower*)    | `14.09 us` (❌ *133.06x slower*)   | `400.88 ns` (❌ *3.79x slower*)    | `105.89 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.41 us` (❌ *28.10x slower*)    | `13.97 us` (❌ *88.96x slower*)    | `568.00 ns` (❌ *3.62x slower*)    | `157.07 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)        | `15.73 ns` (❌ *2.01x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**)       | `20.95 ns` (❌ *1.97x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `468.62 ns` (✅ **1.00x**)      | `468.87 ns` (✅ **1.00x slower**) | `50.37 ns` (🚀 **9.30x faster**)     | `157.21 ns` (🚀 **2.98x faster**)    | `464.72 ns` (✅ **1.01x faster**)    | `987.06 ns` (❌ *2.11x slower*)    |
| **`serialize_uncompressed`**             | `629.55 ns` (✅ **1.00x**)      | `629.41 ns` (✅ **1.00x faster**) | `50.19 ns` (🚀 **12.54x faster**)    | `157.16 ns` (🚀 **4.01x faster**)    | `464.90 ns` (✅ **1.35x faster**)    | `986.96 ns` (❌ *1.57x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)        | `1.37 ms` (✅ **1.00x slower**)   | `93.68 ns` (🚀 **14615.14x faster**) | `302.94 ns` (🚀 **4519.58x faster**) | `942.47 ns` (🚀 **1452.72x faster**) | `1.90 us` (🚀 **722.49x faster**)  |
| **`deserialize_compressed_unchecked`**   | `253.21 us` (✅ **1.00x**)      | `253.02 us` (✅ **1.00x faster**) | `93.68 ns` (🚀 **2703.06x faster**)  | `302.92 ns` (🚀 **835.90x faster**)  | `942.24 ns` (🚀 **268.73x faster**)  | `1.89 us` (🚀 **134.21x faster**)  |
| **`deserialize_uncompressed`**           | `1.12 ms` (✅ **1.00x**)        | `1.12 ms` (✅ **1.00x slower**)   | `93.68 ns` (🚀 **11922.63x faster**) | `303.07 ns` (🚀 **3685.47x faster**) | `942.55 ns` (🚀 **1185.03x faster**) | `1.89 us` (🚀 **591.91x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `699.67 ns` (✅ **1.00x**)      | `704.25 ns` (✅ **1.01x slower**) | `94.09 ns` (🚀 **7.44x faster**)     | `303.18 ns` (🚀 **2.31x faster**)    | `942.47 ns` (❌ *1.35x slower*)      | `1.89 us` (❌ *2.70x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `11.02 s` (✅ **1.00x**)        | `11.03 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.59 us` (✅ **1.00x**) | `251.71 us` (❌ *3.90x slower*)   | `6.00 ms` (❌ *92.92x slower*)     |
| **`legendre_for_qr`**    | `29.12 us` (✅ **1.00x**) | `251.48 us` (❌ *8.64x slower*)   | `257.15 us` (❌ *8.83x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)        | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `107.96 ns` (✅ **1.00x**)      | `210.87 ns` (❌ *1.95x slower*)    |
| **`from_big-endian_bits`**    | `107.91 ns` (✅ **1.00x**)      | `210.66 ns` (❌ *1.95x slower*)    |
| **`comparison`**              | `4.21 ns` (✅ **1.00x**)        | `4.20 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.66 ns` (✅ **1.00x**)        | `5.06 ns` (✅ **1.09x slower**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)        | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.38 ns` (✅ **1.00x**) | `278.71 ns` (❌ *3.51x slower*)    |
| **`into_bigint`** | `41.53 ns` (✅ **1.00x**) | `142.20 ns` (❌ *3.42x slower*)    |

### pairing_for_bw6_761optimized

|        | `g1_preparation_for_bw6_761optimized`          | `g2_preparation_for_bw6_761optimized`          | `miller_loop_for_bw6_761optimized`          | `final_exponentiation_for_bw6_761optimized`          | `full_pairing_for_bw6_761optimized`           |
|:-------|:-----------------------------------------------|:-----------------------------------------------|:--------------------------------------------|:-----------------------------------------------------|:--------------------------------------------- |
|        | `34.02 ns` (✅ **1.00x**)                       | `10.68 ns` (🚀 **3.19x faster**)                | `3.89 ms` (❌ *114343.65x slower*)           | `3.69 ms` (❌ *108342.35x slower*)                    | `7.58 ms` (❌ *222920.99x slower*)             |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

