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
|        | `212.73 us` (✅ **1.00x**)                 | `2.04 ms` (❌ *9.61x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.22 us` (✅ **1.00x**)          | `4.55 us` (❌ *3.72x slower*)     | `23.16 ns` (🚀 **52.78x faster**)  | `192.53 ns` (🚀 **6.35x faster**)  | `12.67 ns` (🚀 **96.52x faster**) | `8.69 ns` (🚀 **140.70x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.28 us` (✅ **1.00x**)          | `4.60 us` (❌ *3.58x slower*)     | `23.31 ns` (🚀 **55.03x faster**)  | `162.37 ns` (🚀 **7.90x faster**)  | `12.91 ns` (🚀 **99.38x faster**) | `8.79 ns` (🚀 **145.95x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `900.94 ns` (✅ **1.00x**)        | `3.28 us` (❌ *3.65x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `940.18 ns` (✅ **1.00x**)        | `3.32 us` (❌ *3.53x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `592.64 ns` (✅ **1.00x**)        | `2.24 us` (❌ *3.78x slower*)     | `12.31 ns` (🚀 **48.13x faster**)  | `71.99 ns` (🚀 **8.23x faster**)   | `7.13 ns` (🚀 **83.13x faster**)  | `5.86 ns` (🚀 **101.15x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `344.88 us` (✅ **1.00x**)        | `1.17 ms` (❌ *3.41x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.02 ns` (❌ *3.57x slower*)     | `101.88 ns` (❌ *16.51x slower*)   | `18.57 ns` (❌ *3.01x slower*)    | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `265.77 ns` (❌ *6.06x slower*)    | `7.13 us` (❌ *162.57x slower*)    | `76.72 ns` (❌ *1.75x slower*)    | `43.85 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `240.64 ns` (❌ *6.75x slower*)    | `5.04 us` (❌ *141.37x slower*)    | `66.78 ns` (❌ *1.87x slower*)    | `35.65 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.19 us` (❌ *2.15x slower*)     | `27.53 us` (❌ *3.90x slower*)     | `14.84 us` (❌ *2.10x slower*)    | `7.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `616.43 ns` (❌ *10.02x slower*)   | `14.60 us` (❌ *237.25x slower*)   | `117.73 ns` (❌ *1.91x slower*)   | `61.54 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `564.52 ns` (❌ *6.33x slower*)    | `14.49 us` (❌ *162.49x slower*)   | `163.30 ns` (❌ *1.83x slower*)   | `89.17 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.70 ns` (✅ **1.00x**)        | `10.38 ns` (❌ *1.19x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `154.40 ns` (✅ **1.00x**)        | `221.35 ns` (❌ *1.43x slower*)   | `30.82 ns` (🚀 **5.01x faster**)    | `55.50 ns` (🚀 **2.78x faster**)    | `108.38 ns` (✅ **1.42x faster**)    | `698.60 ns` (❌ *4.52x slower*)    |
| **`serialize_uncompressed`**             | `208.04 ns` (✅ **1.00x**)        | `342.47 ns` (❌ *1.65x slower*)   | `30.96 ns` (🚀 **6.72x faster**)    | `54.98 ns` (🚀 **3.78x faster**)    | `109.53 ns` (🚀 **1.90x faster**)    | `694.52 ns` (❌ *3.34x slower*)    |
| **`deserialize_compressed`**             | `318.63 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.32x slower*)     | `52.08 ns` (🚀 **6117.59x faster**) | `93.03 ns` (🚀 **3424.91x faster**) | `209.19 ns` (🚀 **1523.18x faster**) | `1.31 us` (🚀 **242.58x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.91 us` (✅ **1.00x**)         | `183.55 us` (❌ *2.70x slower*)   | `52.10 ns` (🚀 **1303.52x faster**) | `92.96 ns` (🚀 **730.56x faster**)  | `209.59 ns` (🚀 **324.02x faster**)  | `1.31 us` (🚀 **51.75x faster**)   |
| **`deserialize_uncompressed`**           | `250.91 us` (✅ **1.00x**)        | `873.53 us` (❌ *3.48x slower*)   | `52.14 ns` (🚀 **4811.90x faster**) | `93.02 ns` (🚀 **2697.28x faster**) | `209.52 ns` (🚀 **1197.53x faster**) | `1.31 us` (🚀 **191.30x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.99 ns` (✅ **1.00x**)        | `473.92 ns` (❌ *2.09x slower*)   | `52.04 ns` (🚀 **4.36x faster**)    | `92.94 ns` (🚀 **2.44x faster**)    | `209.31 ns` (✅ **1.08x faster**)    | `1.31 us` (❌ *5.78x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.40 s` (✅ **1.00x**)           | `8.32 s` (❌ *3.47x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.24 us` (✅ **1.00x**) | `67.36 us` (❌ *2.16x slower*)   | `182.56 us` (❌ *5.84x slower*)    |
| **`legendre_for_qr`**    | `10.94 us` (✅ **1.00x**) | `31.73 us` (❌ *2.90x slower*)   | `31.76 us` (❌ *2.90x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.83 ns` (✅ **1.00x**)       | `89.40 ns` (❌ *1.83x slower*)    |
| **`from_big-endian_bits`**    | `48.85 ns` (✅ **1.00x**)       | `89.35 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `5.01 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.02x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)        | `5.65 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.62 ns` (✅ **1.00x**) | `75.27 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `23.80 ns` (✅ **1.00x**) | `46.89 ns` (❌ *1.97x slower*)    |

### pairing_for_bls12_377optimized

|        | `g1_preparation_for_bls12_377optimized`          | `g2_preparation_for_bls12_377optimized`          | `miller_loop_for_bls12_377optimized`          | `final_exponentiation_for_bls12_377optimized`          | `full_pairing_for_bls12_377optimized`           |
|:-------|:-------------------------------------------------|:-------------------------------------------------|:----------------------------------------------|:-------------------------------------------------------|:----------------------------------------------- |
|        | `9.13 ns` (✅ **1.00x**)                          | `11.69 ns` (❌ *1.28x slower*)                    | `945.91 us` (❌ *103567.50x slower*)           | `1.28 ms` (❌ *139872.53x slower*)                      | `2.23 ms` (❌ *244538.28x slower*)               |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

