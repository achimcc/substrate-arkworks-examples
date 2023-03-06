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
|        | `2.09 ms` (✅ **1.00x**)                 | `2.08 ms` (✅ **1.00x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                    | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.99 us` (✅ **1.00x**)        | `4.99 us` (✅ **1.00x slower**) | `109.91 ns` (🚀 **45.36x faster**) | `215.98 ns` (🚀 **23.09x faster**) | `35.91 ns` (🚀 **138.84x faster**) | `23.17 ns` (🚀 **215.17x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `5.03 us` (✅ **1.00x**)        | `5.04 us` (✅ **1.00x slower**) | `102.41 ns` (🚀 **49.15x faster**) | `202.46 ns` (🚀 **24.86x faster**) | `34.50 ns` (🚀 **145.91x faster**) | `18.13 ns` (🚀 **277.64x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.51 us` (✅ **1.00x**)        | `3.51 us` (✅ **1.00x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.55 us` (✅ **1.00x**)        | `3.55 us` (✅ **1.00x faster**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.32 us` (✅ **1.00x**)        | `2.33 us` (✅ **1.00x slower**) | `85.58 ns` (🚀 **27.13x faster**)  | `171.35 ns` (🚀 **13.55x faster**) | `26.08 ns` (🚀 **89.03x faster**)  | `13.41 ns` (🚀 **173.19x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.86 ms` (✅ **1.00x**)        | `1.86 ms` (✅ **1.00x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `83.64 ns` (❌ *4.01x slower*)     | `169.89 ns` (❌ *8.15x slower*)    | `29.26 ns` (❌ *1.40x slower*)     | `20.86 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.60 us` (❌ *31.18x slower*)     | `8.27 us` (❌ *99.41x slower*)     | `324.74 ns` (❌ *3.90x slower*)    | `83.23 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.87 us` (❌ *26.37x slower*)     | `5.85 us` (❌ *82.36x slower*)     | `260.64 ns` (❌ *3.67x slower*)    | `70.98 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `61.35 us` (❌ *3.80x slower*)     | `70.66 us` (❌ *4.37x slower*)     | `57.11 us` (❌ *3.53x slower*)     | `16.16 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.35 us` (❌ *42.12x slower*)     | `16.89 us` (❌ *133.08x slower*)   | `480.13 ns` (❌ *3.78x slower*)    | `126.91 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.29 us` (❌ *28.02x slower*)     | `16.73 us` (❌ *88.66x slower*)    | `681.59 ns` (❌ *3.61x slower*)    | `188.73 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `9.40 ns` (✅ **1.00x**)        | `18.83 ns` (❌ *2.00x slower*)   | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `12.87 ns` (✅ **1.00x**)       | `25.09 ns` (❌ *1.95x slower*)   | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.85 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.49 ns` (✅ **1.00x**)        | `4.49 ns` (✅ **1.00x slower**)  | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                        | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `562.85 ns` (✅ **1.00x**)      | `561.82 ns` (✅ **1.00x faster**) | `60.24 ns` (🚀 **9.34x faster**)      | `188.59 ns` (🚀 **2.98x faster**)    | `557.82 ns` (✅ **1.01x faster**)  | `1.19 us` (❌ *2.11x slower*)      |
| **`serialize_uncompressed`**             | `756.92 ns` (✅ **1.00x**)      | `758.22 ns` (✅ **1.00x slower**) | `60.20 ns` (🚀 **12.57x faster**)     | `188.48 ns` (🚀 **4.02x faster**)    | `557.39 ns` (✅ **1.36x faster**)  | `1.18 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.64 ms` (✅ **1.00x**)        | `1.64 ms` (✅ **1.00x slower**)   | `113.39 ns` (🚀 **14436.60x faster**) | `367.75 ns` (🚀 **4451.36x faster**) | `1.13 us` (🚀 **1442.94x faster**) | `2.29 us` (🚀 **715.19x faster**)  |
| **`deserialize_compressed_unchecked`**   | `302.68 us` (✅ **1.00x**)      | `302.74 us` (✅ **1.00x slower**) | `113.44 ns` (🚀 **2668.33x faster**)  | `367.87 ns` (🚀 **822.81x faster**)  | `1.13 us` (🚀 **266.84x faster**)  | `2.29 us` (🚀 **132.19x faster**)  |
| **`deserialize_uncompressed`**           | `1.34 ms` (✅ **1.00x**)        | `1.34 ms` (✅ **1.00x slower**)   | `113.55 ns` (🚀 **11766.64x faster**) | `368.23 ns` (🚀 **3628.55x faster**) | `1.14 us` (🚀 **1176.04x faster**) | `2.29 us` (🚀 **583.72x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `808.31 ns` (✅ **1.00x**)      | `807.58 ns` (✅ **1.00x faster**) | `113.63 ns` (🚀 **7.11x faster**)     | `367.95 ns` (🚀 **2.20x faster**)    | `1.14 us` (❌ *1.41x slower*)      | `2.29 us` (❌ *2.83x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `13.10 s` (✅ **1.00x**)        | `13.12 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `77.70 us` (✅ **1.00x**) | `301.07 us` (❌ *3.87x slower*)   | `7.20 ms` (❌ *92.65x slower*)     |
| **`legendre_for_qr`**    | `35.32 us` (✅ **1.00x**) | `301.64 us` (❌ *8.54x slower*)   | `308.03 us` (❌ *8.72x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)        | `5.09 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `131.47 ns` (✅ **1.00x**)      | `257.94 ns` (❌ *1.96x slower*)    |
| **`from_big-endian_bits`**    | `131.53 ns` (✅ **1.00x**)      | `258.57 ns` (❌ *1.97x slower*)    |
| **`comparison`**              | `5.04 ns` (✅ **1.00x**)        | `5.04 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `5.68 ns` (✅ **1.00x**)        | `6.54 ns` (❌ *1.15x slower*)      |
| **`is_zero`**                 | `4.81 ns` (✅ **1.00x**)        | `4.93 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `94.12 ns` (✅ **1.00x**) | `335.79 ns` (❌ *3.57x slower*)    |
| **`into_bigint`** | `49.77 ns` (✅ **1.00x**) | `169.95 ns` (❌ *3.42x slower*)    |

### pairing_for_bw6_761optimized

|        | `g1_preparation_for_bw6_761optimized`          | `g2_preparation_for_bw6_761optimized`          | `miller_loop_for_bw6_761optimized`          | `final_exponentiation_for_bw6_761optimized`          | `full_pairing_for_bw6_761optimized`           |
|:-------|:-----------------------------------------------|:-----------------------------------------------|:--------------------------------------------|:-----------------------------------------------------|:--------------------------------------------- |
|        | `19.27 ns` (✅ **1.00x**)                       | `13.35 ns` (✅ **1.44x faster**)                | `4.66 ms` (❌ *241947.08x slower*)           | `4.42 ms` (❌ *229472.72x slower*)                    | `9.10 ms` (❌ *472075.87x slower*)             |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

