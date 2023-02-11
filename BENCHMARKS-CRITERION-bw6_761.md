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
|        | `1.97 ms` (✅ **1.00x**)          | `1.96 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.81 us` (✅ **1.00x**) | `4.81 us` (✅ **1.00x slower**) | `78.86 ns` (🚀 **60.93x faster**) | `161.35 ns` (🚀 **29.78x faster**) | `27.76 ns` (🚀 **173.13x faster**) | `12.64 ns` (🚀 **380.27x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.88 us` (✅ **1.00x**) | `4.88 us` (✅ **1.00x slower**) | `79.69 ns` (🚀 **61.18x faster**) | `151.91 ns` (🚀 **32.09x faster**) | `25.80 ns` (🚀 **188.98x faster**) | `13.26 ns` (🚀 **367.55x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.34 us` (✅ **1.00x**) | `3.37 us` (✅ **1.01x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.38 us` (✅ **1.00x**) | `3.38 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.24 us` (✅ **1.00x**) | `2.24 us` (✅ **1.00x slower**) | `54.49 ns` (🚀 **41.02x faster**) | `116.58 ns` (🚀 **19.18x faster**) | `19.20 ns` (🚀 **116.41x faster**) | `7.16 ns` (🚀 **312.21x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.75 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `71.44 ns` (❌ *3.89x slower*)    | `118.99 ns` (❌ *6.48x slower*)    | `22.49 ns` (❌ *1.22x slower*)     | `18.38 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.50 us` (❌ *32.65x slower*)    | `7.95 us` (❌ *103.93x slower*)    | `304.59 ns` (❌ *3.98x slower*)    | `76.49 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.78 us` (❌ *26.82x slower*)    | `5.57 us` (❌ *83.72x slower*)     | `244.80 ns` (❌ *3.68x slower*)    | `66.50 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.87 us` (❌ *3.64x slower*)    | `61.00 us` (❌ *4.27x slower*)     | `47.70 us` (❌ *3.34x slower*)     | `14.27 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.13 us` (❌ *43.71x slower*)    | `16.24 us` (❌ *138.40x slower*)   | `419.21 ns` (❌ *3.57x slower*)    | `117.33 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.08 us` (❌ *31.04x slower*)    | `16.12 us` (❌ *98.54x slower*)    | `651.02 ns` (❌ *3.98x slower*)    | `163.61 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)  | `17.23 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.33 ns` (✅ **1.00x**) | `21.90 ns` (❌ *2.12x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**)  | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `512.41 ns` (✅ **1.00x**) | `519.28 ns` (✅ **1.01x slower**) | `58.70 ns` (🚀 **8.73x faster**)     | `175.07 ns` (🚀 **2.93x faster**)    | `513.44 ns` (✅ **1.00x slower**)  | `1.10 us` (❌ *2.14x slower*)      |
| **`serialize_uncompressed`**             | `698.11 ns` (✅ **1.00x**) | `698.15 ns` (✅ **1.00x slower**) | `56.60 ns` (🚀 **12.33x faster**)    | `175.43 ns` (🚀 **3.98x faster**)    | `513.46 ns` (✅ **1.36x faster**)  | `1.10 us` (❌ *1.58x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `93.93 ns` (🚀 **16892.47x faster**) | `339.62 ns` (🚀 **4671.77x faster**) | `1.04 us` (🚀 **1525.38x faster**) | `2.08 us` (🚀 **761.67x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.50 us` (✅ **1.00x**) | `291.40 us` (✅ **1.00x faster**) | `93.87 ns` (🚀 **3105.24x faster**)  | `339.46 ns` (🚀 **858.70x faster**)  | `1.04 us` (🚀 **280.21x faster**)  | `2.08 us` (🚀 **139.95x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x faster**)   | `93.50 ns` (🚀 **13856.79x faster**) | `339.75 ns` (🚀 **3813.37x faster**) | `1.04 us` (🚀 **1245.73x faster**) | `2.08 us` (🚀 **622.00x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `789.82 ns` (✅ **1.00x**) | `789.29 ns` (✅ **1.00x faster**) | `93.77 ns` (🚀 **8.42x faster**)     | `339.74 ns` (🚀 **2.32x faster**)    | `1.04 us` (❌ *1.32x slower*)      | `2.08 us` (❌ *2.64x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.32 s` (✅ **1.00x**) | `12.30 s` (✅ **1.00x faster**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.04 us` (✅ **1.00x**) | `289.86 us` (❌ *4.32x slower*)   | `6.97 ms` (❌ *104.00x slower*)    |
| **`legendre_for_qr`**    | `31.64 us` (✅ **1.00x**) | `290.93 us` (❌ *9.19x slower*)   | `298.05 us` (❌ *9.42x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `84.17 ns` (✅ **1.00x**) | `166.65 ns` (❌ *1.98x slower*)    |
| **`from_big-endian_bits`**    | `84.29 ns` (✅ **1.00x**) | `167.17 ns` (❌ *1.98x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)  | `5.09 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.81 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.07 ns` (✅ **1.00x**) | `311.79 ns` (❌ *4.15x slower*)    |
| **`into_bigint`** | `47.00 ns` (✅ **1.00x**) | `155.42 ns` (❌ *3.31x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

