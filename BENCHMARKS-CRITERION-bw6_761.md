# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bw6_761](#sample_bw6_761)
    - [arithmetic_for_bw6_761](#arithmetic_for_bw6_761)
    - [serialization_for_bw6_761](#serialization_for_bw6_761)
    - [msm_for_bw6_761](#msm_for_bw6_761)
    - [squareroot_for_bw6_761](#squareroot_for_bw6_761)
    - [bitwise_operations_for_bw6_761](#bitwise_operations_for_bw6_761)
    - [conversions_for_bw6_761](#conversions_for_bw6_761)

## Benchmark Results

### sample_bw6_761

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `1.70 ms` (✅ **1.00x**)          | `1.68 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.15 us` (✅ **1.00x**) | `4.15 us` (✅ **1.00x slower**) | `90.18 ns` (🚀 **46.04x faster**) | `180.23 ns` (🚀 **23.04x faster**) | `30.06 ns` (🚀 **138.12x faster**) | `19.01 ns` (🚀 **218.44x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.20 us` (✅ **1.00x**) | `4.20 us` (✅ **1.00x faster**) | `80.95 ns` (🚀 **51.91x faster**) | `157.87 ns` (🚀 **26.62x faster**) | `27.91 ns` (🚀 **150.56x faster**) | `15.41 ns` (🚀 **272.61x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.95 us` (✅ **1.00x**) | `2.95 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.93 us` (✅ **1.00x**) | `1.93 us` (✅ **1.00x faster**) | `71.84 ns` (🚀 **26.92x faster**) | `143.00 ns` (🚀 **13.52x faster**) | `21.80 ns` (🚀 **88.70x faster**)  | `11.16 ns` (🚀 **173.22x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.50 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `69.66 ns` (❌ *4.16x slower*)    | `124.19 ns` (❌ *7.41x slower*)    | `23.94 ns` (❌ *1.43x slower*)     | `16.75 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.16 us` (❌ *30.48x slower*)    | `6.87 us` (❌ *97.07x slower*)     | `270.03 ns` (❌ *3.81x slower*)    | `70.79 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.56 us` (❌ *26.58x slower*)    | `4.83 us` (❌ *82.32x slower*)     | `215.67 ns` (❌ *3.68x slower*)    | `58.64 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `50.23 us` (❌ *3.74x slower*)    | `57.92 us` (❌ *4.31x slower*)     | `46.67 us` (❌ *3.48x slower*)     | `13.43 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.44 us` (❌ *41.97x slower*)    | `13.99 us` (❌ *132.19x slower*)   | `401.20 ns` (❌ *3.79x slower*)    | `105.87 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.39 us` (❌ *27.80x slower*)    | `13.89 us` (❌ *87.98x slower*)    | `567.81 ns` (❌ *3.60x slower*)    | `157.85 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)  | `15.62 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**) | `21.18 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)  | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `469.76 ns` (✅ **1.00x**) | `467.32 ns` (✅ **1.01x faster**) | `50.26 ns` (🚀 **9.35x faster**)     | `156.35 ns` (🚀 **3.00x faster**)    | `464.43 ns` (✅ **1.01x faster**)    | `985.40 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `628.38 ns` (✅ **1.00x**) | `628.45 ns` (✅ **1.00x slower**) | `50.04 ns` (🚀 **12.56x faster**)    | `157.02 ns` (🚀 **4.00x faster**)    | `464.45 ns` (✅ **1.35x faster**)    | `985.46 ns` (❌ *1.57x slower*)    |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)   | `1.36 ms` (✅ **1.00x faster**)   | `95.89 ns` (🚀 **14224.14x faster**) | `304.55 ns` (🚀 **4478.71x faster**) | `945.62 ns` (🚀 **1442.43x faster**) | `1.90 us` (🚀 **717.84x faster**)  |
| **`deserialize_compressed_unchecked`**   | `251.96 us` (✅ **1.00x**) | `251.98 us` (✅ **1.00x slower**) | `95.89 ns` (🚀 **2627.50x faster**)  | `304.54 ns` (🚀 **827.33x faster**)  | `945.73 ns` (🚀 **266.42x faster**)  | `1.90 us` (🚀 **132.60x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x faster**)   | `95.22 ns` (🚀 **11676.63x faster**) | `304.48 ns` (🚀 **3651.59x faster**) | `941.98 ns` (🚀 **1180.31x faster**) | `1.91 us` (🚀 **582.47x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `693.88 ns` (✅ **1.00x**) | `694.56 ns` (✅ **1.00x slower**) | `95.21 ns` (🚀 **7.29x faster**)     | `304.50 ns` (🚀 **2.28x faster**)    | `941.87 ns` (❌ *1.36x slower*)      | `1.90 us` (❌ *2.74x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `10.92 s` (✅ **1.00x**) | `10.94 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.81 us` (✅ **1.00x**) | `250.70 us` (❌ *3.87x slower*)   | `5.98 ms` (❌ *92.34x slower*)     |
| **`legendre_for_qr`**    | `29.22 us` (✅ **1.00x**) | `250.92 us` (❌ *8.59x slower*)   | `257.01 us` (❌ *8.80x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `107.74 ns` (✅ **1.00x**) | `217.91 ns` (❌ *2.02x slower*)    |
| **`from_big-endian_bits`**    | `107.70 ns` (✅ **1.00x**) | `217.66 ns` (❌ *2.02x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)   | `4.19 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)   | `5.03 ns` (✅ **1.08x slower**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.31 ns` (✅ **1.00x**) | `278.21 ns` (❌ *3.51x slower*)    |
| **`into_bigint`** | `41.46 ns` (✅ **1.00x**) | `143.91 ns` (❌ *3.47x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

