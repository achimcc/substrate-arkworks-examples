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
|        | `194.62 us` (✅ **1.00x**)        | `2.03 ms` (❌ *10.41x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `4.79 us` (❌ *3.85x slower*)   | `23.39 ns` (🚀 **53.25x faster**) | `183.20 ns` (🚀 **6.80x faster**)  | `12.50 ns` (🚀 **99.63x faster**)  | `8.71 ns` (🚀 **143.03x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.29 us` (✅ **1.00x**)   | `4.84 us` (❌ *3.75x slower*)   | `23.36 ns` (🚀 **55.31x faster**) | `162.46 ns` (🚀 **7.95x faster**)  | `12.74 ns` (🚀 **101.41x faster**) | `8.80 ns` (🚀 **146.83x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `893.64 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.85x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `922.30 ns` (✅ **1.00x**) | `3.48 us` (❌ *3.77x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `597.40 ns` (✅ **1.00x**) | `2.25 us` (❌ *3.77x slower*)   | `12.35 ns` (🚀 **48.36x faster**) | `74.16 ns` (🚀 **8.06x faster**)   | `7.13 ns` (🚀 **83.84x faster**)   | `5.92 ns` (🚀 **100.87x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `322.38 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.60x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.02 ns` (❌ *3.57x slower*)    | `95.19 ns` (❌ *15.43x slower*)    | `18.16 ns` (❌ *2.94x slower*)     | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `271.60 ns` (❌ *6.14x slower*)   | `7.13 us` (❌ *161.00x slower*)    | `76.51 ns` (❌ *1.73x slower*)     | `44.25 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `242.38 ns` (❌ *6.72x slower*)   | `5.02 us` (❌ *138.99x slower*)    | `66.25 ns` (❌ *1.84x slower*)     | `36.09 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.20 us` (❌ *2.16x slower*)    | `27.44 us` (❌ *3.90x slower*)     | `14.81 us` (❌ *2.10x slower*)     | `7.04 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `588.19 ns` (❌ *9.58x slower*)   | `14.57 us` (❌ *237.35x slower*)   | `118.56 ns` (❌ *1.93x slower*)    | `61.40 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `577.06 ns` (❌ *6.49x slower*)   | `14.45 us` (❌ *162.53x slower*)   | `162.82 ns` (❌ *1.83x slower*)    | `88.94 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.67 ns` (✅ **1.00x**) | `10.42 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.78 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.53 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `154.38 ns` (✅ **1.00x**) | `219.71 ns` (❌ *1.42x slower*)   | `30.62 ns` (🚀 **5.04x faster**)    | `57.10 ns` (🚀 **2.70x faster**)    | `109.00 ns` (✅ **1.42x faster**)    | `694.72 ns` (❌ *4.50x slower*)    |
| **`serialize_uncompressed`**             | `210.40 ns` (✅ **1.00x**) | `334.56 ns` (❌ *1.59x slower*)   | `31.19 ns` (🚀 **6.75x faster**)    | `55.62 ns` (🚀 **3.78x faster**)    | `109.06 ns` (🚀 **1.93x faster**)    | `695.09 ns` (❌ *3.30x slower*)    |
| **`deserialize_compressed`**             | `314.74 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.36x slower*)     | `52.75 ns` (🚀 **5966.90x faster**) | `93.04 ns` (🚀 **3382.83x faster**) | `210.45 ns` (🚀 **1495.58x faster**) | `1.27 us` (🚀 **248.37x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.90 us` (✅ **1.00x**)  | `183.58 us` (❌ *2.70x slower*)   | `52.90 ns` (🚀 **1283.48x faster**) | `93.34 ns` (🚀 **727.46x faster**)  | `210.29 ns` (🚀 **322.89x faster**)  | `1.27 us` (🚀 **53.51x faster**)   |
| **`deserialize_uncompressed`**           | `246.96 us` (✅ **1.00x**) | `871.94 us` (❌ *3.53x slower*)   | `52.68 ns` (🚀 **4688.20x faster**) | `92.95 ns` (🚀 **2656.85x faster**) | `210.44 ns` (🚀 **1173.55x faster**) | `1.26 us` (🚀 **195.60x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `224.92 ns` (✅ **1.00x**) | `465.24 ns` (❌ *2.07x slower*)   | `52.83 ns` (🚀 **4.26x faster**)    | `93.07 ns` (🚀 **2.42x faster**)    | `210.26 ns` (✅ **1.07x faster**)    | `1.27 us` (❌ *5.65x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.33 s` (✅ **1.00x**)  | `8.32 s` (❌ *3.57x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.18 us` (✅ **1.00x**) | `67.73 us` (❌ *2.17x slower*)   | `182.58 us` (❌ *5.86x slower*)    |
| **`legendre_for_qr`**    | `10.94 us` (✅ **1.00x**) | `32.47 us` (❌ *2.97x slower*)   | `31.98 us` (❌ *2.92x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.98 ns` (✅ **1.00x**) | `83.74 ns` (❌ *1.71x slower*)    |
| **`from_big-endian_bits`**    | `48.98 ns` (✅ **1.00x**) | `83.67 ns` (❌ *1.71x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `5.20 ns` (✅ **1.07x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.91 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.08 ns` (✅ **1.00x**) | `75.50 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `23.81 ns` (✅ **1.00x**) | `47.10 ns` (❌ *1.98x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

