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
|        | `178.48 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.36x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.14 us` (✅ **1.00x**)   | `4.45 us` (❌ *3.90x slower*)   | `27.10 ns` (🚀 **42.14x faster**)  | `178.54 ns` (🚀 **6.40x faster**)  | `19.30 ns` (🚀 **59.17x faster**) | `8.29 ns` (🚀 **137.68x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.18 us` (✅ **1.00x**)   | `4.51 us` (❌ *3.83x slower*)   | `27.77 ns` (🚀 **42.47x faster**)  | `170.17 ns` (🚀 **6.93x faster**)  | `14.86 ns` (🚀 **79.37x faster**) | `8.60 ns` (🚀 **137.17x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `836.36 ns` (✅ **1.00x**) | `3.18 us` (❌ *3.80x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `862.47 ns` (✅ **1.00x**) | `3.21 us` (❌ *3.73x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `571.58 ns` (✅ **1.00x**) | `2.07 us` (❌ *3.62x slower*)   | `12.84 ns` (🚀 **44.52x faster**)  | `101.14 ns` (🚀 **5.65x faster**)  | `7.49 ns` (🚀 **76.34x faster**)  | `9.08 ns` (🚀 **62.98x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `285.16 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.72x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.87 ns` (❌ *3.85x slower*)     | `104.91 ns` (❌ *17.64x slower*)   | `17.24 ns` (❌ *2.90x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.65 ns` (❌ *7.18x slower*)    | `6.66 us` (❌ *178.73x slower*)    | `69.44 ns` (❌ *1.86x slower*)    | `37.29 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `248.98 ns` (❌ *7.82x slower*)    | `4.66 us` (❌ *146.21x slower*)    | `59.11 ns` (❌ *1.86x slower*)    | `31.85 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.73 us` (❌ *2.17x slower*)     | `25.07 us` (❌ *3.97x slower*)     | `13.37 us` (❌ *2.12x slower*)    | `6.32 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `571.04 ns` (❌ *10.76x slower*)   | `13.59 us` (❌ *256.27x slower*)   | `112.18 ns` (❌ *2.11x slower*)   | `53.05 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `554.03 ns` (❌ *6.98x slower*)    | `13.54 us` (❌ *170.45x slower*)   | `158.79 ns` (❌ *2.00x slower*)   | `79.41 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.91 ns` (❌ *1.21x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.68 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.92 ns` (✅ **1.00x**) | `213.73 ns` (❌ *1.44x slower*)   | `27.95 ns` (🚀 **5.29x faster**)    | `50.31 ns` (🚀 **2.94x faster**)    | `99.35 ns` (✅ **1.49x faster**)     | `631.37 ns` (❌ *4.27x slower*)    |
| **`serialize_uncompressed`**             | `198.87 ns` (✅ **1.00x**) | `317.78 ns` (❌ *1.60x slower*)   | `27.84 ns` (🚀 **7.14x faster**)    | `50.08 ns` (🚀 **3.97x faster**)    | `99.38 ns` (🚀 **2.00x faster**)     | `630.96 ns` (❌ *3.17x slower*)    |
| **`deserialize_compressed`**             | `281.64 us` (✅ **1.00x**) | `968.92 us` (❌ *3.44x slower*)   | `46.52 ns` (🚀 **6054.62x faster**) | `93.60 ns` (🚀 **3009.11x faster**) | `206.62 ns` (🚀 **1363.12x faster**) | `1.25 us` (🚀 **225.26x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.64 us` (✅ **1.00x**)  | `174.43 us` (❌ *2.66x slower*)   | `46.52 ns` (🚀 **1410.98x faster**) | `93.62 ns` (🚀 **701.08x faster**)  | `207.63 ns` (🚀 **316.13x faster**)  | `1.25 us` (🚀 **52.67x faster**)   |
| **`deserialize_uncompressed`**           | `216.36 us` (✅ **1.00x**) | `790.03 us` (❌ *3.65x slower*)   | `46.18 ns` (🚀 **4685.06x faster**) | `93.22 ns` (🚀 **2320.85x faster**) | `208.89 ns` (🚀 **1035.75x faster**) | `1.25 us` (🚀 **173.14x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.20 ns` (✅ **1.00x**) | `469.77 ns` (❌ *2.08x slower*)   | `46.41 ns` (🚀 **4.87x faster**)    | `93.50 ns` (🚀 **2.42x faster**)    | `206.34 ns` (✅ **1.10x faster**)    | `1.25 us` (❌ *5.52x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.29 s` (✅ **1.00x**)  | `8.01 s` (❌ *3.49x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.73 us` (✅ **1.00x**) | `65.32 us` (❌ *2.36x slower*)   | `174.16 us` (❌ *6.28x slower*)    |
| **`legendre_for_qr`**    | `9.56 us` (✅ **1.00x**)  | `29.35 us` (❌ *3.07x slower*)   | `29.88 us` (❌ *3.12x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.75 ns` (✅ **1.00x**) | `110.06 ns` (❌ *1.81x slower*)    |
| **`from_big-endian_bits`**    | `60.78 ns` (✅ **1.00x**) | `110.14 ns` (❌ *1.81x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.32 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.73 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.91 ns` (✅ **1.00x**) | `78.53 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.73 ns` (✅ **1.00x**) | `41.48 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

