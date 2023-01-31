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
|        | `384.65 us` (✅ **1.00x**)                 | `2.65 ms` (❌ *6.88x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.39 us` (✅ **1.00x**)          | `5.13 us` (❌ *3.70x slower*)     | `32.81 ns` (🚀 **42.23x faster**)  | `261.66 ns` (🚀 **5.29x faster**)  | `24.11 ns` (🚀 **57.45x faster**) | `10.14 ns` (🚀 **136.69x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.42 us` (✅ **1.00x**)          | `5.48 us` (❌ *3.85x slower*)     | `35.28 ns` (🚀 **40.31x faster**)  | `257.91 ns` (🚀 **5.52x faster**)  | `19.92 ns` (🚀 **71.42x faster**) | `12.47 ns` (🚀 **114.06x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `1.02 us` (✅ **1.00x**)          | `3.69 us` (❌ *3.61x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `1.04 us` (✅ **1.00x**)          | `3.91 us` (❌ *3.76x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `693.45 ns` (✅ **1.00x**)        | `2.43 us` (❌ *3.51x slower*)     | `18.27 ns` (🚀 **37.96x faster**)  | `215.41 ns` (🚀 **3.22x faster**)  | `8.98 ns` (🚀 **77.23x faster**)  | `6.13 ns` (🚀 **113.21x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `524.82 us` (✅ **1.00x**)        | `1.75 ms` (❌ *3.33x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `50.65 ns` (❌ *7.34x slower*)     | `175.99 ns` (❌ *25.51x slower*)   | `20.44 ns` (❌ *2.96x slower*)    | `6.90 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `342.18 ns` (❌ *7.21x slower*)    | `8.61 us` (❌ *181.30x slower*)    | `90.44 ns` (❌ *1.90x slower*)    | `47.48 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `312.25 ns` (❌ *8.53x slower*)    | `6.24 us` (❌ *170.43x slower*)    | `76.18 ns` (❌ *2.08x slower*)    | `36.62 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `16.74 us` (❌ *2.28x slower*)     | `31.88 us` (❌ *4.34x slower*)     | `16.83 us` (❌ *2.29x slower*)    | `7.34 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `748.63 ns` (❌ *11.64x slower*)   | `17.63 us` (❌ *274.14x slower*)   | `143.75 ns` (❌ *2.24x slower*)   | `64.31 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `694.31 ns` (❌ *7.12x slower*)    | `17.23 us` (❌ *176.58x slower*)   | `191.71 ns` (❌ *1.96x slower*)   | `97.57 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.76 ns` (✅ **1.00x**)        | `10.90 ns` (❌ *1.40x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.77 ns` (✅ **1.00x**)       | `13.78 ns` (❌ *1.28x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.56 ns` (✅ **1.00x**)        | `4.64 ns` (✅ **1.02x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.48 ns` (✅ **1.00x**)        | `4.34 ns` (✅ **1.03x faster**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                       | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `173.99 ns` (✅ **1.00x**)        | `244.59 ns` (❌ *1.41x slower*)   | `34.45 ns` (🚀 **5.05x faster**)    | `59.50 ns` (🚀 **2.92x faster**)     | `119.55 ns` (✅ **1.46x faster**)    | `753.21 ns` (❌ *4.33x slower*)    |
| **`serialize_uncompressed`**             | `236.56 ns` (✅ **1.00x**)        | `380.09 ns` (❌ *1.61x slower*)   | `35.40 ns` (🚀 **6.68x faster**)    | `61.14 ns` (🚀 **3.87x faster**)     | `125.67 ns` (🚀 **1.88x faster**)    | `749.70 ns` (❌ *3.17x slower*)    |
| **`deserialize_compressed`**             | `438.36 us` (✅ **1.00x**)        | `1.41 ms` (❌ *3.23x slower*)     | `69.52 ns` (🚀 **6305.86x faster**) | `118.70 ns` (🚀 **3693.16x faster**) | `271.47 ns` (🚀 **1614.80x faster**) | `1.64 us` (🚀 **267.85x faster**)  |
| **`deserialize_compressed_unchecked`**   | `80.19 us` (✅ **1.00x**)         | `212.90 us` (❌ *2.66x slower*)   | `64.67 ns` (🚀 **1239.87x faster**) | `116.04 ns` (🚀 **691.03x faster**)  | `261.01 ns` (🚀 **307.22x faster**)  | `1.62 us` (🚀 **49.45x faster**)   |
| **`deserialize_uncompressed`**           | `336.61 us` (✅ **1.00x**)        | `1.14 ms` (❌ *3.39x slower*)     | `64.94 ns` (🚀 **5183.73x faster**) | `122.59 ns` (🚀 **2745.82x faster**) | `270.65 ns` (🚀 **1243.72x faster**) | `1.62 us` (🚀 **208.37x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `277.21 ns` (✅ **1.00x**)        | `610.56 ns` (❌ *2.20x slower*)   | `65.70 ns` (🚀 **4.22x faster**)    | `122.17 ns` (🚀 **2.27x faster**)    | `266.61 ns` (✅ **1.04x faster**)    | `1.62 us` (❌ *5.86x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.82 s` (✅ **1.00x**)           | `10.13 s` (❌ *3.59x slower*)      |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `32.95 us` (✅ **1.00x**) | `83.39 us` (❌ *2.53x slower*)   | `226.54 us` (❌ *6.88x slower*)    |
| **`legendre_for_qr`**    | `11.61 us` (✅ **1.00x**) | `39.15 us` (❌ *3.37x slower*)   | `39.65 us` (❌ *3.41x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.75 ns` (✅ **1.00x**)        | `4.93 ns` (✅ **1.04x slower**)    |
| **`from_little-endian_bits`** | `177.72 ns` (✅ **1.00x**)      | `262.88 ns` (❌ *1.48x slower*)    |
| **`from_big-endian_bits`**    | `180.23 ns` (✅ **1.00x**)      | `252.75 ns` (❌ *1.40x slower*)    |
| **`comparison`**              | `4.77 ns` (✅ **1.00x**)        | `4.96 ns` (✅ **1.04x slower**)    |
| **`equality`**                | `5.21 ns` (✅ **1.00x**)        | `5.88 ns` (❌ *1.13x slower*)      |
| **`is_zero`**                 | `4.65 ns` (✅ **1.00x**)        | `4.81 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.37 ns` (✅ **1.00x**) | `96.21 ns` (❌ *2.27x slower*)    |
| **`into_bigint`** | `29.94 ns` (✅ **1.00x**) | `50.68 ns` (❌ *1.69x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

