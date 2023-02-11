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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.80 us` (✅ **1.00x**) | `4.81 us` (✅ **1.00x slower**) | `78.28 ns` (🚀 **61.38x faster**) | `162.40 ns` (🚀 **29.59x faster**) | `27.64 ns` (🚀 **173.83x faster**) | `12.64 ns` (🚀 **380.03x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.88 us` (✅ **1.00x**) | `4.88 us` (✅ **1.00x slower**) | `79.52 ns` (🚀 **61.31x faster**) | `154.17 ns` (🚀 **31.63x faster**) | `25.81 ns` (🚀 **188.87x faster**) | `13.28 ns` (🚀 **367.27x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.35 us` (✅ **1.00x**) | `3.35 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.39 us` (✅ **1.00x**) | `3.39 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.25 us` (✅ **1.00x**) | `2.25 us` (✅ **1.00x slower**) | `54.64 ns` (🚀 **41.13x faster**) | `118.82 ns` (🚀 **18.91x faster**) | `19.20 ns` (🚀 **117.00x faster**) | `7.17 ns` (🚀 **313.22x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.75 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `71.79 ns` (❌ *3.70x slower*)    | `120.96 ns` (❌ *6.24x slower*)    | `22.88 ns` (❌ *1.18x slower*)     | `19.38 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.50 us` (❌ *32.67x slower*)    | `7.95 us` (❌ *104.05x slower*)    | `304.57 ns` (❌ *3.99x slower*)    | `76.41 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.78 us` (❌ *26.83x slower*)    | `5.56 us` (❌ *83.62x slower*)     | `244.76 ns` (❌ *3.68x slower*)    | `66.49 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.85 us` (❌ *3.62x slower*)    | `60.97 us` (❌ *4.26x slower*)     | `47.67 us` (❌ *3.33x slower*)     | `14.30 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.13 us` (❌ *43.69x slower*)    | `16.20 us` (❌ *138.10x slower*)   | `419.11 ns` (❌ *3.57x slower*)    | `117.31 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.09 us` (❌ *31.13x slower*)    | `16.14 us` (❌ *98.83x slower*)    | `651.86 ns` (❌ *3.99x slower*)    | `163.36 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.63 ns` (✅ **1.00x**)  | `17.21 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.33 ns` (✅ **1.00x**) | `21.84 ns` (❌ *2.11x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**)  | `4.56 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `512.36 ns` (✅ **1.00x**) | `519.10 ns` (✅ **1.01x slower**) | `58.86 ns` (🚀 **8.70x faster**)     | `175.04 ns` (🚀 **2.93x faster**)    | `513.47 ns` (✅ **1.00x slower**)  | `1.10 us` (❌ *2.15x slower*)      |
| **`serialize_uncompressed`**             | `698.52 ns` (✅ **1.00x**) | `698.42 ns` (✅ **1.00x faster**) | `56.63 ns` (🚀 **12.33x faster**)    | `175.39 ns` (🚀 **3.98x faster**)    | `513.49 ns` (✅ **1.36x faster**)  | `1.10 us` (❌ *1.58x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `93.31 ns` (🚀 **16996.16x faster**) | `339.92 ns` (🚀 **4665.44x faster**) | `1.05 us` (🚀 **1510.96x faster**) | `2.09 us` (🚀 **758.79x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.47 us` (✅ **1.00x**) | `291.38 us` (✅ **1.00x faster**) | `93.22 ns` (🚀 **3126.53x faster**)  | `338.68 ns` (🚀 **860.60x faster**)  | `1.05 us` (🚀 **277.73x faster**)  | `2.09 us` (🚀 **139.47x faster**)  |
| **`deserialize_uncompressed`**           | `1.29 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x slower**)   | `93.23 ns` (🚀 **13889.27x faster**) | `339.85 ns` (🚀 **3810.17x faster**) | `1.05 us` (🚀 **1233.79x faster**) | `2.09 us` (🚀 **619.71x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `778.16 ns` (✅ **1.00x**) | `778.54 ns` (✅ **1.00x slower**) | `93.15 ns` (🚀 **8.35x faster**)     | `339.25 ns` (🚀 **2.29x faster**)    | `1.05 us` (❌ *1.35x slower*)      | `2.09 us` (❌ *2.69x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.29 s` (✅ **1.00x**) | `12.30 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.05 us` (✅ **1.00x**) | `289.79 us` (❌ *4.32x slower*)   | `6.96 ms` (❌ *103.84x slower*)    |
| **`legendre_for_qr`**    | `31.64 us` (✅ **1.00x**) | `290.84 us` (❌ *9.19x slower*)   | `298.16 us` (❌ *9.42x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `84.29 ns` (✅ **1.00x**) | `167.41 ns` (❌ *1.99x slower*)    |
| **`from_big-endian_bits`**    | `84.11 ns` (✅ **1.00x**) | `167.03 ns` (❌ *1.99x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)  | `5.09 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.80 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.63 ns` (✅ **1.00x**) | `311.80 ns` (❌ *4.12x slower*)    |
| **`into_bigint`** | `47.00 ns` (✅ **1.00x**) | `155.37 ns` (❌ *3.31x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

