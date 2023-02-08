# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_377_optimized](#sample_bls12_377_optimized)
    - [arithmetic_for_bls12_377_optimized](#arithmetic_for_bls12_377_optimized)
    - [serialization_for_bls12_377_optimized](#serialization_for_bls12_377_optimized)
    - [msm_for_bls12_377_optimized](#msm_for_bls12_377_optimized)
    - [squareroot_for_bls12_377_optimized](#squareroot_for_bls12_377_optimized)
    - [bitwise_operations_for_bls12_377_optimized](#bitwise_operations_for_bls12_377_optimized)
    - [conversions_for_bls12_377_optimized](#conversions_for_bls12_377_optimized)

## Benchmark Results

### sample_bls12_377_optimized

|        | `g1projectivebls12_377_elements`          | `g2projectivebls12_377_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `192.63 us` (✅ **1.00x**)                 | `1.87 ms` (❌ *9.73x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.13 us` (✅ **1.00x**)          | `4.27 us` (❌ *3.77x slower*)     | `28.51 ns` (🚀 **39.77x faster**)  | `174.04 ns` (🚀 **6.51x faster**)  | `19.44 ns` (🚀 **58.32x faster**) | `8.30 ns` (🚀 **136.69x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.17 us` (✅ **1.00x**)          | `4.31 us` (❌ *3.69x slower*)     | `27.34 ns` (🚀 **42.73x faster**)  | `167.60 ns` (🚀 **6.97x faster**)  | `14.74 ns` (🚀 **79.23x faster**) | `8.60 ns` (🚀 **135.83x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `829.53 ns` (✅ **1.00x**)        | `3.06 us` (❌ *3.69x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `852.92 ns` (✅ **1.00x**)        | `3.11 us` (❌ *3.64x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `565.97 ns` (✅ **1.00x**)        | `2.04 us` (❌ *3.61x slower*)     | `12.88 ns` (🚀 **43.94x faster**)  | `101.83 ns` (🚀 **5.56x faster**)  | `7.48 ns` (🚀 **75.63x faster**)  | `5.21 ns` (🚀 **108.53x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `299.44 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.59x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.96 ns` (❌ *3.86x slower*)     | `104.01 ns` (❌ *17.49x slower*)   | `17.00 ns` (❌ *2.86x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `266.10 ns` (❌ *7.16x slower*)    | `6.68 us` (❌ *179.59x slower*)    | `71.10 ns` (❌ *1.91x slower*)    | `37.18 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `245.40 ns` (❌ *7.71x slower*)    | `4.69 us` (❌ *147.47x slower*)    | `58.90 ns` (❌ *1.85x slower*)    | `31.82 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.62 us` (❌ *2.16x slower*)     | `24.89 us` (❌ *3.95x slower*)     | `13.31 us` (❌ *2.11x slower*)    | `6.30 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `564.68 ns` (❌ *10.63x slower*)   | `13.60 us` (❌ *255.99x slower*)   | `111.33 ns` (❌ *2.10x slower*)   | `53.11 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `550.01 ns` (❌ *6.67x slower*)    | `13.51 us` (❌ *163.79x slower*)   | `156.87 ns` (❌ *1.90x slower*)   | `82.46 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.74 ns` (❌ *1.37x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.35 ns` (✅ **1.00x**)        | `212.76 ns` (❌ *1.42x slower*)   | `28.04 ns` (🚀 **5.36x faster**)    | `50.31 ns` (🚀 **2.99x faster**)    | `99.97 ns` (✅ **1.50x faster**)     | `628.91 ns` (❌ *4.18x slower*)    |
| **`serialize_uncompressed`**             | `197.77 ns` (✅ **1.00x**)        | `322.32 ns` (❌ *1.63x slower*)   | `27.94 ns` (🚀 **7.08x faster**)    | `50.59 ns` (🚀 **3.91x faster**)    | `99.96 ns` (🚀 **1.98x faster**)     | `631.01 ns` (❌ *3.19x slower*)    |
| **`deserialize_compressed`**             | `281.29 us` (✅ **1.00x**)        | `972.68 us` (❌ *3.46x slower*)   | `46.57 ns` (🚀 **6040.63x faster**) | `95.55 ns` (🚀 **2943.75x faster**) | `208.73 ns` (🚀 **1347.63x faster**) | `1.27 us` (🚀 **221.54x faster**)  |
| **`deserialize_compressed_unchecked`**   | `64.72 us` (✅ **1.00x**)         | `173.25 us` (❌ *2.68x slower*)   | `46.53 ns` (🚀 **1390.81x faster**) | `95.61 ns` (🚀 **676.86x faster**)  | `208.54 ns` (🚀 **310.32x faster**)  | `1.27 us` (🚀 **50.98x faster**)   |
| **`deserialize_uncompressed`**           | `216.79 us` (✅ **1.00x**)        | `794.45 us` (❌ *3.66x slower*)   | `46.47 ns` (🚀 **4664.97x faster**) | `95.79 ns` (🚀 **2263.19x faster**) | `207.81 ns` (🚀 **1043.21x faster**) | `1.27 us` (🚀 **170.57x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `225.63 ns` (✅ **1.00x**)        | `470.21 ns` (❌ *2.08x slower*)   | `46.48 ns` (🚀 **4.85x faster**)    | `96.29 ns` (🚀 **2.34x faster**)    | `207.68 ns` (✅ **1.09x faster**)    | `1.27 us` (❌ *5.64x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.29 s` (✅ **1.00x**)           | `8.01 s` (❌ *3.50x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.72 us` (✅ **1.00x**) | `64.28 us` (❌ *2.32x slower*)   | `171.84 us` (❌ *6.20x slower*)    |
| **`legendre_for_qr`**    | `9.52 us` (✅ **1.00x**)  | `28.99 us` (❌ *3.05x slower*)   | `29.33 us` (❌ *3.08x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.28 ns` (✅ **1.00x**)       | `108.02 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `61.06 ns` (✅ **1.00x**)       | `108.33 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.51 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.86 ns` (✅ **1.00x**) | `78.59 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.75 ns` (✅ **1.00x**) | `41.48 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

