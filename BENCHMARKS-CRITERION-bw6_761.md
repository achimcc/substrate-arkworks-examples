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
    - [pairing_for_bw6_761](#pairing_for_bw6_761)

## Benchmark Results

### sample_bw6_761

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `1.98 ms` (✅ **1.00x**)          | `1.96 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.82 us` (✅ **1.00x**) | `4.82 us` (✅ **1.00x slower**) | `75.41 ns` (🚀 **63.98x faster**) | `159.90 ns` (🚀 **30.17x faster**) | `27.37 ns` (🚀 **176.30x faster**) | `12.47 ns` (🚀 **386.96x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.89 us` (✅ **1.00x**) | `4.89 us` (✅ **1.00x slower**) | `76.22 ns` (🚀 **64.16x faster**) | `151.82 ns` (🚀 **32.21x faster**) | `25.81 ns` (🚀 **189.43x faster**) | `13.07 ns` (🚀 **374.20x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.35 us` (✅ **1.00x**) | `3.35 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.41 us` (✅ **1.00x**) | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.24 us` (✅ **1.00x**) | `2.24 us` (✅ **1.00x slower**) | `53.36 ns` (🚀 **41.90x faster**) | `116.40 ns` (🚀 **19.21x faster**) | `19.14 ns` (🚀 **116.80x faster**) | `7.13 ns` (🚀 **313.62x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.75 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `72.96 ns` (❌ *3.96x slower*)    | `119.05 ns` (❌ *6.46x slower*)    | `22.54 ns` (❌ *1.22x slower*)     | `18.43 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.49 us` (❌ *32.92x slower*)    | `7.96 us` (❌ *105.23x slower*)    | `301.87 ns` (❌ *3.99x slower*)    | `75.66 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.79 us` (❌ *26.81x slower*)    | `5.55 us` (❌ *82.98x slower*)     | `243.81 ns` (❌ *3.65x slower*)    | `66.88 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.62 us` (❌ *3.57x slower*)    | `60.75 us` (❌ *4.20x slower*)     | `47.41 us` (❌ *3.28x slower*)     | `14.46 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.13 us` (❌ *43.76x slower*)    | `16.21 us` (❌ *138.39x slower*)   | `418.73 ns` (❌ *3.57x slower*)    | `117.14 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.07 us` (❌ *30.94x slower*)    | `16.10 us` (❌ *98.24x slower*)    | `645.31 ns` (❌ *3.94x slower*)    | `163.90 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.65 ns` (✅ **1.00x**)  | `17.15 ns` (❌ *1.98x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.38 ns` (✅ **1.00x**) | `21.80 ns` (❌ *2.10x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)  | `4.54 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `517.67 ns` (✅ **1.00x**) | `518.17 ns` (✅ **1.00x slower**) | `55.89 ns` (🚀 **9.26x faster**)     | `171.31 ns` (🚀 **3.02x faster**)    | `517.72 ns` (✅ **1.00x slower**)  | `1.07 us` (❌ *2.08x slower*)      |
| **`serialize_uncompressed`**             | `695.99 ns` (✅ **1.00x**) | `696.15 ns` (✅ **1.00x slower**) | `55.84 ns` (🚀 **12.46x faster**)    | `172.27 ns` (🚀 **4.04x faster**)    | `517.76 ns` (✅ **1.34x faster**)  | `1.07 us` (❌ *1.54x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `94.23 ns` (🚀 **16862.66x faster**) | `340.24 ns` (🚀 **4670.15x faster**) | `1.03 us` (🚀 **1539.02x faster**) | `2.08 us` (🚀 **765.10x faster**)  |
| **`deserialize_compressed_unchecked`**   | `292.12 us` (✅ **1.00x**) | `292.19 us` (✅ **1.00x slower**) | `94.25 ns` (🚀 **3099.55x faster**)  | `340.26 ns` (🚀 **858.53x faster**)  | `1.03 us` (🚀 **282.94x faster**)  | `2.08 us` (🚀 **140.64x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x slower**)   | `94.21 ns` (🚀 **13771.85x faster**) | `340.65 ns` (🚀 **3808.87x faster**) | `1.03 us` (🚀 **1256.86x faster**) | `2.08 us` (🚀 **624.79x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `777.47 ns` (✅ **1.00x**) | `770.97 ns` (✅ **1.01x faster**) | `93.97 ns` (🚀 **8.27x faster**)     | `340.18 ns` (🚀 **2.29x faster**)    | `1.03 us` (❌ *1.33x slower*)      | `2.08 us` (❌ *2.67x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.35 s` (✅ **1.00x**) | `12.35 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.41 us` (✅ **1.00x**) | `290.55 us` (❌ *4.31x slower*)   | `6.97 ms` (❌ *103.39x slower*)    |
| **`legendre_for_qr`**    | `31.90 us` (✅ **1.00x**) | `291.17 us` (❌ *9.13x slower*)   | `298.83 us` (❌ *9.37x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.64 ns` (✅ **1.00x**) | `174.29 ns` (❌ *2.08x slower*)    |
| **`from_big-endian_bits`**    | `83.30 ns` (✅ **1.00x**) | `176.61 ns` (❌ *2.12x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)  | `5.09 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.68 ns` (✅ **1.00x**)  | `5.76 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `76.87 ns` (✅ **1.00x**) | `312.62 ns` (❌ *4.07x slower*)    |
| **`into_bigint`** | `46.89 ns` (✅ **1.00x**) | `155.69 ns` (❌ *3.32x slower*)    |

### pairing_for_bw6_761

|        | `g1_preparation_for_bw6_761`          | `g2_preparation_for_bw6_761`          | `miller_loop_for_bw6_761`           | `final_exponentiation_for_bw6_761`          | `full_pairing_for_bw6_761`           |
|:-------|:--------------------------------------|:--------------------------------------|:------------------------------------|:--------------------------------------------|:------------------------------------ |
|        | `18.55 ns` (✅ **1.00x**)              | `994.38 us` (❌ *53617.44x slower*)    | `3.53 ms` (❌ *190222.95x slower*)   | `4.21 ms` (❌ *226757.74x slower*)           | `8.73 ms` (❌ *470949.02x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

