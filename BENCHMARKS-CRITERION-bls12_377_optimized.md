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
|        | `193.57 us` (✅ **1.00x**)                 | `1.87 ms` (❌ *9.67x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.13 us` (✅ **1.00x**)          | `4.27 us` (❌ *3.77x slower*)     | `28.07 ns` (🚀 **40.40x faster**)  | `179.55 ns` (🚀 **6.32x faster**)  | `19.52 ns` (🚀 **58.08x faster**) | `8.30 ns` (🚀 **136.56x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.17 us` (✅ **1.00x**)          | `4.32 us` (❌ *3.70x slower*)     | `27.28 ns` (🚀 **42.82x faster**)  | `169.16 ns` (🚀 **6.90x faster**)  | `15.04 ns` (🚀 **77.64x faster**) | `8.71 ns` (🚀 **134.05x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `832.15 ns` (✅ **1.00x**)        | `3.08 us` (❌ *3.70x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `856.70 ns` (✅ **1.00x**)        | `3.11 us` (❌ *3.63x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `569.49 ns` (✅ **1.00x**)        | `2.05 us` (❌ *3.60x slower*)     | `12.86 ns` (🚀 **44.30x faster**)  | `101.02 ns` (🚀 **5.64x faster**)  | `7.49 ns` (🚀 **75.99x faster**)  | `9.06 ns` (🚀 **62.84x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `299.06 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.60x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.89 ns` (❌ *3.84x slower*)     | `108.15 ns` (❌ *18.17x slower*)   | `17.10 ns` (❌ *2.87x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.21 ns` (❌ *7.17x slower*)    | `6.67 us` (❌ *179.09x slower*)    | `69.35 ns` (❌ *1.86x slower*)    | `37.25 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `249.65 ns` (❌ *7.82x slower*)    | `4.69 us` (❌ *147.01x slower*)    | `59.40 ns` (❌ *1.86x slower*)    | `31.91 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.77 us` (❌ *2.16x slower*)     | `25.05 us` (❌ *3.93x slower*)     | `13.43 us` (❌ *2.11x slower*)    | `6.38 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `569.93 ns` (❌ *10.75x slower*)   | `13.66 us` (❌ *257.55x slower*)   | `111.65 ns` (❌ *2.11x slower*)   | `53.04 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `551.80 ns` (❌ *6.91x slower*)    | `13.56 us` (❌ *169.92x slower*)   | `157.75 ns` (❌ *1.98x slower*)   | `79.82 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.62 ns` (❌ *1.36x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `144.15 ns` (✅ **1.00x**)        | `211.40 ns` (❌ *1.47x slower*)   | `28.07 ns` (🚀 **5.14x faster**)    | `50.29 ns` (🚀 **2.87x faster**)    | `99.88 ns` (✅ **1.44x faster**)     | `630.14 ns` (❌ *4.37x slower*)    |
| **`serialize_uncompressed`**             | `199.36 ns` (✅ **1.00x**)        | `319.48 ns` (❌ *1.60x slower*)   | `28.19 ns` (🚀 **7.07x faster**)    | `50.17 ns` (🚀 **3.97x faster**)    | `99.85 ns` (🚀 **2.00x faster**)     | `626.25 ns` (❌ *3.14x slower*)    |
| **`deserialize_compressed`**             | `281.87 us` (✅ **1.00x**)        | `973.07 us` (❌ *3.45x slower*)   | `47.28 ns` (🚀 **5961.48x faster**) | `93.81 ns` (🚀 **3004.78x faster**) | `208.18 ns` (🚀 **1353.93x faster**) | `1.26 us` (🚀 **224.53x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.67 us` (✅ **1.00x**)         | `174.41 us` (❌ *2.66x slower*)   | `47.29 ns` (🚀 **1388.67x faster**) | `94.01 ns` (🚀 **698.52x faster**)  | `208.21 ns` (🚀 **315.40x faster**)  | `1.25 us` (🚀 **52.48x faster**)   |
| **`deserialize_uncompressed`**           | `216.51 us` (✅ **1.00x**)        | `795.79 us` (❌ *3.68x slower*)   | `47.24 ns` (🚀 **4582.90x faster**) | `93.80 ns` (🚀 **2308.13x faster**) | `208.71 ns` (🚀 **1037.38x faster**) | `1.25 us` (🚀 **172.90x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `229.65 ns` (✅ **1.00x**)        | `469.53 ns` (❌ *2.04x slower*)   | `47.26 ns` (🚀 **4.86x faster**)    | `93.82 ns` (🚀 **2.45x faster**)    | `208.72 ns` (✅ **1.10x faster**)    | `1.25 us` (❌ *5.45x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.27 s` (✅ **1.00x**)           | `7.95 s` (❌ *3.50x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.63 us` (✅ **1.00x**) | `65.13 us` (❌ *2.36x slower*)   | `173.61 us` (❌ *6.28x slower*)    |
| **`legendre_for_qr`**    | `9.53 us` (✅ **1.00x**)  | `29.09 us` (❌ *3.05x slower*)   | `29.32 us` (❌ *3.08x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.54 ns` (✅ **1.00x**)       | `109.02 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `60.52 ns` (✅ **1.00x**)       | `109.09 ns` (❌ *1.80x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.32 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.87 ns` (✅ **1.00x**) | `79.40 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `21.77 ns` (✅ **1.00x**) | `41.49 ns` (❌ *1.91x slower*)    |

### pairing_for_bls12_377optimized

|        | `g1_preparation_for_bls12_377optimized`          | `g2_preparation_for_bls12_377optimized`          | `miller_loop_for_bls12_377optimized`          | `final_exponentiation_for_bls12_377optimized`          | `full_pairing_for_bls12_377optimized`           |
|:-------|:-------------------------------------------------|:-------------------------------------------------|:----------------------------------------------|:-------------------------------------------------------|:----------------------------------------------- |
|        | `9.06 ns` (✅ **1.00x**)                          | `11.04 ns` (❌ *1.22x slower*)                    | `866.22 us` (❌ *95626.39x slower*)            | `1.17 ms` (❌ *129701.81x slower*)                      | `2.06 ms` (❌ *227751.38x slower*)               |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

