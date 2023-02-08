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
|        | `178.30 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.40x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.14 us` (✅ **1.00x**)   | `4.45 us` (❌ *3.90x slower*)   | `27.67 ns` (🚀 **41.19x faster**)  | `176.84 ns` (🚀 **6.45x faster**)  | `19.04 ns` (🚀 **59.85x faster**) | `8.30 ns` (🚀 **137.35x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.18 us` (✅ **1.00x**)   | `4.49 us` (❌ *3.82x slower*)   | `27.12 ns` (🚀 **43.35x faster**)  | `170.91 ns` (🚀 **6.88x faster**)  | `18.41 ns` (🚀 **63.88x faster**) | `8.61 ns` (🚀 **136.62x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `831.74 ns` (✅ **1.00x**) | `3.16 us` (❌ *3.80x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `857.81 ns` (✅ **1.00x**) | `3.21 us` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `568.85 ns` (✅ **1.00x**) | `2.08 us` (❌ *3.66x slower*)   | `12.87 ns` (🚀 **44.20x faster**)  | `99.76 ns` (🚀 **5.70x faster**)   | `11.07 ns` (🚀 **51.36x faster**) | `5.31 ns` (🚀 **107.22x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.25 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `23.45 ns` (❌ *3.94x slower*)     | `104.54 ns` (❌ *17.59x slower*)   | `17.10 ns` (❌ *2.88x slower*)    | `5.94 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `265.39 ns` (❌ *7.11x slower*)    | `6.66 us` (❌ *178.36x slower*)    | `69.39 ns` (❌ *1.86x slower*)    | `37.35 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `247.36 ns` (❌ *7.76x slower*)    | `4.69 us` (❌ *147.20x slower*)    | `59.13 ns` (❌ *1.86x slower*)    | `31.87 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.75 us` (❌ *2.15x slower*)     | `25.05 us` (❌ *3.91x slower*)     | `13.43 us` (❌ *2.10x slower*)    | `6.40 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `568.97 ns` (❌ *10.72x slower*)   | `13.58 us` (❌ *255.92x slower*)   | `111.62 ns` (❌ *2.10x slower*)   | `53.06 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `551.02 ns` (❌ *6.93x slower*)    | `13.50 us` (❌ *169.60x slower*)   | `157.31 ns` (❌ *1.98x slower*)   | `79.57 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.74 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.32 ns` (✅ **1.00x**) | `212.59 ns` (❌ *1.44x slower*)   | `28.08 ns` (🚀 **5.25x faster**)    | `50.34 ns` (🚀 **2.93x faster**)    | `99.40 ns` (✅ **1.48x faster**)     | `631.72 ns` (❌ *4.29x slower*)    |
| **`serialize_uncompressed`**             | `197.46 ns` (✅ **1.00x**) | `317.73 ns` (❌ *1.61x slower*)   | `28.02 ns` (🚀 **7.05x faster**)    | `50.07 ns` (🚀 **3.94x faster**)    | `99.40 ns` (🚀 **1.99x faster**)     | `630.70 ns` (❌ *3.19x slower*)    |
| **`deserialize_compressed`**             | `280.96 us` (✅ **1.00x**) | `970.89 us` (❌ *3.46x slower*)   | `46.36 ns` (🚀 **6060.85x faster**) | `95.37 ns` (🚀 **2945.91x faster**) | `206.86 ns` (🚀 **1358.23x faster**) | `1.27 us` (🚀 **222.09x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.39 us` (✅ **1.00x**)  | `174.54 us` (❌ *2.67x slower*)   | `46.34 ns` (🚀 **1411.09x faster**) | `95.94 ns` (🚀 **681.50x faster**)  | `206.87 ns` (🚀 **316.06x faster**)  | `1.27 us` (🚀 **51.69x faster**)   |
| **`deserialize_uncompressed`**           | `215.78 us` (✅ **1.00x**) | `790.83 us` (❌ *3.66x slower*)   | `46.33 ns` (🚀 **4657.73x faster**) | `95.87 ns` (🚀 **2250.79x faster**) | `206.82 ns` (🚀 **1043.32x faster**) | `1.27 us` (🚀 **170.55x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.36 ns` (✅ **1.00x**) | `472.14 ns` (❌ *2.09x slower*)   | `46.35 ns` (🚀 **4.88x faster**)    | `95.95 ns` (🚀 **2.36x faster**)    | `206.71 ns` (✅ **1.10x faster**)    | `1.27 us` (❌ *5.61x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `8.02 s` (❌ *3.46x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.73 us` (✅ **1.00x**) | `65.17 us` (❌ *2.35x slower*)   | `173.60 us` (❌ *6.26x slower*)    |
| **`legendre_for_qr`**    | `9.54 us` (✅ **1.00x**)  | `29.37 us` (❌ *3.08x slower*)   | `29.46 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.99 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.96 ns` (✅ **1.00x**) | `110.60 ns` (❌ *1.81x slower*)    |
| **`from_big-endian_bits`**    | `60.91 ns` (✅ **1.00x**) | `110.70 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.66 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.91 ns` (✅ **1.00x**) | `78.47 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.75 ns` (✅ **1.00x**) | `41.45 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

