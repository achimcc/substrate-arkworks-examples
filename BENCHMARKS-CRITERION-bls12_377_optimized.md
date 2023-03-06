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
    - [pairing_for_bls12_377optimized](#pairing_for_bls12_377optimized)

## Benchmark Results

### sample_bls12_377_optimized

|        | `g1projectivebls12_377_elements`          | `g2projectivebls12_377_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `227.83 us` (✅ **1.00x**)                 | `2.21 ms` (❌ *9.72x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.36 us` (✅ **1.00x**)          | `5.03 us` (❌ *3.71x slower*)     | `34.91 ns` (🚀 **38.90x faster**)  | `209.40 ns` (🚀 **6.49x faster**)  | `22.23 ns` (🚀 **61.09x faster**) | `9.92 ns` (🚀 **136.89x faster**)   |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.40 us` (✅ **1.00x**)          | `5.12 us` (❌ *3.66x slower*)     | `32.24 ns` (🚀 **43.36x faster**)  | `202.49 ns` (🚀 **6.90x faster**)  | `17.40 ns` (🚀 **80.32x faster**) | `10.35 ns` (🚀 **135.10x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `984.34 ns` (✅ **1.00x**)        | `3.60 us` (❌ *3.66x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `1.02 us` (✅ **1.00x**)          | `3.66 us` (❌ *3.59x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `678.17 ns` (✅ **1.00x**)        | `2.40 us` (❌ *3.53x slower*)     | `15.13 ns` (🚀 **44.82x faster**)  | `122.81 ns` (🚀 **5.52x faster**)  | `8.86 ns` (🚀 **76.52x faster**)  | `10.72 ns` (🚀 **63.25x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `355.85 us` (✅ **1.00x**)        | `1.27 ms` (❌ *3.57x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `27.35 ns` (❌ *3.92x slower*)     | `129.48 ns` (❌ *18.57x slower*)   | `19.79 ns` (❌ *2.84x slower*)    | `6.97 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `314.96 ns` (❌ *7.18x slower*)    | `7.76 us` (❌ *176.81x slower*)    | `82.35 ns` (❌ *1.88x slower*)    | `43.89 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `291.65 ns` (❌ *7.97x slower*)    | `5.43 us` (❌ *148.49x slower*)    | `69.44 ns` (❌ *1.90x slower*)    | `36.58 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `16.40 us` (❌ *2.20x slower*)     | `29.01 us` (❌ *3.89x slower*)     | `15.91 us` (❌ *2.13x slower*)    | `7.45 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `680.90 ns` (❌ *10.84x slower*)   | `15.58 us` (❌ *248.03x slower*)   | `131.12 ns` (❌ *2.09x slower*)   | `62.82 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `651.48 ns` (❌ *6.61x slower*)    | `15.85 us` (❌ *160.88x slower*)   | `181.66 ns` (❌ *1.84x slower*)   | `98.55 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.73 ns` (✅ **1.00x**)        | `9.30 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `8.95 ns` (✅ **1.00x**)        | `12.68 ns` (❌ *1.42x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.60 ns` (✅ **1.00x**)        | `4.79 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.41 ns` (✅ **1.00x**)        | `4.41 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                       | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `170.50 ns` (✅ **1.00x**)        | `249.66 ns` (❌ *1.46x slower*)   | `33.09 ns` (🚀 **5.15x faster**)    | `59.34 ns` (🚀 **2.87x faster**)     | `119.41 ns` (✅ **1.43x faster**)    | `738.91 ns` (❌ *4.33x slower*)    |
| **`serialize_uncompressed`**             | `235.73 ns` (✅ **1.00x**)        | `383.51 ns` (❌ *1.63x slower*)   | `32.51 ns` (🚀 **7.25x faster**)    | `58.85 ns` (🚀 **4.01x faster**)     | `118.12 ns` (🚀 **2.00x faster**)    | `746.51 ns` (❌ *3.17x slower*)    |
| **`deserialize_compressed`**             | `332.94 us` (✅ **1.00x**)        | `1.14 ms` (❌ *3.43x slower*)     | `54.53 ns` (🚀 **6105.95x faster**) | `110.59 ns` (🚀 **3010.57x faster**) | `241.77 ns` (🚀 **1377.09x faster**) | `1.49 us` (🚀 **223.41x faster**)  |
| **`deserialize_compressed_unchecked`**   | `75.50 us` (✅ **1.00x**)         | `206.34 us` (❌ *2.73x slower*)   | `55.37 ns` (🚀 **1363.56x faster**) | `111.10 ns` (🚀 **679.58x faster**)  | `248.63 ns` (🚀 **303.67x faster**)  | `1.49 us` (🚀 **50.53x faster**)   |
| **`deserialize_uncompressed`**           | `255.78 us` (✅ **1.00x**)        | `941.11 us` (❌ *3.68x slower*)   | `55.32 ns` (🚀 **4623.87x faster**) | `114.06 ns` (🚀 **2242.64x faster**) | `247.52 ns` (🚀 **1033.39x faster**) | `1.49 us` (🚀 **171.41x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `270.68 ns` (✅ **1.00x**)        | `544.31 ns` (❌ *2.01x slower*)   | `54.66 ns` (🚀 **4.95x faster**)    | `113.51 ns` (🚀 **2.38x faster**)    | `247.76 ns` (✅ **1.09x faster**)    | `1.48 us` (❌ *5.48x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.66 s` (✅ **1.00x**)           | `9.28 s` (❌ *3.49x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `32.75 us` (✅ **1.00x**) | `76.49 us` (❌ *2.34x slower*)   | `205.63 us` (❌ *6.28x slower*)    |
| **`legendre_for_qr`**    | `11.42 us` (✅ **1.00x**) | `34.46 us` (❌ *3.02x slower*)   | `35.16 us` (❌ *3.08x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.68 ns` (✅ **1.00x**)        | `4.83 ns` (✅ **1.03x slower**)    |
| **`from_little-endian_bits`** | `71.84 ns` (✅ **1.00x**)       | `128.18 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `71.92 ns` (✅ **1.00x**)       | `127.95 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.67 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.09x slower**)    |
| **`equality`**                | `5.32 ns` (✅ **1.00x**)        | `5.52 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `4.64 ns` (✅ **1.00x**)        | `4.73 ns` (✅ **1.02x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.97 ns` (✅ **1.00x**) | `92.40 ns` (❌ *2.15x slower*)    |
| **`into_bigint`** | `25.47 ns` (✅ **1.00x**) | `49.39 ns` (❌ *1.94x slower*)    |

### pairing_for_bls12_377optimized

|        | `g1_preparation_for_bls12_377optimized`          | `g2_preparation_for_bls12_377optimized`          | `miller_loop_for_bls12_377optimized`          | `final_exponentiation_for_bls12_377optimized`          | `full_pairing_for_bls12_377optimized`           |
|:-------|:-------------------------------------------------|:-------------------------------------------------|:----------------------------------------------|:-------------------------------------------------------|:----------------------------------------------- |
|        | `10.77 ns` (✅ **1.00x**)                         | `12.80 ns` (❌ *1.19x slower*)                    | `1.02 ms` (❌ *94699.93x slower*)              | `1.37 ms` (❌ *127126.48x slower*)                      | `2.42 ms` (❌ *224791.73x slower*)               |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

