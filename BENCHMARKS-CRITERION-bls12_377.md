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
|        | `178.26 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.40x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.13 us` (✅ **1.00x**)   | `4.45 us` (❌ *3.93x slower*)   | `27.33 ns` (🚀 **41.51x faster**)  | `180.05 ns` (🚀 **6.30x faster**)  | `19.22 ns` (🚀 **59.02x faster**) | `8.30 ns` (🚀 **136.69x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `4.50 us` (❌ *3.84x slower*)   | `27.42 ns` (🚀 **42.67x faster**)  | `170.90 ns` (🚀 **6.85x faster**)  | `14.97 ns` (🚀 **78.15x faster**) | `8.63 ns` (🚀 **135.55x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `830.65 ns` (✅ **1.00x**) | `3.18 us` (❌ *3.83x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `859.55 ns` (✅ **1.00x**) | `3.22 us` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `572.15 ns` (✅ **1.00x**) | `2.07 us` (❌ *3.63x slower*)   | `12.84 ns` (🚀 **44.58x faster**)  | `104.57 ns` (🚀 **5.47x faster**)  | `7.48 ns` (🚀 **76.53x faster**)  | `9.12 ns` (🚀 **62.71x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.45 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.51 ns` (❌ *3.78x slower*)     | `100.67 ns` (❌ *16.93x slower*)   | `17.24 ns` (❌ *2.90x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.86 ns` (❌ *7.14x slower*)    | `6.67 us` (❌ *177.89x slower*)    | `69.39 ns` (❌ *1.85x slower*)    | `37.51 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `247.79 ns` (❌ *7.77x slower*)    | `4.69 us` (❌ *147.10x slower*)    | `59.39 ns` (❌ *1.86x slower*)    | `31.90 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.98 us` (❌ *2.22x slower*)     | `25.27 us` (❌ *4.02x slower*)     | `13.64 us` (❌ *2.17x slower*)    | `6.29 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `567.55 ns` (❌ *10.69x slower*)   | `13.60 us` (❌ *256.25x slower*)   | `112.39 ns` (❌ *2.12x slower*)   | `53.07 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `552.08 ns` (❌ *6.94x slower*)    | `13.52 us` (❌ *169.94x slower*)   | `156.82 ns` (❌ *1.97x slower*)   | `79.54 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**) | `10.63 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.24 ns` (✅ **1.00x**) | `211.93 ns` (❌ *1.44x slower*)   | `28.06 ns` (🚀 **5.25x faster**)    | `49.97 ns` (🚀 **2.95x faster**)    | `100.40 ns` (✅ **1.47x faster**)    | `625.91 ns` (❌ *4.25x slower*)    |
| **`serialize_uncompressed`**             | `198.51 ns` (✅ **1.00x**) | `317.00 ns` (❌ *1.60x slower*)   | `28.03 ns` (🚀 **7.08x faster**)    | `50.03 ns` (🚀 **3.97x faster**)    | `100.38 ns` (🚀 **1.98x faster**)    | `626.17 ns` (❌ *3.15x slower*)    |
| **`deserialize_compressed`**             | `281.13 us` (✅ **1.00x**) | `970.57 us` (❌ *3.45x slower*)   | `46.56 ns` (🚀 **6038.31x faster**) | `95.46 ns` (🚀 **2945.03x faster**) | `206.82 ns` (🚀 **1359.33x faster**) | `1.27 us` (🚀 **222.20x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.25 us` (✅ **1.00x**)  | `174.28 us` (❌ *2.67x slower*)   | `46.51 ns` (🚀 **1402.82x faster**) | `95.46 ns` (🚀 **683.49x faster**)  | `206.71 ns` (🚀 **315.64x faster**)  | `1.27 us` (🚀 **51.56x faster**)   |
| **`deserialize_uncompressed`**           | `216.04 us` (✅ **1.00x**) | `793.74 us` (❌ *3.67x slower*)   | `46.43 ns` (🚀 **4653.25x faster**) | `95.39 ns` (🚀 **2264.74x faster**) | `206.80 ns` (🚀 **1044.67x faster**) | `1.27 us` (🚀 **170.67x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `225.25 ns` (✅ **1.00x**) | `470.32 ns` (❌ *2.09x slower*)   | `45.47 ns` (🚀 **4.95x faster**)    | `95.38 ns` (🚀 **2.36x faster**)    | `206.80 ns` (✅ **1.09x faster**)    | `1.27 us` (❌ *5.62x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.26 s` (✅ **1.00x**)  | `8.00 s` (❌ *3.54x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.70 us` (✅ **1.00x**) | `64.86 us` (❌ *2.34x slower*)   | `173.62 us` (❌ *6.27x slower*)    |
| **`legendre_for_qr`**    | `9.59 us` (✅ **1.00x**)  | `29.28 us` (❌ *3.05x slower*)   | `29.43 us` (❌ *3.07x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.99 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.75 ns` (✅ **1.00x**) | `108.33 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `60.92 ns` (✅ **1.00x**) | `108.25 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.51 ns` (✅ **1.00x**)  | `4.65 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.88 ns` (✅ **1.00x**) | `78.47 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.74 ns` (✅ **1.00x**) | `41.48 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

