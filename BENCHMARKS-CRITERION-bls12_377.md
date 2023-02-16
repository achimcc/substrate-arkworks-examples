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
|        | `178.51 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.38x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.13 us` (✅ **1.00x**)   | `4.45 us` (❌ *3.93x slower*)   | `27.08 ns` (🚀 **41.87x faster**)  | `178.01 ns` (🚀 **6.37x faster**)  | `19.19 ns` (🚀 **59.09x faster**) | `8.27 ns` (🚀 **137.09x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `4.52 us` (❌ *3.85x slower*)   | `27.28 ns` (🚀 **43.04x faster**)  | `169.31 ns` (🚀 **6.93x faster**)  | `14.93 ns` (🚀 **78.64x faster**) | `8.61 ns` (🚀 **136.32x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `829.95 ns` (✅ **1.00x**) | `3.18 us` (❌ *3.84x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `858.91 ns` (✅ **1.00x**) | `3.22 us` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `572.08 ns` (✅ **1.00x**) | `2.07 us` (❌ *3.63x slower*)   | `12.76 ns` (🚀 **44.82x faster**)  | `101.35 ns` (🚀 **5.64x faster**)  | `7.46 ns` (🚀 **76.66x faster**)  | `9.10 ns` (🚀 **62.84x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.70 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.73x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.61 ns` (❌ *3.80x slower*)     | `104.74 ns` (❌ *17.62x slower*)   | `16.76 ns` (❌ *2.82x slower*)    | `5.94 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.42 ns` (❌ *7.13x slower*)    | `6.67 us` (❌ *177.93x slower*)    | `69.35 ns` (❌ *1.85x slower*)    | `37.50 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `247.98 ns` (❌ *7.78x slower*)    | `4.69 us` (❌ *147.10x slower*)    | `59.39 ns` (❌ *1.86x slower*)    | `31.87 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.99 us` (❌ *2.22x slower*)     | `25.29 us` (❌ *4.02x slower*)     | `13.67 us` (❌ *2.17x slower*)    | `6.29 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `569.22 ns` (❌ *10.73x slower*)   | `13.59 us` (❌ *256.24x slower*)   | `112.39 ns` (❌ *2.12x slower*)   | `53.05 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `552.45 ns` (❌ *6.95x slower*)    | `13.51 us` (❌ *169.94x slower*)   | `157.25 ns` (❌ *1.98x slower*)   | `79.51 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.62 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.12 ns` (✅ **1.00x**) | `211.42 ns` (❌ *1.44x slower*)   | `27.99 ns` (🚀 **5.26x faster**)    | `50.32 ns` (🚀 **2.92x faster**)    | `99.78 ns` (✅ **1.47x faster**)     | `626.12 ns` (❌ *4.26x slower*)    |
| **`serialize_uncompressed`**             | `198.43 ns` (✅ **1.00x**) | `316.38 ns` (❌ *1.59x slower*)   | `27.98 ns` (🚀 **7.09x faster**)    | `50.06 ns` (🚀 **3.96x faster**)    | `99.80 ns` (🚀 **1.99x faster**)     | `626.22 ns` (❌ *3.16x slower*)    |
| **`deserialize_compressed`**             | `281.34 us` (✅ **1.00x**) | `969.64 us` (❌ *3.45x slower*)   | `46.44 ns` (🚀 **6057.47x faster**) | `93.42 ns` (🚀 **3011.39x faster**) | `206.99 ns` (🚀 **1359.18x faster**) | `1.25 us` (🚀 **225.03x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.62 us` (✅ **1.00x**)  | `174.62 us` (❌ *2.66x slower*)   | `46.44 ns` (🚀 **1412.84x faster**) | `93.42 ns` (🚀 **702.38x faster**)  | `206.99 ns` (🚀 **317.01x faster**)  | `1.25 us` (🚀 **52.47x faster**)   |
| **`deserialize_uncompressed`**           | `215.84 us` (✅ **1.00x**) | `792.97 us` (❌ *3.67x slower*)   | `46.38 ns` (🚀 **4653.43x faster**) | `93.16 ns` (🚀 **2316.83x faster**) | `206.75 ns` (🚀 **1043.97x faster**) | `1.26 us` (🚀 **171.76x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `227.66 ns` (✅ **1.00x**) | `474.69 ns` (❌ *2.09x slower*)   | `46.38 ns` (🚀 **4.91x faster**)    | `93.16 ns` (🚀 **2.44x faster**)    | `206.74 ns` (✅ **1.10x faster**)    | `1.25 us` (❌ *5.49x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.25 s` (✅ **1.00x**)  | `7.90 s` (❌ *3.51x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.68 us` (✅ **1.00x**) | `65.06 us` (❌ *2.35x slower*)   | `174.28 us` (❌ *6.30x slower*)    |
| **`legendre_for_qr`**    | `9.59 us` (✅ **1.00x**)  | `29.25 us` (❌ *3.05x slower*)   | `29.40 us` (❌ *3.07x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.90 ns` (✅ **1.00x**) | `108.41 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `60.90 ns` (✅ **1.00x**) | `108.26 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.32 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.51 ns` (✅ **1.00x**)  | `4.65 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.96 ns` (✅ **1.00x**) | `79.69 ns` (❌ *2.22x slower*)    |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `41.50 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

