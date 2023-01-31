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
|        | `326.96 us` (✅ **1.00x**)                 | `2.29 ms` (❌ *7.00x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.42 us` (✅ **1.00x**)          | `4.42 us` (❌ *3.12x slower*)     | `33.61 ns` (🚀 **42.17x faster**) | `253.57 ns` (🚀 **5.59x faster**)  | `23.78 ns` (🚀 **59.60x faster**) | `9.91 ns` (🚀 **142.97x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.44 us` (✅ **1.00x**)          | `4.61 us` (❌ *3.20x slower*)     | `34.28 ns` (🚀 **42.09x faster**) | `257.49 ns` (🚀 **5.60x faster**)  | `20.34 ns` (🚀 **70.92x faster**) | `16.43 ns` (🚀 **87.80x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `998.93 ns` (✅ **1.00x**)        | `3.21 us` (❌ *3.21x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `1.04 us` (✅ **1.00x**)          | `3.33 us` (❌ *3.22x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `683.29 ns` (✅ **1.00x**)        | `2.13 us` (❌ *3.11x slower*)     | `18.53 ns` (🚀 **36.87x faster**) | `203.73 ns` (🚀 **3.35x faster**)  | `9.21 ns` (🚀 **74.21x faster**)  | `6.11 ns` (🚀 **111.88x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `466.82 us` (✅ **1.00x**)        | `1.40 ms` (❌ *2.99x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `30.93 ns` (❌ *4.61x slower*)    | `169.77 ns` (❌ *25.31x slower*)   | `19.69 ns` (❌ *2.94x slower*)    | `6.71 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `284.90 ns` (❌ *6.03x slower*)   | `7.37 us` (❌ *156.02x slower*)    | `80.79 ns` (❌ *1.71x slower*)    | `47.21 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `217.41 ns` (❌ *5.48x slower*)   | `5.35 us` (❌ *134.91x slower*)    | `71.32 ns` (❌ *1.80x slower*)    | `39.64 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `16.75 us` (❌ *2.35x slower*)    | `29.08 us` (❌ *4.09x slower*)     | `16.03 us` (❌ *2.25x slower*)    | `7.11 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `616.48 ns` (❌ *6.12x slower*)   | `14.80 us` (❌ *146.98x slower*)   | `132.53 ns` (❌ *1.32x slower*)   | `100.69 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `582.98 ns` (❌ *6.06x slower*)   | `14.47 us` (❌ *150.37x slower*)   | `183.34 ns` (❌ *1.91x slower*)   | `96.24 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.52 ns` (✅ **1.00x**)        | `10.41 ns` (❌ *1.38x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.68 ns` (✅ **1.00x**)        | `13.83 ns` (❌ *1.43x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.46 ns` (✅ **1.00x**)        | `4.67 ns` (✅ **1.05x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.17 ns` (✅ **1.00x**)        | `4.38 ns` (✅ **1.05x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                       | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `178.55 ns` (✅ **1.00x**)        | `243.73 ns` (❌ *1.37x slower*)   | `34.59 ns` (🚀 **5.16x faster**)    | `60.36 ns` (🚀 **2.96x faster**)     | `118.82 ns` (✅ **1.50x faster**)    | `741.58 ns` (❌ *4.15x slower*)    |
| **`serialize_uncompressed`**             | `226.80 ns` (✅ **1.00x**)        | `345.52 ns` (❌ *1.52x slower*)   | `33.69 ns` (🚀 **6.73x faster**)    | `62.23 ns` (🚀 **3.64x faster**)     | `122.47 ns` (🚀 **1.85x faster**)    | `735.15 ns` (❌ *3.24x slower*)    |
| **`deserialize_compressed`**             | `363.41 us` (✅ **1.00x**)        | `621.89 us` (❌ *1.71x slower*)   | `65.82 ns` (🚀 **5521.47x faster**) | `120.87 ns` (🚀 **3006.60x faster**) | `266.89 ns` (🚀 **1361.62x faster**) | `1.62 us` (🚀 **224.86x faster**)  |
| **`deserialize_compressed_unchecked`**   | `46.66 us` (✅ **1.00x**)         | `156.51 us` (❌ *3.35x slower*)   | `66.84 ns` (🚀 **698.11x faster**)  | `119.53 ns` (🚀 **390.38x faster**)  | `264.81 ns` (🚀 **176.20x faster**)  | `1.61 us` (🚀 **28.89x faster**)   |
| **`deserialize_uncompressed`**           | `322.91 us` (✅ **1.00x**)        | `476.56 us` (❌ *1.48x slower*)   | `65.73 ns` (🚀 **4913.07x faster**) | `118.41 ns` (🚀 **2727.02x faster**) | `271.13 ns` (🚀 **1191.01x faster**) | `1.61 us` (🚀 **200.50x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `318.10 ns` (✅ **1.00x**)        | `667.07 ns` (❌ *2.10x slower*)   | `65.83 ns` (🚀 **4.83x faster**)    | `119.89 ns` (🚀 **2.65x faster**)    | `273.53 ns` (✅ **1.16x faster**)    | `1.66 us` (❌ *5.21x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.84 s` (✅ **1.00x**)           | `8.27 s` (❌ *2.91x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `26.20 us` (✅ **1.00x**) | `46.87 us` (❌ *1.79x slower*)   | `156.22 us` (❌ *5.96x slower*)    |
| **`legendre_for_qr`**    | `15.07 us` (✅ **1.00x**) | `45.34 us` (❌ *3.01x slower*)   | `46.33 us` (❌ *3.07x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.69 ns` (✅ **1.00x**)        | `4.79 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `178.57 ns` (✅ **1.00x**)      | `257.55 ns` (❌ *1.44x slower*)    |
| **`from_big-endian_bits`**    | `179.63 ns` (✅ **1.00x**)      | `260.18 ns` (❌ *1.45x slower*)    |
| **`comparison`**              | `4.77 ns` (✅ **1.00x**)        | `4.86 ns` (✅ **1.02x slower**)    |
| **`equality`**                | `4.75 ns` (✅ **1.00x**)        | `6.30 ns` (❌ *1.33x slower*)      |
| **`is_zero`**                 | `4.37 ns` (✅ **1.00x**)        | `4.71 ns` (✅ **1.08x slower**)    |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.42 ns` (✅ **1.00x**) | `96.70 ns` (❌ *2.23x slower*)    |
| **`into_bigint`** | `32.23 ns` (✅ **1.00x**) | `49.96 ns` (❌ *1.55x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

