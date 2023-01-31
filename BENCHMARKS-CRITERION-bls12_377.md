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
|        | `178.72 us` (✅ **1.00x**)        | `1.86 ms` (❌ *10.39x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.14 us` (✅ **1.00x**)   | `4.46 us` (❌ *3.91x slower*)   | `27.35 ns` (🚀 **41.69x faster**)  | `156.60 ns` (🚀 **7.28x faster**)  | `19.00 ns` (🚀 **60.01x faster**) | `8.30 ns` (🚀 **137.36x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.18 us` (✅ **1.00x**)   | `4.50 us` (❌ *3.82x slower*)   | `27.79 ns` (🚀 **42.38x faster**)  | `173.50 ns` (🚀 **6.79x faster**)  | `13.75 ns` (🚀 **85.66x faster**) | `8.62 ns` (🚀 **136.56x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `838.75 ns` (✅ **1.00x**) | `3.19 us` (❌ *3.80x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `858.94 ns` (✅ **1.00x**) | `3.23 us` (❌ *3.75x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `573.76 ns` (✅ **1.00x**) | `2.08 us` (❌ *3.62x slower*)   | `15.91 ns` (🚀 **36.06x faster**)  | `105.27 ns` (🚀 **5.45x faster**)  | `11.22 ns` (🚀 **51.12x faster**) | `5.29 ns` (🚀 **108.45x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `285.99 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.72x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.62 ns` (❌ *3.80x slower*)     | `101.96 ns` (❌ *17.14x slower*)   | `16.76 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.05 ns` (❌ *7.13x slower*)    | `6.68 us` (❌ *178.43x slower*)    | `69.38 ns` (❌ *1.85x slower*)    | `37.44 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `247.79 ns` (❌ *7.83x slower*)    | `4.71 us` (❌ *148.84x slower*)    | `58.81 ns` (❌ *1.86x slower*)    | `31.64 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.72 us` (❌ *2.17x slower*)     | `24.98 us` (❌ *3.94x slower*)     | `13.36 us` (❌ *2.11x slower*)    | `6.33 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `571.31 ns` (❌ *10.75x slower*)   | `13.57 us` (❌ *255.32x slower*)   | `111.64 ns` (❌ *2.10x slower*)   | `53.15 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `553.13 ns` (❌ *6.90x slower*)    | `13.52 us` (❌ *168.60x slower*)   | `156.74 ns` (❌ *1.95x slower*)   | `80.19 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.63 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `129.71 ns` (✅ **1.00x**) | `212.10 ns` (❌ *1.64x slower*)   | `28.01 ns` (🚀 **4.63x faster**)    | `50.28 ns` (🚀 **2.58x faster**)    | `100.28 ns` (✅ **1.29x faster**)    | `628.79 ns` (❌ *4.85x slower*)    |
| **`serialize_uncompressed`**             | `198.85 ns` (✅ **1.00x**) | `318.67 ns` (❌ *1.60x slower*)   | `27.91 ns` (🚀 **7.13x faster**)    | `50.07 ns` (🚀 **3.97x faster**)    | `100.26 ns` (🚀 **1.98x faster**)    | `628.82 ns` (❌ *3.16x slower*)    |
| **`deserialize_compressed`**             | `282.58 us` (✅ **1.00x**) | `971.00 us` (❌ *3.44x slower*)   | `47.03 ns` (🚀 **6008.21x faster**) | `94.84 ns` (🚀 **2979.67x faster**) | `206.88 ns` (🚀 **1365.95x faster**) | `1.25 us` (🚀 **225.34x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.27 us` (✅ **1.00x**)  | `173.83 us` (❌ *2.66x slower*)   | `46.99 ns` (🚀 **1389.25x faster**) | `94.84 ns` (🚀 **688.28x faster**)  | `206.91 ns` (🚀 **315.47x faster**)  | `1.25 us` (🚀 **52.06x faster**)   |
| **`deserialize_uncompressed`**           | `217.41 us` (✅ **1.00x**) | `793.40 us` (❌ *3.65x slower*)   | `46.56 ns` (🚀 **4669.61x faster**) | `93.72 ns` (🚀 **2319.72x faster**) | `206.64 ns` (🚀 **1052.08x faster**) | `1.26 us` (🚀 **172.99x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `225.71 ns` (✅ **1.00x**) | `468.23 ns` (❌ *2.07x slower*)   | `46.94 ns` (🚀 **4.81x faster**)    | `93.72 ns` (🚀 **2.41x faster**)    | `206.67 ns` (✅ **1.09x faster**)    | `1.11 us` (❌ *4.90x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.30 s` (✅ **1.00x**)  | `7.98 s` (❌ *3.47x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.70 us` (✅ **1.00x**) | `64.61 us` (❌ *2.33x slower*)   | `172.79 us` (❌ *6.24x slower*)    |
| **`legendre_for_qr`**    | `9.55 us` (✅ **1.00x**)  | `29.24 us` (❌ *3.06x slower*)   | `29.41 us` (❌ *3.08x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.68 ns` (✅ **1.00x**) | `108.75 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `60.79 ns` (✅ **1.00x**) | `108.85 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `3.97 ns` (✅ **1.00x**)  | `4.21 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.51 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.98 ns` (✅ **1.00x**) | `78.97 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.93 ns` (✅ **1.00x**) | `41.51 ns` (❌ *1.89x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

