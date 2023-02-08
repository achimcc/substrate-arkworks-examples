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
|        | `194.74 us` (✅ **1.00x**)        | `2.03 ms` (❌ *10.40x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.24 us` (✅ **1.00x**)   | `4.79 us` (❌ *3.86x slower*)   | `23.16 ns` (🚀 **53.67x faster**) | `179.05 ns` (🚀 **6.94x faster**)  | `12.52 ns` (🚀 **99.25x faster**)  | `8.70 ns` (🚀 **142.78x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.29 us` (✅ **1.00x**)   | `4.85 us` (❌ *3.75x slower*)   | `23.19 ns` (🚀 **55.74x faster**) | `159.06 ns` (🚀 **8.13x faster**)  | `12.75 ns` (🚀 **101.43x faster**) | `8.81 ns` (🚀 **146.81x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `892.26 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.86x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `920.06 ns` (✅ **1.00x**) | `3.48 us` (❌ *3.78x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `598.36 ns` (✅ **1.00x**) | `2.25 us` (❌ *3.77x slower*)   | `12.27 ns` (🚀 **48.76x faster**) | `70.94 ns` (🚀 **8.43x faster**)   | `7.13 ns` (🚀 **83.93x faster**)   | `5.92 ns` (🚀 **101.00x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `322.39 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.60x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.55 ns` (❌ *3.65x slower*)    | `102.88 ns` (❌ *16.66x slower*)   | `18.15 ns` (❌ *2.94x slower*)     | `6.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `271.00 ns` (❌ *6.09x slower*)   | `7.12 us` (❌ *160.02x slower*)    | `76.29 ns` (❌ *1.72x slower*)     | `44.46 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `242.96 ns` (❌ *6.74x slower*)   | `5.02 us` (❌ *139.36x slower*)    | `66.24 ns` (❌ *1.84x slower*)     | `36.03 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.18 us` (❌ *2.15x slower*)    | `27.49 us` (❌ *3.90x slower*)     | `14.84 us` (❌ *2.10x slower*)     | `7.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `588.34 ns` (❌ *9.58x slower*)   | `14.57 us` (❌ *237.38x slower*)   | `118.42 ns` (❌ *1.93x slower*)    | `61.40 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `577.56 ns` (❌ *6.49x slower*)   | `14.47 us` (❌ *162.64x slower*)   | `163.06 ns` (❌ *1.83x slower*)    | `88.96 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.71 ns` (✅ **1.00x**) | `10.43 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.79 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**) | `4.54 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `154.71 ns` (✅ **1.00x**) | `219.81 ns` (❌ *1.42x slower*)   | `30.68 ns` (🚀 **5.04x faster**)    | `57.07 ns` (🚀 **2.71x faster**)    | `109.43 ns` (✅ **1.41x faster**)    | `695.36 ns` (❌ *4.49x slower*)    |
| **`serialize_uncompressed`**             | `210.45 ns` (✅ **1.00x**) | `334.40 ns` (❌ *1.59x slower*)   | `31.23 ns` (🚀 **6.74x faster**)    | `55.77 ns` (🚀 **3.77x faster**)    | `109.29 ns` (🚀 **1.93x faster**)    | `695.40 ns` (❌ *3.30x slower*)    |
| **`deserialize_compressed`**             | `314.66 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.36x slower*)     | `52.58 ns` (🚀 **5984.44x faster**) | `93.21 ns` (🚀 **3375.79x faster**) | `209.84 ns` (🚀 **1499.54x faster**) | `1.28 us` (🚀 **245.29x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.88 us` (✅ **1.00x**)  | `183.43 us` (❌ *2.70x slower*)   | `52.72 ns` (🚀 **1287.59x faster**) | `93.53 ns` (🚀 **725.77x faster**)  | `210.08 ns` (🚀 **323.10x faster**)  | `1.28 us` (🚀 **53.07x faster**)   |
| **`deserialize_uncompressed`**           | `246.97 us` (✅ **1.00x**) | `871.38 us` (❌ *3.53x slower*)   | `52.41 ns` (🚀 **4712.41x faster**) | `93.41 ns` (🚀 **2643.91x faster**) | `209.34 ns` (🚀 **1179.74x faster**) | `1.28 us` (🚀 **192.99x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `224.35 ns` (✅ **1.00x**) | `466.14 ns` (❌ *2.08x slower*)   | `52.40 ns` (🚀 **4.28x faster**)    | `93.31 ns` (🚀 **2.40x faster**)    | `209.42 ns` (✅ **1.07x faster**)    | `1.28 us` (❌ *5.72x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.33 s` (✅ **1.00x**)  | `8.30 s` (❌ *3.56x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.14 us` (✅ **1.00x**) | `67.25 us` (❌ *2.16x slower*)   | `182.40 us` (❌ *5.86x slower*)    |
| **`legendre_for_qr`**    | `10.94 us` (✅ **1.00x**) | `32.48 us` (❌ *2.97x slower*)   | `31.97 us` (❌ *2.92x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.00 ns` (✅ **1.00x**) | `83.64 ns` (❌ *1.71x slower*)    |
| **`from_big-endian_bits`**    | `48.84 ns` (✅ **1.00x**) | `83.85 ns` (❌ *1.72x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `5.20 ns` (✅ **1.07x slower**)   |
| **`equality`**                | `5.35 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.07 ns` (✅ **1.00x**) | `75.23 ns` (❌ *1.83x slower*)    |
| **`into_bigint`** | `23.82 ns` (✅ **1.00x**) | `47.18 ns` (❌ *1.98x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

