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

## Benchmark Results

### sample_bls12_377

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `194.40 us` (✅ **1.00x**)        | `2.03 ms` (❌ *10.43x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.24 us` (✅ **1.00x**)   | `4.78 us` (❌ *3.84x slower*)   | `23.10 ns` (🚀 **53.89x faster**) | `181.58 ns` (🚀 **6.86x faster**)  | `12.51 ns` (🚀 **99.52x faster**)  | `8.71 ns` (🚀 **142.99x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.29 us` (✅ **1.00x**)   | `4.85 us` (❌ *3.75x slower*)   | `23.18 ns` (🚀 **55.69x faster**) | `164.01 ns` (🚀 **7.87x faster**)  | `12.81 ns` (🚀 **100.76x faster**) | `8.80 ns` (🚀 **146.67x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `891.88 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.86x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `928.45 ns` (✅ **1.00x**) | `3.48 us` (❌ *3.75x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `597.19 ns` (✅ **1.00x**) | `2.26 us` (❌ *3.78x slower*)   | `12.27 ns` (🚀 **48.65x faster**) | `71.93 ns` (🚀 **8.30x faster**)   | `7.12 ns` (🚀 **83.93x faster**)   | `5.92 ns` (🚀 **100.85x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `322.32 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.60x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.94 ns` (❌ *3.72x slower*)    | `94.69 ns` (❌ *15.34x slower*)    | `18.58 ns` (❌ *3.01x slower*)     | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `270.87 ns` (❌ *6.08x slower*)   | `7.11 us` (❌ *159.59x slower*)    | `76.48 ns` (❌ *1.72x slower*)     | `44.58 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `241.63 ns` (❌ *6.70x slower*)   | `5.01 us` (❌ *139.05x slower*)    | `66.22 ns` (❌ *1.84x slower*)     | `36.04 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.13 us` (❌ *2.15x slower*)    | `27.46 us` (❌ *3.90x slower*)     | `14.81 us` (❌ *2.10x slower*)     | `7.04 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `588.04 ns` (❌ *9.58x slower*)   | `14.56 us` (❌ *237.37x slower*)   | `118.47 ns` (❌ *1.93x slower*)    | `61.35 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `577.53 ns` (❌ *6.50x slower*)   | `14.49 us` (❌ *163.07x slower*)   | `162.91 ns` (❌ *1.83x slower*)    | `88.89 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.68 ns` (✅ **1.00x**) | `10.42 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.78 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `155.02 ns` (✅ **1.00x**) | `219.94 ns` (❌ *1.42x slower*)   | `30.53 ns` (🚀 **5.08x faster**)    | `56.85 ns` (🚀 **2.73x faster**)    | `109.19 ns` (✅ **1.42x faster**)    | `694.78 ns` (❌ *4.48x slower*)    |
| **`serialize_uncompressed`**             | `210.96 ns` (✅ **1.00x**) | `335.11 ns` (❌ *1.59x slower*)   | `31.35 ns` (🚀 **6.73x faster**)    | `55.75 ns` (🚀 **3.78x faster**)    | `109.17 ns` (🚀 **1.93x faster**)    | `695.02 ns` (❌ *3.29x slower*)    |
| **`deserialize_compressed`**             | `314.52 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.36x slower*)     | `52.61 ns` (🚀 **5978.28x faster**) | `93.03 ns` (🚀 **3380.70x faster**) | `210.55 ns` (🚀 **1493.81x faster**) | `1.27 us` (🚀 **247.32x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.83 us` (✅ **1.00x**)  | `183.37 us` (❌ *2.70x slower*)   | `52.72 ns` (🚀 **1286.71x faster**) | `93.02 ns` (🚀 **729.23x faster**)  | `210.29 ns` (🚀 **322.57x faster**)  | `1.27 us` (🚀 **53.48x faster**)   |
| **`deserialize_uncompressed`**           | `246.94 us` (✅ **1.00x**) | `871.20 us` (❌ *3.53x slower*)   | `52.53 ns` (🚀 **4701.33x faster**) | `92.94 ns` (🚀 **2656.94x faster**) | `210.46 ns` (🚀 **1173.34x faster**) | `1.27 us` (🚀 **195.16x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.65 ns` (✅ **1.00x**) | `466.83 ns` (❌ *2.09x slower*)   | `52.53 ns` (🚀 **4.26x faster**)    | `92.95 ns` (🚀 **2.41x faster**)    | `210.44 ns` (✅ **1.06x faster**)    | `1.26 us` (❌ *5.66x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.33 s` (✅ **1.00x**)  | `8.30 s` (❌ *3.56x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.13 us` (✅ **1.00x**) | `67.23 us` (❌ *2.16x slower*)   | `182.34 us` (❌ *5.86x slower*)    |
| **`legendre_for_qr`**    | `10.93 us` (✅ **1.00x**) | `32.47 us` (❌ *2.97x slower*)   | `31.97 us` (❌ *2.92x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.86 ns` (✅ **1.00x**) | `83.65 ns` (❌ *1.71x slower*)    |
| **`from_big-endian_bits`**    | `48.88 ns` (✅ **1.00x**) | `83.66 ns` (❌ *1.71x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `5.20 ns` (✅ **1.07x slower**)   |
| **`equality`**                | `5.35 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.07 ns` (✅ **1.00x**) | `75.18 ns` (❌ *1.83x slower*)    |
| **`into_bigint`** | `23.81 ns` (✅ **1.00x**) | `47.19 ns` (❌ *1.98x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

