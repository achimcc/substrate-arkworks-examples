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
|        | `1.98 ms` (✅ **1.00x**)          | `1.96 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.83 us` (✅ **1.00x**) | `4.83 us` (✅ **1.00x slower**) | `78.41 ns` (🚀 **61.63x faster**) | `162.38 ns` (🚀 **29.76x faster**) | `27.66 ns` (🚀 **174.72x faster**) | `12.67 ns` (🚀 **381.45x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.89 us` (✅ **1.00x**) | `4.89 us` (✅ **1.00x slower**) | `79.41 ns` (🚀 **61.53x faster**) | `153.97 ns` (🚀 **31.73x faster**) | `25.93 ns` (🚀 **188.45x faster**) | `13.30 ns` (🚀 **367.48x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.37 us` (✅ **1.00x**) | `3.37 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.39 us` (✅ **1.00x**) | `3.39 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.23 us` (✅ **1.00x**) | `2.23 us` (✅ **1.00x slower**) | `54.82 ns` (🚀 **40.76x faster**) | `118.40 ns` (🚀 **18.87x faster**) | `19.24 ns` (🚀 **116.10x faster**) | `7.13 ns` (🚀 **313.21x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.75 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `73.24 ns` (❌ *4.05x slower*)    | `120.17 ns` (❌ *6.65x slower*)    | `22.51 ns` (❌ *1.25x slower*)     | `18.07 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.50 us` (❌ *32.68x slower*)    | `7.98 us` (❌ *104.41x slower*)    | `304.78 ns` (❌ *3.99x slower*)    | `76.39 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.80 us` (❌ *26.95x slower*)    | `5.56 us` (❌ *83.36x slower*)     | `243.30 ns` (❌ *3.65x slower*)    | `66.67 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.88 us` (❌ *3.66x slower*)    | `60.99 us` (❌ *4.30x slower*)     | `47.64 us` (❌ *3.36x slower*)     | `14.18 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.15 us` (❌ *43.91x slower*)    | `16.23 us` (❌ *138.31x slower*)   | `420.45 ns` (❌ *3.58x slower*)    | `117.36 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.09 us` (❌ *31.05x slower*)    | `16.14 us` (❌ *98.51x slower*)    | `649.37 ns` (❌ *3.96x slower*)    | `163.83 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.65 ns` (✅ **1.00x**)  | `17.21 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.35 ns` (✅ **1.00x**) | `21.88 ns` (❌ *2.11x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**)  | `4.53 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `515.90 ns` (✅ **1.00x**) | `516.42 ns` (✅ **1.00x slower**) | `56.42 ns` (🚀 **9.14x faster**)     | `168.45 ns` (🚀 **3.06x faster**)    | `509.09 ns` (✅ **1.01x faster**)  | `1.08 us` (❌ *2.10x slower*)      |
| **`serialize_uncompressed`**             | `699.03 ns` (✅ **1.00x**) | `699.26 ns` (✅ **1.00x slower**) | `57.66 ns` (🚀 **12.12x faster**)    | `169.98 ns` (🚀 **4.11x faster**)    | `509.00 ns` (✅ **1.37x faster**)  | `1.08 us` (❌ *1.55x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `94.26 ns` (🚀 **16889.40x faster**) | `341.56 ns` (🚀 **4660.75x faster**) | `1.04 us` (🚀 **1528.82x faster**) | `2.13 us` (🚀 **747.24x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.94 us` (✅ **1.00x**) | `291.96 us` (✅ **1.00x slower**) | `94.55 ns` (🚀 **3087.79x faster**)  | `341.53 ns` (🚀 **854.81x faster**)  | `1.04 us` (🚀 **280.47x faster**)  | `2.13 us` (🚀 **137.11x faster**)  |
| **`deserialize_uncompressed`**           | `1.31 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x faster**)   | `94.16 ns` (🚀 **13867.98x faster**) | `341.52 ns` (🚀 **3823.35x faster**) | `1.04 us` (🚀 **1254.81x faster**) | `2.13 us` (🚀 **613.23x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `765.76 ns` (✅ **1.00x**) | `771.88 ns` (✅ **1.01x slower**) | `94.35 ns` (🚀 **8.12x faster**)     | `341.60 ns` (🚀 **2.24x faster**)    | `1.04 us` (❌ *1.36x slower*)      | `2.13 us` (❌ *2.78x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.65 s` (✅ **1.00x**) | `12.68 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.18 us` (✅ **1.00x**) | `290.35 us` (❌ *4.32x slower*)   | `7.00 ms` (❌ *104.24x slower*)    |
| **`legendre_for_qr`**    | `31.70 us` (✅ **1.00x**) | `293.13 us` (❌ *9.25x slower*)   | `297.29 us` (❌ *9.38x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `89.26 ns` (✅ **1.00x**) | `170.48 ns` (❌ *1.91x slower*)    |
| **`from_big-endian_bits`**    | `89.77 ns` (✅ **1.00x**) | `168.84 ns` (❌ *1.88x slower*)    |
| **`comparison`**              | `5.14 ns` (✅ **1.00x**)  | `5.10 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.75 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.33 ns` (✅ **1.00x**) | `315.05 ns` (❌ *4.18x slower*)    |
| **`into_bigint`** | `46.97 ns` (✅ **1.00x**) | `155.78 ns` (❌ *3.32x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

