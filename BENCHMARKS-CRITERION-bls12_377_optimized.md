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
|        | `349.40 us` (✅ **1.00x**)                 | `2.43 ms` (❌ *6.95x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `4.56 us` (❌ *3.59x slower*)     | `23.02 ns` (🚀 **55.22x faster**) | `182.65 ns` (🚀 **6.96x faster**)  | `12.54 ns` (🚀 **101.40x faster**) | `8.75 ns` (🚀 **145.31x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.31 us` (✅ **1.00x**)          | `4.62 us` (❌ *3.52x slower*)     | `23.24 ns` (🚀 **56.53x faster**) | `164.21 ns` (🚀 **8.00x faster**)  | `13.06 ns` (🚀 **100.55x faster**) | `8.83 ns` (🚀 **148.83x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `926.44 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.57x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `956.10 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.50x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `597.51 ns` (✅ **1.00x**)        | `2.26 us` (❌ *3.79x slower*)     | `12.52 ns` (🚀 **47.73x faster**) | `68.71 ns` (🚀 **8.70x faster**)   | `7.13 ns` (🚀 **83.85x faster**)   | `5.86 ns` (🚀 **102.02x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `478.92 us` (✅ **1.00x**)        | `1.55 ms` (❌ *3.24x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.12 ns` (❌ *3.76x slower*)    | `101.95 ns` (❌ *16.57x slower*)   | `18.56 ns` (❌ *3.02x slower*)     | `6.15 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `269.96 ns` (❌ *6.31x slower*)   | `7.09 us` (❌ *165.80x slower*)    | `76.23 ns` (❌ *1.78x slower*)     | `42.79 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `244.26 ns` (❌ *6.87x slower*)   | `5.00 us` (❌ *140.73x slower*)    | `66.66 ns` (❌ *1.88x slower*)     | `35.55 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.38 us` (❌ *2.14x slower*)    | `27.77 us` (❌ *3.87x slower*)     | `15.02 us` (❌ *2.09x slower*)     | `7.18 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `585.38 ns` (❌ *9.32x slower*)   | `14.59 us` (❌ *232.26x slower*)   | `117.00 ns` (❌ *1.86x slower*)    | `62.83 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `574.63 ns` (❌ *6.28x slower*)   | `14.49 us` (❌ *158.39x slower*)   | `163.39 ns` (❌ *1.79x slower*)    | `91.51 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.64 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.68 ns` (✅ **1.00x**)        | `10.36 ns` (❌ *1.19x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `158.54 ns` (✅ **1.00x**)        | `221.49 ns` (❌ *1.40x slower*)   | `31.14 ns` (🚀 **5.09x faster**)    | `56.73 ns` (🚀 **2.79x faster**)    | `111.08 ns` (✅ **1.43x faster**)    | `700.63 ns` (❌ *4.42x slower*)    |
| **`serialize_uncompressed`**             | `208.61 ns` (✅ **1.00x**)        | `338.17 ns` (❌ *1.62x slower*)   | `31.63 ns` (🚀 **6.59x faster**)    | `56.11 ns` (🚀 **3.72x faster**)    | `111.27 ns` (🚀 **1.87x faster**)    | `700.15 ns` (❌ *3.36x slower*)    |
| **`deserialize_compressed`**             | `386.24 us` (✅ **1.00x**)        | `1.25 ms` (❌ *3.24x slower*)     | `52.01 ns` (🚀 **7425.97x faster**) | `93.07 ns` (🚀 **4150.22x faster**) | `210.14 ns` (🚀 **1837.99x faster**) | `1.30 us` (🚀 **297.05x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.75 us` (✅ **1.00x**)         | `185.68 us` (❌ *2.70x slower*)   | `52.05 ns` (🚀 **1320.80x faster**) | `93.20 ns` (🚀 **737.63x faster**)  | `210.40 ns` (🚀 **326.75x faster**)  | `1.30 us` (🚀 **52.89x faster**)   |
| **`deserialize_uncompressed`**           | `317.85 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.35x slower*)     | `51.72 ns` (🚀 **6145.49x faster**) | `93.47 ns` (🚀 **3400.50x faster**) | `210.31 ns` (🚀 **1511.34x faster**) | `1.30 us` (🚀 **244.06x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `236.29 ns` (✅ **1.00x**)        | `557.07 ns` (❌ *2.36x slower*)   | `51.71 ns` (🚀 **4.57x faster**)    | `93.25 ns` (🚀 **2.53x faster**)    | `210.33 ns` (✅ **1.12x faster**)    | `1.30 us` (❌ *5.51x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.46 s` (✅ **1.00x**)           | `8.35 s` (❌ *3.40x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.12 us` (✅ **1.00x**) | `68.33 us` (❌ *2.20x slower*)   | `184.73 us` (❌ *5.94x slower*)    |
| **`legendre_for_qr`**    | `10.89 us` (✅ **1.00x**) | `32.10 us` (❌ *2.95x slower*)   | `32.29 us` (❌ *2.97x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.19 ns` (✅ **1.00x**)       | `90.58 ns` (❌ *1.84x slower*)    |
| **`from_big-endian_bits`**    | `49.07 ns` (✅ **1.00x**)       | `89.94 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)        | `5.09 ns` (✅ **1.04x slower**)   |
| **`equality`**                | `5.38 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.85 ns` (✅ **1.00x**) | `75.09 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `22.81 ns` (✅ **1.00x**) | `46.87 ns` (❌ *2.05x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

