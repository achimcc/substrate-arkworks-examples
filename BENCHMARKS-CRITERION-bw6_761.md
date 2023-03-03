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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.81 us` (✅ **1.00x**) | `4.82 us` (✅ **1.00x slower**) | `75.50 ns` (🚀 **63.76x faster**) | `158.72 ns` (🚀 **30.33x faster**) | `27.67 ns` (🚀 **173.99x faster**) | `12.64 ns` (🚀 **380.96x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.87 us` (✅ **1.00x**) | `4.87 us` (✅ **1.00x slower**) | `76.49 ns` (🚀 **63.67x faster**) | `150.19 ns` (🚀 **32.43x faster**) | `25.78 ns` (🚀 **188.92x faster**) | `13.27 ns` (🚀 **366.96x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.36 us` (✅ **1.00x**) | `3.36 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.38 us` (✅ **1.00x**) | `3.38 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.23 us` (✅ **1.00x**) | `2.24 us` (✅ **1.00x slower**) | `54.47 ns` (🚀 **41.03x faster**) | `116.06 ns` (🚀 **19.26x faster**) | `19.03 ns` (🚀 **117.44x faster**) | `7.13 ns` (🚀 **313.50x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.76 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `71.00 ns` (❌ *3.90x slower*)    | `118.41 ns` (❌ *6.51x slower*)    | `22.32 ns` (❌ *1.23x slower*)     | `18.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.47 us` (❌ *32.42x slower*)    | `7.91 us` (❌ *103.78x slower*)    | `307.94 ns` (❌ *4.04x slower*)    | `76.21 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.80 us` (❌ *27.04x slower*)    | `5.53 us` (❌ *83.26x slower*)     | `244.19 ns` (❌ *3.67x slower*)    | `66.45 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.70 us` (❌ *3.54x slower*)    | `60.78 us` (❌ *4.17x slower*)     | `47.57 us` (❌ *3.26x slower*)     | `14.59 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.09 us` (❌ *43.33x slower*)    | `16.16 us` (❌ *137.66x slower*)   | `417.98 ns` (❌ *3.56x slower*)    | `117.39 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.04 us` (❌ *30.79x slower*)    | `16.05 us` (❌ *98.14x slower*)    | `653.50 ns` (❌ *4.00x slower*)    | `163.54 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.62 ns` (✅ **1.00x**)  | `17.24 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.30 ns` (✅ **1.00x**) | `21.90 ns` (❌ *2.13x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.71 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `525.23 ns` (✅ **1.00x**) | `523.16 ns` (✅ **1.00x faster**) | `56.83 ns` (🚀 **9.24x faster**)     | `170.40 ns` (🚀 **3.08x faster**)    | `515.24 ns` (✅ **1.02x faster**)  | `1.10 us` (❌ *2.10x slower*)      |
| **`serialize_uncompressed`**             | `698.81 ns` (✅ **1.00x**) | `695.22 ns` (✅ **1.01x faster**) | `55.59 ns` (🚀 **12.57x faster**)    | `174.46 ns` (🚀 **4.01x faster**)    | `515.08 ns` (✅ **1.36x faster**)  | `1.10 us` (❌ *1.58x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `94.03 ns` (🚀 **16936.05x faster**) | `347.09 ns` (🚀 **4588.22x faster**) | `1.04 us` (🚀 **1524.55x faster**) | `2.09 us` (🚀 **763.21x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.39 us` (✅ **1.00x**) | `291.51 us` (✅ **1.00x slower**) | `94.01 ns` (🚀 **3099.65x faster**)  | `347.06 ns` (🚀 **839.61x faster**)  | `1.04 us` (🚀 **279.00x faster**)  | `2.09 us` (🚀 **139.65x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x slower**)   | `93.98 ns` (🚀 **13852.68x faster**) | `347.01 ns` (🚀 **3751.79x faster**) | `1.04 us` (🚀 **1246.74x faster**) | `2.09 us` (🚀 **624.00x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `771.92 ns` (✅ **1.00x**) | `767.10 ns` (✅ **1.01x faster**) | `93.88 ns` (🚀 **8.22x faster**)     | `347.04 ns` (🚀 **2.22x faster**)    | `1.04 us` (❌ *1.35x slower*)      | `2.09 us` (❌ *2.70x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.56 s` (✅ **1.00x**) | `12.58 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.10 us` (✅ **1.00x**) | `289.71 us` (❌ *4.32x slower*)   | `6.99 ms` (❌ *104.16x slower*)    |
| **`legendre_for_qr`**    | `31.64 us` (✅ **1.00x**) | `289.62 us` (❌ *9.15x slower*)   | `296.81 us` (❌ *9.38x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `91.99 ns` (✅ **1.00x**) | `186.60 ns` (❌ *2.03x slower*)    |
| **`from_big-endian_bits`**    | `89.66 ns` (✅ **1.00x**) | `185.32 ns` (❌ *2.07x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)  | `5.09 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.75 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.22 ns` (✅ **1.00x**) | `310.88 ns` (❌ *4.13x slower*)    |
| **`into_bigint`** | `47.10 ns` (✅ **1.00x**) | `158.87 ns` (❌ *3.37x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

