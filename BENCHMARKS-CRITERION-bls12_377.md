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
|        | `194.73 us` (✅ **1.00x**)        | `2.04 ms` (❌ *10.48x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.30 us` (✅ **1.00x**)   | `4.79 us` (❌ *3.68x slower*)   | `23.52 ns` (🚀 **55.32x faster**) | `198.93 ns` (🚀 **6.54x faster**)  | `12.65 ns` (🚀 **102.91x faster**) | `8.74 ns` (🚀 **148.88x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.33 us` (✅ **1.00x**)   | `4.84 us` (❌ *3.64x slower*)   | `23.50 ns` (🚀 **56.49x faster**) | `163.38 ns` (🚀 **8.13x faster**)  | `13.15 ns` (🚀 **100.96x faster**) | `8.79 ns` (🚀 **151.09x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `918.85 ns` (✅ **1.00x**) | `3.42 us` (❌ *3.73x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `952.07 ns` (✅ **1.00x**) | `3.45 us` (❌ *3.63x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `609.42 ns` (✅ **1.00x**) | `2.28 us` (❌ *3.74x slower*)   | `12.58 ns` (🚀 **48.44x faster**) | `73.96 ns` (🚀 **8.24x faster**)   | `7.12 ns` (🚀 **85.65x faster**)   | `5.85 ns` (🚀 **104.11x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `326.64 us` (✅ **1.00x**) | `1.17 ms` (❌ *3.57x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.14 ns` (❌ *3.60x slower*)    | `94.57 ns` (❌ *15.39x slower*)    | `18.44 ns` (❌ *3.00x slower*)     | `6.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `266.70 ns` (❌ *6.23x slower*)   | `7.12 us` (❌ *166.29x slower*)    | `78.06 ns` (❌ *1.82x slower*)     | `42.83 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `235.92 ns` (❌ *6.60x slower*)   | `5.04 us` (❌ *141.04x slower*)    | `66.65 ns` (❌ *1.87x slower*)     | `35.72 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.19 us` (❌ *2.13x slower*)    | `27.63 us` (❌ *3.87x slower*)     | `14.82 us` (❌ *2.07x slower*)     | `7.15 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `578.13 ns` (❌ *9.24x slower*)   | `14.56 us` (❌ *232.65x slower*)   | `117.36 ns` (❌ *1.87x slower*)    | `62.59 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `563.53 ns` (❌ *6.32x slower*)   | `14.52 us` (❌ *162.98x slower*)   | `163.84 ns` (❌ *1.84x slower*)    | `89.11 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.60 ns` (✅ **1.00x**) | `8.69 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.69 ns` (✅ **1.00x**) | `10.30 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `155.91 ns` (✅ **1.00x**) | `220.01 ns` (❌ *1.41x slower*)   | `32.36 ns` (🚀 **4.82x faster**)    | `55.09 ns` (🚀 **2.83x faster**)    | `109.19 ns` (✅ **1.43x faster**)    | `841.70 ns` (❌ *5.40x slower*)    |
| **`serialize_uncompressed`**             | `211.34 ns` (✅ **1.00x**) | `332.22 ns` (❌ *1.57x slower*)   | `31.12 ns` (🚀 **6.79x faster**)    | `55.17 ns` (🚀 **3.83x faster**)    | `109.08 ns` (🚀 **1.94x faster**)    | `814.26 ns` (❌ *3.85x slower*)    |
| **`deserialize_compressed`**             | `316.36 us` (✅ **1.00x**) | `1.07 ms` (❌ *3.37x slower*)     | `51.42 ns` (🚀 **6152.43x faster**) | `94.31 ns` (🚀 **3354.40x faster**) | `212.73 ns` (🚀 **1487.13x faster**) | `1.30 us` (🚀 **243.44x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.92 us` (✅ **1.00x**)  | `183.51 us` (❌ *2.70x slower*)   | `51.44 ns` (🚀 **1320.54x faster**) | `94.09 ns` (🚀 **721.88x faster**)  | `212.65 ns` (🚀 **319.42x faster**)  | `1.30 us` (🚀 **52.28x faster**)   |
| **`deserialize_uncompressed`**           | `248.47 us` (✅ **1.00x**) | `879.40 us` (❌ *3.54x slower*)   | `51.26 ns` (🚀 **4847.18x faster**) | `94.06 ns` (🚀 **2641.61x faster**) | `212.52 ns` (🚀 **1169.15x faster**) | `1.30 us` (🚀 **191.41x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.16 ns` (✅ **1.00x**) | `467.85 ns` (❌ *2.10x slower*)   | `51.31 ns` (🚀 **4.35x faster**)    | `94.06 ns` (🚀 **2.37x faster**)    | `212.54 ns` (✅ **1.05x faster**)    | `1.29 us` (❌ *5.80x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.41 s` (✅ **1.00x**)  | `8.33 s` (❌ *3.46x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.15 us` (✅ **1.00x**) | `67.46 us` (❌ *2.17x slower*)   | `182.54 us` (❌ *5.86x slower*)    |
| **`legendre_for_qr`**    | `10.89 us` (✅ **1.00x**) | `31.64 us` (❌ *2.91x slower*)   | `31.80 us` (❌ *2.92x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.21 ns` (✅ **1.00x**) | `86.19 ns` (❌ *1.75x slower*)    |
| **`from_big-endian_bits`**    | `49.02 ns` (✅ **1.00x**) | `85.78 ns` (❌ *1.75x slower*)    |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)  | `5.08 ns` (✅ **1.03x slower**)   |
| **`equality`**                | `5.35 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.78 ns` (✅ **1.00x**) | `75.31 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.93 ns` (✅ **1.00x**) | `47.07 ns` (❌ *2.05x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

