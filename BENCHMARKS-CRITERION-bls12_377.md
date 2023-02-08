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
|        | `194.42 us` (✅ **1.00x**)        | `2.03 ms` (❌ *10.42x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.27 us` (✅ **1.00x**)   | `4.78 us` (❌ *3.77x slower*)   | `23.13 ns` (🚀 **54.78x faster**) | `183.99 ns` (🚀 **6.89x faster**)  | `12.48 ns` (🚀 **101.54x faster**) | `8.70 ns` (🚀 **145.57x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.32 us` (✅ **1.00x**)   | `4.85 us` (❌ *3.67x slower*)   | `23.21 ns` (🚀 **56.86x faster**) | `160.37 ns` (🚀 **8.23x faster**)  | `12.75 ns` (🚀 **103.52x faster**) | `8.81 ns` (🚀 **149.80x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `891.68 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.86x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `920.28 ns` (✅ **1.00x**) | `3.48 us` (❌ *3.78x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `597.40 ns` (✅ **1.00x**) | `2.26 us` (❌ *3.78x slower*)   | `12.32 ns` (🚀 **48.50x faster**) | `68.76 ns` (🚀 **8.69x faster**)   | `7.12 ns` (🚀 **83.90x faster**)   | `5.91 ns` (🚀 **101.03x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `322.40 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.60x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `23.16 ns` (❌ *3.75x slower*)    | `94.87 ns` (❌ *15.37x slower*)    | `18.56 ns` (❌ *3.01x slower*)     | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `270.82 ns` (❌ *6.06x slower*)   | `7.13 us` (❌ *159.48x slower*)    | `76.36 ns` (❌ *1.71x slower*)     | `44.71 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `244.17 ns` (❌ *6.77x slower*)   | `5.02 us` (❌ *139.12x slower*)    | `66.24 ns` (❌ *1.84x slower*)     | `36.07 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.17 us` (❌ *2.15x slower*)    | `27.50 us` (❌ *3.90x slower*)     | `14.83 us` (❌ *2.11x slower*)     | `7.04 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `588.31 ns` (❌ *9.58x slower*)   | `14.61 us` (❌ *237.89x slower*)   | `118.56 ns` (❌ *1.93x slower*)    | `61.39 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `577.10 ns` (❌ *6.49x slower*)   | `14.48 us` (❌ *162.79x slower*)   | `162.96 ns` (❌ *1.83x slower*)    | `88.97 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.67 ns` (✅ **1.00x**) | `10.43 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.79 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.54 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `155.46 ns` (✅ **1.00x**) | `219.63 ns` (❌ *1.41x slower*)   | `31.67 ns` (🚀 **4.91x faster**)    | `56.89 ns` (🚀 **2.73x faster**)    | `109.18 ns` (✅ **1.42x faster**)    | `695.31 ns` (❌ *4.47x slower*)    |
| **`serialize_uncompressed`**             | `213.43 ns` (✅ **1.00x**) | `334.75 ns` (❌ *1.57x slower*)   | `30.50 ns` (🚀 **7.00x faster**)    | `55.82 ns` (🚀 **3.82x faster**)    | `109.00 ns` (🚀 **1.96x faster**)    | `695.22 ns` (❌ *3.26x slower*)    |
| **`deserialize_compressed`**             | `314.55 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.36x slower*)     | `52.69 ns` (🚀 **5970.39x faster**) | `93.17 ns` (🚀 **3376.25x faster**) | `224.81 ns` (🚀 **1399.20x faster**) | `1.28 us` (🚀 **245.32x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.82 us` (✅ **1.00x**)  | `183.44 us` (❌ *2.70x slower*)   | `52.73 ns` (🚀 **1286.21x faster**) | `93.12 ns` (🚀 **728.31x faster**)  | `224.77 ns` (🚀 **301.72x faster**)  | `1.29 us` (🚀 **52.51x faster**)   |
| **`deserialize_uncompressed`**           | `246.92 us` (✅ **1.00x**) | `871.32 us` (❌ *3.53x slower*)   | `52.56 ns` (🚀 **4698.16x faster**) | `93.04 ns` (🚀 **2653.93x faster**) | `224.31 ns` (🚀 **1100.83x faster**) | `1.29 us` (🚀 **191.84x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.99 ns` (✅ **1.00x**) | `466.44 ns` (❌ *2.08x slower*)   | `52.44 ns` (🚀 **4.27x faster**)    | `93.32 ns` (🚀 **2.40x faster**)    | `223.84 ns` (✅ **1.00x faster**)    | `1.28 us` (❌ *5.72x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.33 s` (✅ **1.00x**)  | `8.29 s` (❌ *3.56x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.16 us` (✅ **1.00x**) | `67.24 us` (❌ *2.16x slower*)   | `182.38 us` (❌ *5.85x slower*)    |
| **`legendre_for_qr`**    | `10.94 us` (✅ **1.00x**) | `32.47 us` (❌ *2.97x slower*)   | `31.97 us` (❌ *2.92x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.90 ns` (✅ **1.00x**) | `83.77 ns` (❌ *1.71x slower*)    |
| **`from_big-endian_bits`**    | `48.86 ns` (✅ **1.00x**) | `83.68 ns` (❌ *1.71x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `5.20 ns` (✅ **1.07x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.09 ns` (✅ **1.00x**) | `75.33 ns` (❌ *1.83x slower*)    |
| **`into_bigint`** | `23.82 ns` (✅ **1.00x**) | `47.17 ns` (❌ *1.98x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

