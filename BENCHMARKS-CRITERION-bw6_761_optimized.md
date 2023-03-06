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
|        | `2.06 ms` (✅ **1.00x**)                 | `2.00 ms` (✅ **1.03x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                    | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.89 us` (✅ **1.00x**)        | `4.79 us` (✅ **1.02x faster**) | `108.39 ns` (🚀 **45.07x faster**) | `214.12 ns` (🚀 **22.82x faster**) | `34.61 ns` (🚀 **141.14x faster**) | `23.28 ns` (🚀 **209.87x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.98 us` (✅ **1.00x**)        | `4.83 us` (✅ **1.03x faster**) | `102.71 ns` (🚀 **48.46x faster**) | `191.38 ns` (🚀 **26.00x faster**) | `34.66 ns` (🚀 **143.58x faster**) | `18.13 ns` (🚀 **274.57x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.36 us` (✅ **1.00x**)        | `3.39 us` (✅ **1.01x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.47 us` (✅ **1.00x**)        | `3.37 us` (✅ **1.03x faster**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.28 us` (✅ **1.00x**)        | `2.19 us` (✅ **1.04x faster**) | `82.01 ns` (🚀 **27.78x faster**)  | `163.45 ns` (🚀 **13.94x faster**) | `25.70 ns` (🚀 **88.64x faster**)  | `12.69 ns` (🚀 **179.47x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.81 ms` (✅ **1.00x**)        | `1.79 ms` (✅ **1.01x faster**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `81.67 ns` (❌ *4.07x slower*)     | `142.91 ns` (❌ *7.13x slower*)    | `28.37 ns` (❌ *1.41x slower*)     | `20.05 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.52 us` (❌ *31.02x slower*)     | `8.09 us` (❌ *99.69x slower*)     | `322.08 ns` (❌ *3.97x slower*)    | `81.17 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.77 us` (❌ *25.50x slower*)     | `5.72 us` (❌ *82.58x slower*)     | `257.91 ns` (❌ *3.72x slower*)    | `69.32 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `59.59 us` (❌ *3.85x slower*)     | `69.17 us` (❌ *4.47x slower*)     | `56.36 us` (❌ *3.64x slower*)     | `15.47 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.30 us` (❌ *42.29x slower*)     | `16.80 us` (❌ *134.01x slower*)   | `479.77 ns` (❌ *3.83x slower*)    | `125.39 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.10 us` (❌ *26.95x slower*)     | `15.90 us` (❌ *84.06x slower*)    | `672.75 ns` (❌ *3.56x slower*)    | `189.19 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `9.29 ns` (✅ **1.00x**)        | `18.35 ns` (❌ *1.98x slower*)   | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `12.71 ns` (✅ **1.00x**)       | `24.07 ns` (❌ *1.89x slower*)   | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.82 ns` (✅ **1.00x**)        | `4.86 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.47 ns` (✅ **1.00x**)        | `4.40 ns` (✅ **1.01x faster**)  | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                        | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `538.39 ns` (✅ **1.00x**)      | `515.15 ns` (✅ **1.05x faster**) | `60.07 ns` (🚀 **8.96x faster**)      | `187.20 ns` (🚀 **2.88x faster**)    | `551.58 ns` (✅ **1.02x slower**)  | `1.14 us` (❌ *2.12x slower*)      |
| **`serialize_uncompressed`**             | `716.60 ns` (✅ **1.00x**)      | `743.64 ns` (✅ **1.04x slower**) | `59.80 ns` (🚀 **11.98x faster**)     | `187.31 ns` (🚀 **3.83x faster**)    | `555.82 ns` (✅ **1.29x faster**)  | `1.17 us` (❌ *1.63x slower*)      |
| **`deserialize_compressed`**             | `1.60 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `110.51 ns` (🚀 **14442.65x faster**) | `362.85 ns` (🚀 **4398.70x faster**) | `1.13 us` (🚀 **1418.27x faster**) | `2.24 us` (🚀 **712.06x faster**)  |
| **`deserialize_compressed_unchecked`**   | `297.53 us` (✅ **1.00x**)      | `298.72 us` (✅ **1.00x slower**) | `112.09 ns` (🚀 **2654.45x faster**)  | `366.63 ns` (🚀 **811.54x faster**)  | `1.07 us` (🚀 **278.01x faster**)  | `2.25 us` (🚀 **132.02x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.31 ms` (✅ **1.01x slower**)   | `112.41 ns` (🚀 **11555.81x faster**) | `356.39 ns` (🚀 **3644.88x faster**) | `1.10 us` (🚀 **1178.30x faster**) | `2.25 us` (🚀 **577.25x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `827.72 ns` (✅ **1.00x**)      | `816.24 ns` (✅ **1.01x faster**) | `109.27 ns` (🚀 **7.57x faster**)     | `366.24 ns` (🚀 **2.26x faster**)    | `1.11 us` (❌ *1.35x slower*)      | `2.24 us` (❌ *2.70x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.87 s` (✅ **1.00x**)        | `12.86 s` (✅ **1.00x faster**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `75.21 us` (✅ **1.00x**) | `296.20 us` (❌ *3.94x slower*)   | `7.05 ms` (❌ *93.71x slower*)     |
| **`legendre_for_qr`**    | `33.78 us` (✅ **1.00x**) | `293.93 us` (❌ *8.70x slower*)   | `302.70 us` (❌ *8.96x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.00 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.03x faster**)    |
| **`from_little-endian_bits`** | `129.43 ns` (✅ **1.00x**)      | `249.87 ns` (❌ *1.93x slower*)    |
| **`from_big-endian_bits`**    | `126.00 ns` (✅ **1.00x**)      | `253.11 ns` (❌ *2.01x slower*)    |
| **`comparison`**              | `4.98 ns` (✅ **1.00x**)        | `4.97 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `5.52 ns` (✅ **1.00x**)        | `5.48 ns` (✅ **1.01x faster**)    |
| **`is_zero`**                 | `4.75 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `95.19 ns` (✅ **1.00x**) | `329.47 ns` (❌ *3.46x slower*)    |
| **`into_bigint`** | `49.40 ns` (✅ **1.00x**) | `163.60 ns` (❌ *3.31x slower*)    |

### pairing_for_bw6_761optimized

|        | `g1_preparation_for_bw6_761optimized`          | `g2_preparation_for_bw6_761optimized`          | `miller_loop_for_bw6_761optimized`          | `final_exponentiation_for_bw6_761optimized`          | `full_pairing_for_bw6_761optimized`           |
|:-------|:-----------------------------------------------|:-----------------------------------------------|:--------------------------------------------|:-----------------------------------------------------|:--------------------------------------------- |
|        | `19.78 ns` (✅ **1.00x**)                       | `13.07 ns` (✅ **1.51x faster**)                | `4.65 ms` (❌ *234786.74x slower*)           | `4.37 ms` (❌ *220754.49x slower*)                    | `8.95 ms` (❌ *452476.32x slower*)             |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

