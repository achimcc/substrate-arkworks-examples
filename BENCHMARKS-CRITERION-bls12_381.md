# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381](#sample_bls12_381)
    - [arithmetic_for_bls12_381](#arithmetic_for_bls12_381)
    - [serialization_for_bls12_381](#serialization_for_bls12_381)
    - [msm_for_bls12_381](#msm_for_bls12_381)
    - [squareroot_for_bls12_381](#squareroot_for_bls12_381)
    - [bitwise_operations_for_bls12_381](#bitwise_operations_for_bls12_381)
    - [conversions_for_bls12_381](#conversions_for_bls12_381)

## Benchmark Results

### sample_bls12_381

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `207.26 us` (✅ **1.00x**)        | `1.82 ms` (❌ *8.76x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.24 us` (✅ **1.00x**)   | `3.96 us` (❌ *3.19x slower*)     | `23.34 ns` (🚀 **53.18x faster**) | `197.21 ns` (🚀 **6.29x faster**)  | `12.67 ns` (🚀 **97.93x faster**) | `8.67 ns` (🚀 **143.22x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.29 us` (✅ **1.00x**)   | `4.02 us` (❌ *3.12x slower*)     | `23.52 ns` (🚀 **54.73x faster**) | `160.92 ns` (🚀 **8.00x faster**)  | `12.89 ns` (🚀 **99.89x faster**) | `8.78 ns` (🚀 **146.63x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `898.76 ns` (✅ **1.00x**) | `2.84 us` (❌ *3.16x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `927.03 ns` (✅ **1.00x**) | `2.88 us` (❌ *3.11x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `611.63 ns` (✅ **1.00x**) | `1.82 us` (❌ *2.97x slower*)     | `12.48 ns` (🚀 **49.00x faster**) | `67.49 ns` (🚀 **9.06x faster**)   | `7.23 ns` (🚀 **84.58x faster**)  | `5.87 ns` (🚀 **104.28x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `327.46 us` (✅ **1.00x**) | `967.91 us` (❌ *2.96x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `23.20 ns` (❌ *3.78x slower*)    | `95.58 ns` (❌ *15.56x slower*)    | `19.36 ns` (❌ *3.15x slower*)    | `6.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `239.27 ns` (❌ *5.19x slower*)   | `6.19 us` (❌ *134.39x slower*)    | `76.76 ns` (❌ *1.67x slower*)    | `46.06 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `179.97 ns` (❌ *4.76x slower*)   | `4.36 us` (❌ *115.24x slower*)    | `65.67 ns` (❌ *1.74x slower*)    | `37.82 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.25 us` (❌ *2.14x slower*)    | `25.55 us` (❌ *3.58x slower*)     | `14.98 us` (❌ *2.10x slower*)    | `7.14 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.85 ns` (❌ *6.19x slower*)   | `12.72 us` (❌ *151.44x slower*)   | `115.48 ns` (❌ *1.37x slower*)   | `83.99 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `510.79 ns` (❌ *5.81x slower*)   | `12.65 us` (❌ *143.98x slower*)   | `163.38 ns` (❌ *1.86x slower*)   | `87.89 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.74 ns` (✅ **1.00x**) | `10.34 ns` (❌ *1.18x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.59 ns` (✅ **1.00x**) | `4.53 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.60 ns` (✅ **1.00x**) | `205.17 ns` (❌ *1.36x slower*)   | `32.49 ns` (🚀 **4.63x faster**)    | `56.63 ns` (🚀 **2.66x faster**)    | `110.46 ns` (✅ **1.36x faster**)   | `718.37 ns` (❌ *4.77x slower*)    |
| **`serialize_uncompressed`**             | `191.23 ns` (✅ **1.00x**) | `286.34 ns` (❌ *1.50x slower*)   | `32.17 ns` (🚀 **5.94x faster**)    | `56.22 ns` (🚀 **3.40x faster**)    | `110.33 ns` (✅ **1.73x faster**)   | `702.65 ns` (❌ *3.67x slower*)    |
| **`deserialize_compressed`**             | `134.54 us` (✅ **1.00x**) | `269.32 us` (❌ *2.00x slower*)   | `53.15 ns` (🚀 **2531.28x faster**) | `93.42 ns` (🚀 **1440.20x faster**) | `221.87 ns` (🚀 **606.39x faster**) | `1.32 us` (🚀 **101.87x faster**)  |
| **`deserialize_compressed_unchecked`**   | `40.07 us` (✅ **1.00x**)  | `136.79 us` (❌ *3.41x slower*)   | `53.23 ns` (🚀 **752.82x faster**)  | `93.34 ns` (🚀 **429.34x faster**)  | `216.17 ns` (🚀 **185.37x faster**) | `1.32 us` (🚀 **30.35x faster**)   |
| **`deserialize_uncompressed`**           | `94.75 us` (✅ **1.00x**)  | `131.65 us` (❌ *1.39x slower*)   | `53.29 ns` (🚀 **1778.05x faster**) | `93.29 ns` (🚀 **1015.61x faster**) | `216.21 ns` (🚀 **438.24x faster**) | `1.32 us` (🚀 **71.78x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `193.50 ns` (✅ **1.00x**) | `407.52 ns` (❌ *2.11x slower*)   | `53.27 ns` (🚀 **3.63x faster**)    | `93.31 ns` (🚀 **2.07x faster**)    | `216.15 ns` (❌ *1.12x slower*)     | `1.32 us` (❌ *6.81x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.38 s` (✅ **1.00x**)  | `7.11 s` (❌ *2.99x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.65 us` (✅ **1.00x**) | `39.69 us` (❌ *1.55x slower*)   | `135.61 us` (❌ *5.29x slower*)    |
| **`legendre_for_qr`**    | `14.34 us` (✅ **1.00x**) | `40.24 us` (❌ *2.81x slower*)   | `40.09 us` (❌ *2.80x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.02 ns` (✅ **1.00x**) | `89.32 ns` (❌ *1.82x slower*)    |
| **`from_big-endian_bits`**    | `48.95 ns` (✅ **1.00x**) | `89.44 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.74 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.91 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.12 ns` (✅ **1.00x**) | `76.59 ns` (❌ *1.86x slower*)    |
| **`into_bigint`** | `22.45 ns` (✅ **1.00x**) | `48.19 ns` (❌ *2.15x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

