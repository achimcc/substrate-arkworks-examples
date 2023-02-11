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
|        | `177.90 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.41x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.15 us` (✅ **1.00x**)   | `4.46 us` (❌ *3.89x slower*)   | `26.42 ns` (🚀 **43.40x faster**)  | `179.01 ns` (🚀 **6.40x faster**)  | `19.26 ns` (🚀 **59.54x faster**) | `8.29 ns` (🚀 **138.39x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.18 us` (✅ **1.00x**)   | `4.50 us` (❌ *3.82x slower*)   | `27.50 ns` (🚀 **42.86x faster**)  | `172.04 ns` (🚀 **6.85x faster**)  | `14.91 ns` (🚀 **79.07x faster**) | `8.59 ns` (🚀 **137.17x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `832.03 ns` (✅ **1.00x**) | `3.18 us` (❌ *3.83x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `865.46 ns` (✅ **1.00x**) | `3.21 us` (❌ *3.71x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `574.58 ns` (✅ **1.00x**) | `2.07 us` (❌ *3.60x slower*)   | `12.90 ns` (🚀 **44.55x faster**)  | `104.05 ns` (🚀 **5.52x faster**)  | `7.48 ns` (🚀 **76.86x faster**)  | `9.12 ns` (🚀 **63.02x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `285.55 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.72x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.90 ns` (❌ *3.85x slower*)     | `100.72 ns` (❌ *16.95x slower*)   | `17.04 ns` (❌ *2.87x slower*)    | `5.94 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.37 ns` (❌ *7.16x slower*)    | `6.65 us` (❌ *178.24x slower*)    | `70.87 ns` (❌ *1.90x slower*)    | `37.34 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `250.35 ns` (❌ *7.66x slower*)    | `4.68 us` (❌ *143.33x slower*)    | `59.24 ns` (❌ *1.81x slower*)    | `32.68 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.91 us` (❌ *2.20x slower*)     | `25.21 us` (❌ *3.98x slower*)     | `13.57 us` (❌ *2.14x slower*)    | `6.33 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `566.61 ns` (❌ *10.67x slower*)   | `13.57 us` (❌ *255.69x slower*)   | `112.26 ns` (❌ *2.12x slower*)   | `53.08 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `553.29 ns` (❌ *6.94x slower*)    | `13.49 us` (❌ *169.22x slower*)   | `157.37 ns` (❌ *1.97x slower*)   | `79.71 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.74 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `148.14 ns` (✅ **1.00x**) | `214.07 ns` (❌ *1.45x slower*)   | `27.99 ns` (🚀 **5.29x faster**)    | `50.14 ns` (🚀 **2.95x faster**)    | `100.08 ns` (✅ **1.48x faster**)    | `629.82 ns` (❌ *4.25x slower*)    |
| **`serialize_uncompressed`**             | `198.21 ns` (✅ **1.00x**) | `319.38 ns` (❌ *1.61x slower*)   | `27.90 ns` (🚀 **7.10x faster**)    | `50.17 ns` (🚀 **3.95x faster**)    | `100.48 ns` (🚀 **1.97x faster**)    | `627.91 ns` (❌ *3.17x slower*)    |
| **`deserialize_compressed`**             | `282.15 us` (✅ **1.00x**) | `972.21 us` (❌ *3.45x slower*)   | `46.39 ns` (🚀 **6082.61x faster**) | `95.69 ns` (🚀 **2948.61x faster**) | `206.09 ns` (🚀 **1369.06x faster**) | `1.27 us` (🚀 **223.00x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.05 us` (✅ **1.00x**)  | `173.86 us` (❌ *2.67x slower*)   | `46.38 ns` (🚀 **1402.47x faster**) | `95.70 ns` (🚀 **679.70x faster**)  | `207.49 ns` (🚀 **313.50x faster**)  | `1.27 us` (🚀 **51.40x faster**)   |
| **`deserialize_uncompressed`**           | `216.86 us` (✅ **1.00x**) | `792.76 us` (❌ *3.66x slower*)   | `46.33 ns` (🚀 **4680.42x faster**) | `96.55 ns` (🚀 **2246.16x faster**) | `206.05 ns` (🚀 **1052.42x faster**) | `1.27 us` (🚀 **171.05x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `225.39 ns` (✅ **1.00x**) | `472.24 ns` (❌ *2.10x slower*)   | `46.34 ns` (🚀 **4.86x faster**)    | `95.79 ns` (🚀 **2.35x faster**)    | `207.53 ns` (✅ **1.09x faster**)    | `1.27 us` (❌ *5.61x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.30 s` (✅ **1.00x**)  | `7.95 s` (❌ *3.46x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.67 us` (✅ **1.00x**) | `64.64 us` (❌ *2.34x slower*)   | `172.97 us` (❌ *6.25x slower*)    |
| **`legendre_for_qr`**    | `9.54 us` (✅ **1.00x**)  | `29.36 us` (❌ *3.08x slower*)   | `29.46 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `61.09 ns` (✅ **1.00x**) | `109.25 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `61.07 ns` (✅ **1.00x**) | `108.62 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.30 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.65 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.95 ns` (✅ **1.00x**) | `79.38 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `21.66 ns` (✅ **1.00x**) | `41.55 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

