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

## Benchmark Results

### sample_bw6_761_optimized

|        | `g1projectivebw6_761_elements`          | `g2projectivebw6_761_elements`           |
|:-------|:----------------------------------------|:---------------------------------------- |
|        | `2.87 ms` (✅ **1.00x**)                 | `3.00 ms` (✅ **1.05x slower**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                    | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `6.96 us` (✅ **1.00x**)        | `7.26 us` (✅ **1.04x slower**) | `118.58 ns` (🚀 **58.68x faster**) | `235.11 ns` (🚀 **29.60x faster**) | `38.39 ns` (🚀 **181.27x faster**) | `24.44 ns` (🚀 **284.79x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `7.01 us` (✅ **1.00x**)        | `7.24 us` (✅ **1.03x slower**) | `105.20 ns` (🚀 **66.66x faster**) | `202.78 ns` (🚀 **34.58x faster**) | `34.16 ns` (🚀 **205.25x faster**) | `20.73 ns` (🚀 **338.29x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `4.83 us` (✅ **1.00x**)        | `4.95 us` (✅ **1.02x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `4.89 us` (✅ **1.00x**)        | `4.98 us` (✅ **1.02x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `3.38 us` (✅ **1.00x**)        | `3.23 us` (✅ **1.05x faster**) | `87.30 ns` (🚀 **38.72x faster**)  | `180.68 ns` (🚀 **18.71x faster**) | `25.04 ns` (🚀 **135.00x faster**) | `13.66 ns` (🚀 **247.46x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `2.59 ms` (✅ **1.00x**)        | `2.60 ms` (✅ **1.00x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `98.68 ns` (❌ *4.54x slower*)     | `177.95 ns` (❌ *8.18x slower*)    | `31.40 ns` (❌ *1.44x slower*)     | `21.75 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `3.46 us` (❌ *38.71x slower*)     | `11.06 us` (❌ *123.88x slower*)   | `415.17 ns` (❌ *4.65x slower*)    | `89.30 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.45 us` (❌ *26.70x slower*)     | `7.46 us` (❌ *81.26x slower*)     | `309.38 ns` (❌ *3.37x slower*)    | `91.80 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `64.96 us` (❌ *3.97x slower*)     | `77.96 us` (❌ *4.76x slower*)     | `60.40 us` (❌ *3.69x slower*)     | `16.37 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `7.14 us` (❌ *53.53x slower*)     | `21.99 us` (❌ *164.92x slower*)   | `501.82 ns` (❌ *3.76x slower*)    | `133.35 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `7.02 us` (❌ *35.60x slower*)     | `21.73 us` (❌ *110.12x slower*)   | `914.94 ns` (❌ *4.64x slower*)    | `197.31 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `11.44 ns` (✅ **1.00x**)       | `20.27 ns` (❌ *1.77x slower*)   | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `14.42 ns` (✅ **1.00x**)       | `26.78 ns` (❌ *1.86x slower*)   | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.91 ns` (✅ **1.05x slower**)  | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.12 ns` (✅ **1.00x**)        | `4.25 ns` (✅ **1.03x slower**)  | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                        | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `720.39 ns` (✅ **1.00x**)      | `715.83 ns` (✅ **1.01x faster**) | `63.34 ns` (🚀 **11.37x faster**)     | `230.78 ns` (🚀 **3.12x faster**)    | `677.74 ns` (✅ **1.06x faster**)  | `1.43 us` (❌ *1.98x slower*)      |
| **`serialize_uncompressed`**             | `1.01 us` (✅ **1.00x**)        | `965.92 ns` (✅ **1.05x faster**) | `61.56 ns` (🚀 **16.46x faster**)     | `228.06 ns` (🚀 **4.44x faster**)    | `677.24 ns` (✅ **1.50x faster**)  | `1.43 us` (❌ *1.41x slower*)      |
| **`deserialize_compressed`**             | `2.32 ms` (✅ **1.00x**)        | `2.27 ms` (✅ **1.02x faster**)   | `131.92 ns` (🚀 **17612.72x faster**) | `496.94 ns` (🚀 **4675.49x faster**) | `1.52 us` (🚀 **1531.27x faster**) | `3.05 us` (🚀 **761.48x faster**)  |
| **`deserialize_compressed_unchecked`**   | `424.23 us` (✅ **1.00x**)      | `432.62 us` (✅ **1.02x slower**) | `133.71 ns` (🚀 **3172.72x faster**)  | `499.98 ns` (🚀 **848.48x faster**)  | `1.50 us` (🚀 **283.36x faster**)  | `3.04 us` (🚀 **139.72x faster**)  |
| **`deserialize_uncompressed`**           | `1.89 ms` (✅ **1.00x**)        | `1.90 ms` (✅ **1.01x slower**)   | `139.78 ns` (🚀 **13540.70x faster**) | `489.39 ns` (🚀 **3867.56x faster**) | `1.48 us` (🚀 **1276.12x faster**) | `3.13 us` (🚀 **605.52x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `1.15 us` (✅ **1.00x**)        | `1.14 us` (✅ **1.01x faster**)   | `133.56 ns` (🚀 **8.62x faster**)     | `498.22 ns` (🚀 **2.31x faster**)    | `1.51 us` (❌ *1.31x slower*)      | `3.04 us` (❌ *2.64x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `18.53 s` (✅ **1.00x**)        | `17.92 s` (✅ **1.03x faster**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `94.19 us` (✅ **1.00x**) | `407.57 us` (❌ *4.33x slower*)   | `9.09 ms` (❌ *96.55x slower*)     |
| **`legendre_for_qr`**    | `45.74 us` (✅ **1.00x**) | `408.23 us` (❌ *8.93x slower*)   | `418.52 us` (❌ *9.15x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.78 ns` (✅ **1.00x**)        | `4.94 ns` (✅ **1.03x slower**)    |
| **`from_little-endian_bits`** | `131.91 ns` (✅ **1.00x**)      | `253.22 ns` (❌ *1.92x slower*)    |
| **`from_big-endian_bits`**    | `123.08 ns` (✅ **1.00x**)      | `255.00 ns` (❌ *2.07x slower*)    |
| **`comparison`**              | `4.83 ns` (✅ **1.00x**)        | `6.92 ns` (❌ *1.43x slower*)      |
| **`equality`**                | `5.86 ns` (✅ **1.00x**)        | `5.90 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `4.67 ns` (✅ **1.00x**)        | `4.70 ns` (✅ **1.01x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `94.26 ns` (✅ **1.00x**) | `446.85 ns` (❌ *4.74x slower*)    |
| **`into_bigint`** | `50.66 ns` (✅ **1.00x**) | `214.08 ns` (❌ *4.23x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

