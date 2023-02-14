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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.83 us` (✅ **1.00x**) | `4.83 us` (✅ **1.00x slower**) | `77.50 ns` (🚀 **62.33x faster**) | `160.39 ns` (🚀 **30.12x faster**) | `27.71 ns` (🚀 **174.37x faster**) | `12.66 ns` (🚀 **381.59x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.89 us` (✅ **1.00x**) | `4.89 us` (✅ **1.00x slower**) | `78.54 ns` (🚀 **62.26x faster**) | `152.46 ns` (🚀 **32.07x faster**) | `25.90 ns` (🚀 **188.78x faster**) | `13.28 ns` (🚀 **368.32x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.37 us` (✅ **1.00x**) | `3.37 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.39 us` (✅ **1.00x**) | `3.39 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.23 us` (✅ **1.00x**) | `2.23 us` (✅ **1.00x slower**) | `54.07 ns` (🚀 **41.31x faster**) | `118.28 ns` (🚀 **18.88x faster**) | `19.45 ns` (🚀 **114.86x faster**) | `7.13 ns` (🚀 **313.23x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.75 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `71.01 ns` (❌ *3.83x slower*)    | `119.34 ns` (❌ *6.43x slower*)    | `22.87 ns` (❌ *1.23x slower*)     | `18.55 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.51 us` (❌ *33.11x slower*)    | `7.96 us` (❌ *104.75x slower*)    | `304.73 ns` (❌ *4.01x slower*)    | `75.95 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.80 us` (❌ *26.91x slower*)    | `5.56 us` (❌ *83.20x slower*)     | `243.14 ns` (❌ *3.64x slower*)    | `66.77 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.79 us` (❌ *3.63x slower*)    | `60.92 us` (❌ *4.27x slower*)     | `47.52 us` (❌ *3.33x slower*)     | `14.25 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.15 us` (❌ *43.92x slower*)    | `16.23 us` (❌ *138.33x slower*)   | `420.25 ns` (❌ *3.58x slower*)    | `117.35 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.13 us` (❌ *31.32x slower*)    | `16.14 us` (❌ *98.45x slower*)    | `649.41 ns` (❌ *3.96x slower*)    | `163.93 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)  | `17.18 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.33 ns` (✅ **1.00x**) | `21.84 ns` (❌ *2.11x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**)  | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `515.78 ns` (✅ **1.00x**) | `516.23 ns` (✅ **1.00x slower**) | `56.51 ns` (🚀 **9.13x faster**)     | `168.20 ns` (🚀 **3.07x faster**)    | `509.26 ns` (✅ **1.01x faster**)  | `1.08 us` (❌ *2.10x slower*)      |
| **`serialize_uncompressed`**             | `698.96 ns` (✅ **1.00x**) | `698.90 ns` (✅ **1.00x faster**) | `57.20 ns` (🚀 **12.22x faster**)    | `169.77 ns` (🚀 **4.12x faster**)    | `510.03 ns` (✅ **1.37x faster**)  | `1.08 us` (❌ *1.55x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `93.62 ns` (🚀 **16992.50x faster**) | `341.28 ns` (🚀 **4661.52x faster**) | `1.05 us` (🚀 **1515.38x faster**) | `2.12 us` (🚀 **748.75x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.81 us` (✅ **1.00x**) | `291.80 us` (✅ **1.00x faster**) | `93.82 ns` (🚀 **3110.33x faster**)  | `341.30 ns` (🚀 **855.00x faster**)  | `1.05 us` (🚀 **277.93x faster**)  | `2.12 us` (🚀 **137.32x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x faster**)   | `93.59 ns` (🚀 **13881.50x faster**) | `341.16 ns` (🚀 **3808.05x faster**) | `1.05 us` (🚀 **1237.52x faster**) | `2.12 us` (🚀 **611.50x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `769.32 ns` (✅ **1.00x**) | `777.38 ns` (✅ **1.01x slower**) | `93.59 ns` (🚀 **8.22x faster**)     | `341.38 ns` (🚀 **2.25x faster**)    | `1.05 us` (❌ *1.36x slower*)      | `2.12 us` (❌ *2.76x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.56 s` (✅ **1.00x**) | `12.57 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.15 us` (✅ **1.00x**) | `290.34 us` (❌ *4.32x slower*)   | `7.00 ms` (❌ *104.27x slower*)    |
| **`legendre_for_qr`**    | `31.71 us` (✅ **1.00x**) | `293.02 us` (❌ *9.24x slower*)   | `297.32 us` (❌ *9.38x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `89.99 ns` (✅ **1.00x**) | `168.48 ns` (❌ *1.87x slower*)    |
| **`from_big-endian_bits`**    | `85.29 ns` (✅ **1.00x**) | `167.14 ns` (❌ *1.96x slower*)    |
| **`comparison`**              | `5.15 ns` (✅ **1.00x**)  | `5.10 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.76 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.38 ns` (✅ **1.00x**) | `314.95 ns` (❌ *4.18x slower*)    |
| **`into_bigint`** | `46.96 ns` (✅ **1.00x**) | `155.67 ns` (❌ *3.32x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

