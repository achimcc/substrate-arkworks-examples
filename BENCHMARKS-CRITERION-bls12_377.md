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
|        | `179.34 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.34x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.14 us` (✅ **1.00x**)   | `4.45 us` (❌ *3.91x slower*)   | `27.22 ns` (🚀 **41.81x faster**)  | `179.58 ns` (🚀 **6.34x faster**)  | `19.24 ns` (🚀 **59.16x faster**) | `8.29 ns` (🚀 **137.18x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `4.50 us` (❌ *3.84x slower*)   | `27.46 ns` (🚀 **42.71x faster**)  | `170.40 ns` (🚀 **6.88x faster**)  | `15.18 ns` (🚀 **77.24x faster**) | `8.60 ns` (🚀 **136.38x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `831.04 ns` (✅ **1.00x**) | `3.19 us` (❌ *3.83x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `856.72 ns` (✅ **1.00x**) | `3.22 us` (❌ *3.76x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `572.77 ns` (✅ **1.00x**) | `2.08 us` (❌ *3.62x slower*)   | `12.79 ns` (🚀 **44.79x faster**)  | `101.73 ns` (🚀 **5.63x faster**)  | `7.49 ns` (🚀 **76.50x faster**)  | `9.11 ns` (🚀 **62.90x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.45 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.51 ns` (❌ *3.79x slower*)     | `101.63 ns` (❌ *17.10x slower*)   | `16.78 ns` (❌ *2.82x slower*)    | `5.94 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.64 ns` (❌ *7.14x slower*)    | `6.67 us` (❌ *177.77x slower*)    | `69.43 ns` (❌ *1.85x slower*)    | `37.51 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `247.96 ns` (❌ *7.78x slower*)    | `4.68 us` (❌ *147.00x slower*)    | `59.48 ns` (❌ *1.87x slower*)    | `31.85 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.98 us` (❌ *2.22x slower*)     | `25.31 us` (❌ *4.03x slower*)     | `13.64 us` (❌ *2.17x slower*)    | `6.29 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `570.67 ns` (❌ *10.75x slower*)   | `13.57 us` (❌ *255.68x slower*)   | `112.41 ns` (❌ *2.12x slower*)   | `53.07 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `553.41 ns` (❌ *7.02x slower*)    | `13.56 us` (❌ *172.01x slower*)   | `157.00 ns` (❌ *1.99x slower*)   | `78.81 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.63 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `148.07 ns` (✅ **1.00x**) | `211.49 ns` (❌ *1.43x slower*)   | `27.84 ns` (🚀 **5.32x faster**)    | `50.34 ns` (🚀 **2.94x faster**)    | `99.86 ns` (✅ **1.48x faster**)     | `626.53 ns` (❌ *4.23x slower*)    |
| **`serialize_uncompressed`**             | `198.64 ns` (✅ **1.00x**) | `316.63 ns` (❌ *1.59x slower*)   | `27.83 ns` (🚀 **7.14x faster**)    | `50.07 ns` (🚀 **3.97x faster**)    | `99.83 ns` (🚀 **1.99x faster**)     | `626.85 ns` (❌ *3.16x slower*)    |
| **`deserialize_compressed`**             | `281.45 us` (✅ **1.00x**) | `970.47 us` (❌ *3.45x slower*)   | `46.55 ns` (🚀 **6046.57x faster**) | `92.04 ns` (🚀 **3057.91x faster**) | `207.12 ns` (🚀 **1358.87x faster**) | `1.26 us` (🚀 **224.24x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.79 us` (✅ **1.00x**)  | `175.35 us` (❌ *2.67x slower*)   | `46.54 ns` (🚀 **1413.57x faster**) | `92.00 ns` (🚀 **715.10x faster**)  | `207.00 ns` (🚀 **317.83x faster**)  | `1.25 us` (🚀 **52.69x faster**)   |
| **`deserialize_uncompressed`**           | `215.93 us` (✅ **1.00x**) | `793.26 us` (❌ *3.67x slower*)   | `46.47 ns` (🚀 **4646.85x faster**) | `92.00 ns` (🚀 **2346.94x faster**) | `206.95 ns` (🚀 **1043.37x faster**) | `1.25 us` (🚀 **172.95x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.45 ns` (✅ **1.00x**) | `469.60 ns` (❌ *2.07x slower*)   | `46.47 ns` (🚀 **4.87x faster**)    | `92.01 ns` (🚀 **2.46x faster**)    | `206.90 ns` (✅ **1.09x faster**)    | `1.25 us` (❌ *5.51x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.27 s` (✅ **1.00x**)  | `8.04 s` (❌ *3.54x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.70 us` (✅ **1.00x**) | `65.59 us` (❌ *2.37x slower*)   | `174.56 us` (❌ *6.30x slower*)    |
| **`legendre_for_qr`**    | `9.58 us` (✅ **1.00x**)  | `29.29 us` (❌ *3.06x slower*)   | `29.45 us` (❌ *3.07x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.84 ns` (✅ **1.00x**) | `108.60 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `60.72 ns` (✅ **1.00x**) | `108.65 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.66 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.96 ns` (✅ **1.00x**) | `79.46 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `21.66 ns` (✅ **1.00x**) | `41.51 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

