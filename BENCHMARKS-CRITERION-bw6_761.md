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
|        | `2.09 ms` (✅ **1.00x**)          | `2.06 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                             | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `5.18 us` (✅ **1.00x**) | `5.22 us` (✅ **1.01x slower**) | `119.30 ns` (🚀 **43.44x faster**) | `237.06 ns` (🚀 **21.86x faster**) | `37.37 ns` (🚀 **138.68x faster**) | `24.04 ns` (🚀 **215.60x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `5.12 us` (✅ **1.00x**) | `5.42 us` (✅ **1.06x slower**) | `103.78 ns` (🚀 **49.31x faster**) | `205.17 ns` (🚀 **24.94x faster**) | `36.18 ns` (🚀 **141.43x faster**) | `18.60 ns` (🚀 **275.11x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.52 us` (✅ **1.00x**) | `3.77 us` (✅ **1.07x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.54 us` (✅ **1.00x**) | `3.74 us` (✅ **1.06x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.40 us` (✅ **1.00x**) | `2.48 us` (✅ **1.03x slower**) | `89.07 ns` (🚀 **26.93x faster**)  | `196.10 ns` (🚀 **12.23x faster**) | `27.41 ns` (🚀 **87.52x faster**)  | `9.01 ns` (🚀 **266.31x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.83 ms` (✅ **1.00x**) | `1.89 ms` (✅ **1.04x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `90.70 ns` (❌ *4.59x slower*)     | `163.35 ns` (❌ *8.26x slower*)    | `31.37 ns` (❌ *1.59x slower*)     | `19.78 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.79 us` (❌ *32.79x slower*)     | `8.98 us` (❌ *105.64x slower*)    | `353.76 ns` (❌ *4.16x slower*)    | `85.04 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.99 us` (❌ *26.73x slower*)     | `6.34 us` (❌ *84.99x slower*)     | `278.76 ns` (❌ *3.74x slower*)    | `74.56 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `64.43 us` (❌ *4.17x slower*)     | `72.96 us` (❌ *4.72x slower*)     | `61.39 us` (❌ *3.97x slower*)     | `15.45 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.83 us` (❌ *44.25x slower*)     | `18.16 us` (❌ *137.96x slower*)   | `507.87 ns` (❌ *3.86x slower*)    | `131.67 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.74 us` (❌ *30.07x slower*)     | `18.01 us` (❌ *94.30x slower*)    | `739.47 ns` (❌ *3.87x slower*)    | `190.94 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `10.94 ns` (✅ **1.00x**) | `19.19 ns` (❌ *1.75x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `13.57 ns` (✅ **1.00x**) | `26.69 ns` (❌ *1.97x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.64 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.08x slower**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**)  | `4.66 ns` (✅ **1.03x slower**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                 | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `566.50 ns` (✅ **1.00x**) | `585.35 ns` (✅ **1.03x slower**) | `61.39 ns` (🚀 **9.23x faster**)      | `200.30 ns` (🚀 **2.83x faster**)    | `599.04 ns` (✅ **1.06x slower**)  | `1.25 us` (❌ *2.20x slower*)      |
| **`serialize_uncompressed`**             | `790.09 ns` (✅ **1.00x**) | `795.25 ns` (✅ **1.01x slower**) | `61.60 ns` (🚀 **12.83x faster**)     | `197.43 ns` (🚀 **4.00x faster**)    | `592.54 ns` (✅ **1.33x faster**)  | `1.24 us` (❌ *1.58x slower*)      |
| **`deserialize_compressed`**             | `1.75 ms` (✅ **1.00x**)   | `1.72 ms` (✅ **1.02x faster**)   | `111.87 ns` (🚀 **15619.43x faster**) | `415.33 ns` (🚀 **4207.01x faster**) | `1.24 us` (🚀 **1414.74x faster**) | `2.53 us` (🚀 **690.16x faster**)  |
| **`deserialize_compressed_unchecked`**   | `319.43 us` (✅ **1.00x**) | `313.73 us` (✅ **1.02x faster**) | `112.36 ns` (🚀 **2842.80x faster**)  | `407.69 ns` (🚀 **783.52x faster**)  | `1.25 us` (🚀 **255.21x faster**)  | `2.56 us` (🚀 **124.97x faster**)  |
| **`deserialize_uncompressed`**           | `1.43 ms` (✅ **1.00x**)   | `1.37 ms` (✅ **1.04x faster**)   | `110.74 ns` (🚀 **12883.04x faster**) | `406.55 ns` (🚀 **3509.22x faster**) | `1.25 us` (🚀 **1137.82x faster**) | `2.44 us` (🚀 **585.43x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `922.88 ns` (✅ **1.00x**) | `895.63 ns` (✅ **1.03x faster**) | `113.33 ns` (🚀 **8.14x faster**)     | `404.86 ns` (🚀 **2.28x faster**)    | `1.26 us` (❌ *1.37x slower*)      | `2.52 us` (❌ *2.73x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `14.42 s` (✅ **1.00x**) | `14.23 s` (✅ **1.01x faster**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `86.96 us` (✅ **1.00x**) | `318.76 us` (❌ *3.67x slower*)   | `7.80 ms` (❌ *89.75x slower*)     |
| **`legendre_for_qr`**    | `38.42 us` (✅ **1.00x**) | `325.22 us` (❌ *8.47x slower*)   | `329.69 us` (❌ *8.58x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)   | `5.11 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `135.69 ns` (✅ **1.00x**) | `267.84 ns` (❌ *1.97x slower*)    |
| **`from_big-endian_bits`**    | `136.64 ns` (✅ **1.00x**) | `262.88 ns` (❌ *1.92x slower*)    |
| **`comparison`**              | `5.18 ns` (✅ **1.00x**)   | `7.20 ns` (❌ *1.39x slower*)      |
| **`equality`**                | `6.07 ns` (✅ **1.00x**)   | `7.17 ns` (❌ *1.18x slower*)      |
| **`is_zero`**                 | `4.96 ns` (✅ **1.00x**)   | `4.88 ns` (✅ **1.01x faster**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `97.34 ns` (✅ **1.00x**) | `362.10 ns` (❌ *3.72x slower*)    |
| **`into_bigint`** | `53.36 ns` (✅ **1.00x**) | `182.03 ns` (❌ *3.41x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

