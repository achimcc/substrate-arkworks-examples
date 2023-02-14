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
|        | `196.56 us` (✅ **1.00x**)                 | `1.64 ms` (❌ *8.33x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.11 us` (✅ **1.00x**)          | `3.57 us` (❌ *3.20x slower*)     | `26.58 ns` (🚀 **41.94x faster**) | `179.18 ns` (🚀 **6.22x faster**)  | `19.18 ns` (🚀 **58.13x faster**) | `8.20 ns` (🚀 **135.99x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.15 us` (✅ **1.00x**)          | `3.62 us` (❌ *3.15x slower*)     | `27.60 ns` (🚀 **41.62x faster**) | `170.91 ns` (🚀 **6.72x faster**)  | `14.72 ns` (🚀 **78.03x faster**) | `8.63 ns` (🚀 **133.15x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `815.29 ns` (✅ **1.00x**)        | `2.58 us` (❌ *3.17x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `839.42 ns` (✅ **1.00x**)        | `2.62 us` (❌ *3.12x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `563.00 ns` (✅ **1.00x**)        | `1.63 us` (❌ *2.90x slower*)     | `13.03 ns` (🚀 **43.21x faster**) | `105.30 ns` (🚀 **5.35x faster**)  | `7.64 ns` (🚀 **73.71x faster**)  | `5.40 ns` (🚀 **104.23x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `298.63 us` (✅ **1.00x**)        | `883.08 us` (❌ *2.96x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.98 ns` (❌ *3.86x slower*)    | `106.93 ns` (❌ *17.98x slower*)   | `16.76 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `224.66 ns` (❌ *5.79x slower*)   | `5.74 us` (❌ *148.05x slower*)    | `70.27 ns` (❌ *1.81x slower*)    | `38.80 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `174.71 ns` (❌ *4.91x slower*)   | `4.04 us` (❌ *113.67x slower*)    | `58.45 ns` (❌ *1.64x slower*)    | `35.57 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.85 us` (❌ *2.17x slower*)    | `23.09 us` (❌ *3.62x slower*)     | `13.55 us` (❌ *2.12x slower*)    | `6.38 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `492.99 ns` (❌ *6.06x slower*)   | `11.74 us` (❌ *144.34x slower*)   | `106.38 ns` (❌ *1.31x slower*)   | `81.35 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `474.30 ns` (❌ *5.84x slower*)   | `11.67 us` (❌ *143.59x slower*)   | `157.10 ns` (❌ *1.93x slower*)   | `81.25 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**)        | `10.63 ns` (❌ *1.35x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `141.38 ns` (✅ **1.00x**)        | `191.06 ns` (❌ *1.35x slower*)   | `29.90 ns` (🚀 **4.73x faster**)    | `49.62 ns` (🚀 **2.85x faster**)    | `98.28 ns` (✅ **1.44x faster**)    | `634.30 ns` (❌ *4.49x slower*)    |
| **`serialize_uncompressed`**             | `178.92 ns` (✅ **1.00x**)        | `269.54 ns` (❌ *1.51x slower*)   | `29.83 ns` (🚀 **6.00x faster**)    | `49.63 ns` (🚀 **3.61x faster**)    | `98.27 ns` (🚀 **1.82x faster**)    | `629.88 ns` (❌ *3.52x slower*)    |
| **`deserialize_compressed`**             | `148.27 us` (✅ **1.00x**)        | `258.01 us` (❌ *1.74x slower*)   | `46.50 ns` (🚀 **3188.35x faster**) | `93.62 ns` (🚀 **1583.67x faster**) | `206.07 ns` (🚀 **719.50x faster**) | `1.26 us` (🚀 **117.29x faster**)  |
| **`deserialize_compressed_unchecked`**   | `36.04 us` (✅ **1.00x**)         | `122.84 us` (❌ *3.41x slower*)   | `46.50 ns` (🚀 **775.05x faster**)  | `93.64 ns` (🚀 **384.89x faster**)  | `206.06 ns` (🚀 **174.90x faster**) | `1.26 us` (🚀 **28.51x faster**)   |
| **`deserialize_uncompressed`**           | `111.96 us` (✅ **1.00x**)        | `134.87 us` (❌ *1.20x slower*)   | `46.48 ns` (🚀 **2408.92x faster**) | `93.62 ns` (🚀 **1195.90x faster**) | `207.30 ns` (🚀 **540.09x faster**) | `1.27 us` (🚀 **88.50x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `231.85 ns` (✅ **1.00x**)        | `497.05 ns` (❌ *2.14x slower*)   | `46.50 ns` (🚀 **4.99x faster**)    | `94.30 ns` (🚀 **2.46x faster**)    | `206.02 ns` (✅ **1.13x faster**)   | `1.27 us` (❌ *5.49x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.27 s` (✅ **1.00x**)           | `6.76 s` (❌ *2.98x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.15 us` (✅ **1.00x**) | `35.62 us` (❌ *1.61x slower*)   | `121.84 us` (❌ *5.50x slower*)    |
| **`legendre_for_qr`**    | `12.50 us` (✅ **1.00x**) | `35.90 us` (❌ *2.87x slower*)   | `35.91 us` (❌ *2.87x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.97 ns` (✅ **1.00x**)       | `108.95 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `61.03 ns` (✅ **1.00x**)       | `108.91 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)        | `4.31 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.16 ns` (✅ **1.00x**) | `79.36 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.57 ns` (✅ **1.00x**) | `41.45 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

