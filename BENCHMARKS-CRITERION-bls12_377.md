# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_377](#sample_bls12_377)
    - [arithmetic_for_bls12_377](#arithmetic_for_bls12_377)
    - [serialization_for_bls12_377](#serialization_for_bls12_377)
    - [msm_for_bls12_377](#msm_for_bls12_377)
    - [squareroot_for_bls12_377](#squareroot_for_bls12_377)
    - [bitwise_operations_for_bls12_377](#bitwise_operations_for_bls12_377)
    - [conversions_for_bls12_377](#conversions_for_bls12_377)
    - [pairing_for_bls12_377](#pairing_for_bls12_377)

## Benchmark Results

### sample_bls12_377

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `194.49 us` (✅ **1.00x**)        | `2.02 ms` (❌ *10.40x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.23 us` (✅ **1.00x**)   | `4.77 us` (❌ *3.88x slower*)   | `23.34 ns` (🚀 **52.71x faster**) | `179.04 ns` (🚀 **6.87x faster**)  | `12.72 ns` (🚀 **96.72x faster**) | `8.72 ns` (🚀 **141.15x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.27 us` (✅ **1.00x**)   | `4.82 us` (❌ *3.80x slower*)   | `23.38 ns` (🚀 **54.29x faster**) | `158.60 ns` (🚀 **8.00x faster**)  | `12.95 ns` (🚀 **98.00x faster**) | `8.80 ns` (🚀 **144.24x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `890.35 ns` (✅ **1.00x**) | `3.42 us` (❌ *3.84x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `934.34 ns` (✅ **1.00x**) | `3.45 us` (❌ *3.69x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `584.30 ns` (✅ **1.00x**) | `2.26 us` (❌ *3.86x slower*)   | `12.43 ns` (🚀 **47.01x faster**) | `71.84 ns` (🚀 **8.13x faster**)   | `7.15 ns` (🚀 **81.75x faster**)  | `5.92 ns` (🚀 **98.73x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `319.97 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.61x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.14 ns` (❌ *3.59x slower*)    | `100.32 ns` (❌ *16.24x slower*)   | `18.25 ns` (❌ *2.95x slower*)    | `6.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `271.03 ns` (❌ *6.34x slower*)   | `7.11 us` (❌ *166.37x slower*)    | `76.53 ns` (❌ *1.79x slower*)    | `42.74 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `244.49 ns` (❌ *6.89x slower*)   | `5.02 us` (❌ *141.61x slower*)    | `66.19 ns` (❌ *1.87x slower*)    | `35.48 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.15 us` (❌ *2.15x slower*)    | `27.47 us` (❌ *3.89x slower*)     | `14.84 us` (❌ *2.10x slower*)    | `7.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `589.61 ns` (❌ *9.60x slower*)   | `14.55 us` (❌ *236.83x slower*)   | `117.90 ns` (❌ *1.92x slower*)   | `61.44 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `578.50 ns` (❌ *6.48x slower*)   | `14.44 us` (❌ *161.78x slower*)   | `163.59 ns` (❌ *1.83x slower*)   | `89.27 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.72 ns` (✅ **1.00x**) | `10.31 ns` (❌ *1.18x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.78 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `158.62 ns` (✅ **1.00x**) | `222.25 ns` (❌ *1.40x slower*)   | `31.60 ns` (🚀 **5.02x faster**)    | `58.44 ns` (🚀 **2.71x faster**)    | `110.54 ns` (✅ **1.43x faster**)    | `705.31 ns` (❌ *4.45x slower*)    |
| **`serialize_uncompressed`**             | `214.98 ns` (✅ **1.00x**) | `335.10 ns` (❌ *1.56x slower*)   | `31.39 ns` (🚀 **6.85x faster**)    | `56.26 ns` (🚀 **3.82x faster**)    | `110.61 ns` (🚀 **1.94x faster**)    | `705.06 ns` (❌ *3.28x slower*)    |
| **`deserialize_compressed`**             | `314.41 us` (✅ **1.00x**) | `1.05 ms` (❌ *3.35x slower*)     | `52.64 ns` (🚀 **5973.03x faster**) | `92.08 ns` (🚀 **3414.53x faster**) | `220.24 ns` (🚀 **1427.61x faster**) | `1.28 us` (🚀 **244.86x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.04 us` (✅ **1.00x**)  | `183.31 us` (❌ *2.69x slower*)   | `52.62 ns` (🚀 **1293.16x faster**) | `91.94 ns` (🚀 **740.01x faster**)  | `220.27 ns` (🚀 **308.90x faster**)  | `1.29 us` (🚀 **52.94x faster**)   |
| **`deserialize_uncompressed`**           | `246.59 us` (✅ **1.00x**) | `869.70 us` (❌ *3.53x slower*)   | `52.39 ns` (🚀 **4706.52x faster**) | `91.90 ns` (🚀 **2683.22x faster**) | `220.68 ns` (🚀 **1117.39x faster**) | `1.28 us` (🚀 **191.91x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.09 ns` (✅ **1.00x**) | `475.78 ns` (❌ *2.10x slower*)   | `52.40 ns` (🚀 **4.31x faster**)    | `92.43 ns` (🚀 **2.45x faster**)    | `220.24 ns` (✅ **1.03x faster**)    | `1.28 us` (❌ *5.67x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.34 s` (✅ **1.00x**)  | `8.26 s` (❌ *3.53x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.08 us` (✅ **1.00x**) | `67.58 us` (❌ *2.17x slower*)   | `182.45 us` (❌ *5.87x slower*)    |
| **`legendre_for_qr`**    | `10.92 us` (✅ **1.00x**) | `31.36 us` (❌ *2.87x slower*)   | `31.41 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `50.05 ns` (✅ **1.00x**) | `84.75 ns` (❌ *1.69x slower*)    |
| **`from_big-endian_bits`**    | `49.93 ns` (✅ **1.00x**) | `85.26 ns` (❌ *1.71x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.17 ns` (✅ **1.00x**) | `74.78 ns` (❌ *1.82x slower*)    |
| **`into_bigint`** | `23.79 ns` (✅ **1.00x**) | `46.92 ns` (❌ *1.97x slower*)    |

### pairing_for_bls12_377

|        | `g1_preparation_for_bls12_377`          | `g2_preparation_for_bls12_377`          | `miller_loop_for_bls12_377`          | `final_exponentiation_for_bls12_377`          | `full_pairing_for_bls12_377`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.18 ns` (✅ **1.00x**)                 | `256.25 us` (❌ *27911.94x slower*)      | `669.83 us` (❌ *72960.31x slower*)   | `1.27 ms` (❌ *138201.96x slower*)             | `2.22 ms` (❌ *242035.98x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

