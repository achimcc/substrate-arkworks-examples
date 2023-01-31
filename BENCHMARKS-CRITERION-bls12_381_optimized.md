# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381_optimized](#sample_bls12_381_optimized)
    - [arithmetic_for_bls12_381_optimized](#arithmetic_for_bls12_381_optimized)
    - [serialization_for_bls12_381_optimized](#serialization_for_bls12_381_optimized)
    - [msm_for_bls12_381_optimized](#msm_for_bls12_381_optimized)
    - [squareroot_for_bls12_381_optimized](#squareroot_for_bls12_381_optimized)
    - [bitwise_operations_for_bls12_381_optimized](#bitwise_operations_for_bls12_381_optimized)
    - [conversions_for_bls12_381_optimized](#conversions_for_bls12_381_optimized)

## Benchmark Results

### sample_bls12_381_optimized

|        | `g1projectivebls12_381_elements`          | `g2projectivebls12_381_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `236.65 us` (✅ **1.00x**)                 | `1.89 ms` (❌ *7.97x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.12 us` (✅ **1.00x**)          | `3.57 us` (❌ *3.19x slower*)     | `24.12 ns` (🚀 **46.38x faster**) | `177.62 ns` (🚀 **6.30x faster**)  | `17.22 ns` (🚀 **64.95x faster**) | `8.18 ns` (🚀 **136.79x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.15 us` (✅ **1.00x**)          | `3.61 us` (❌ *3.13x slower*)     | `27.79 ns` (🚀 **41.52x faster**) | `173.12 ns` (🚀 **6.66x faster**)  | `15.20 ns` (🚀 **75.89x faster**) | `8.55 ns` (🚀 **134.85x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `816.88 ns` (✅ **1.00x**)        | `2.57 us` (❌ *3.15x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `840.62 ns` (✅ **1.00x**)        | `2.61 us` (❌ *3.11x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `562.96 ns` (✅ **1.00x**)        | `1.63 us` (❌ *2.90x slower*)     | `14.23 ns` (🚀 **39.57x faster**) | `103.93 ns` (🚀 **5.42x faster**)  | `7.66 ns` (🚀 **73.48x faster**)  | `5.41 ns` (🚀 **104.08x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `370.50 us` (✅ **1.00x**)        | `1.14 ms` (❌ *3.07x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `19.85 ns` (❌ *3.33x slower*)    | `101.26 ns` (❌ *17.01x slower*)   | `14.80 ns` (❌ *2.49x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `197.09 ns` (❌ *5.07x slower*)   | `5.77 us` (❌ *148.55x slower*)    | `70.21 ns` (❌ *1.81x slower*)    | `38.85 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `174.96 ns` (❌ *4.95x slower*)   | `4.05 us` (❌ *114.47x slower*)    | `51.51 ns` (❌ *1.46x slower*)    | `35.34 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.84 us` (❌ *2.14x slower*)    | `22.98 us` (❌ *3.55x slower*)     | `13.56 us` (❌ *2.10x slower*)    | `6.46 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `436.56 ns` (❌ *5.36x slower*)   | `11.74 us` (❌ *144.29x slower*)   | `94.18 ns` (❌ *1.16x slower*)    | `81.38 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `420.52 ns` (❌ *5.16x slower*)   | `11.67 us` (❌ *143.04x slower*)   | `137.07 ns` (❌ *1.68x slower*)   | `81.57 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**)        | `10.73 ns` (❌ *1.37x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `142.49 ns` (✅ **1.00x**)        | `168.10 ns` (❌ *1.18x slower*)   | `30.11 ns` (🚀 **4.73x faster**)    | `43.76 ns` (🚀 **3.26x faster**)    | `97.88 ns` (✅ **1.46x faster**)     | `631.93 ns` (❌ *4.43x slower*)    |
| **`serialize_uncompressed`**             | `181.46 ns` (✅ **1.00x**)        | `269.61 ns` (❌ *1.49x slower*)   | `30.06 ns` (🚀 **6.04x faster**)    | `49.42 ns` (🚀 **3.67x faster**)    | `97.87 ns` (🚀 **1.85x faster**)     | `632.01 ns` (❌ *3.48x slower*)    |
| **`deserialize_compressed`**             | `291.59 us` (✅ **1.00x**)        | `503.98 us` (❌ *1.73x slower*)   | `47.49 ns` (🚀 **6139.61x faster**) | `94.55 ns` (🚀 **3084.05x faster**) | `180.51 ns` (🚀 **1615.37x faster**) | `1.27 us` (🚀 **229.28x faster**)  |
| **`deserialize_compressed_unchecked`**   | `36.13 us` (✅ **1.00x**)         | `122.85 us` (❌ *3.40x slower*)   | `47.45 ns` (🚀 **761.47x faster**)  | `82.89 ns` (🚀 **435.86x faster**)  | `206.03 ns` (🚀 **175.36x faster**)  | `1.27 us` (🚀 **28.47x faster**)   |
| **`deserialize_uncompressed`**           | `255.94 us` (✅ **1.00x**)        | `381.58 us` (❌ *1.49x slower*)   | `47.40 ns` (🚀 **5400.21x faster**) | `93.81 ns` (🚀 **2728.36x faster**) | `205.60 ns` (🚀 **1244.88x faster**) | `1.27 us` (🚀 **201.17x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `232.27 ns` (✅ **1.00x**)        | `499.77 ns` (❌ *2.15x slower*)   | `47.40 ns` (🚀 **4.90x faster**)    | `82.80 ns` (🚀 **2.81x faster**)    | `205.70 ns` (✅ **1.13x faster**)    | `1.12 us` (❌ *4.82x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.24 s` (✅ **1.00x**)           | `6.67 s` (❌ *2.97x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.10 us` (✅ **1.00x**) | `35.73 us` (❌ *1.62x slower*)   | `121.96 us` (❌ *5.52x slower*)    |
| **`legendre_for_qr`**    | `12.53 us` (✅ **1.00x**) | `35.88 us` (❌ *2.86x slower*)   | `35.98 us` (❌ *2.87x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)   |
| **`from_little-endian_bits`** | `60.25 ns` (✅ **1.00x**)       | `96.51 ns` (❌ *1.60x slower*)    |
| **`from_big-endian_bits`**    | `60.39 ns` (✅ **1.00x**)       | `96.26 ns` (❌ *1.59x slower*)    |
| **`comparison`**              | `3.96 ns` (✅ **1.00x**)        | `4.20 ns` (✅ **1.06x slower**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.14 ns` (✅ **1.09x faster**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `3.53 ns` (✅ **1.11x faster**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `33.00 ns` (✅ **1.00x**) | `69.99 ns` (❌ *2.12x slower*)    |
| **`into_bigint`** | `22.25 ns` (✅ **1.00x**) | `36.31 ns` (❌ *1.63x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

