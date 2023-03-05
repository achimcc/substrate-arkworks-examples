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
|        | `1.98 ms` (✅ **1.00x**)          | `1.97 ms` (✅ **1.00x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.82 us` (✅ **1.00x**) | `4.82 us` (✅ **1.00x faster**) | `78.18 ns` (🚀 **61.60x faster**) | `160.03 ns` (🚀 **30.09x faster**) | `27.59 ns` (🚀 **174.53x faster**) | `12.63 ns` (🚀 **381.31x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.90 us` (✅ **1.00x**) | `4.91 us` (✅ **1.00x slower**) | `78.78 ns` (🚀 **62.26x faster**) | `153.42 ns` (🚀 **31.97x faster**) | `25.93 ns` (🚀 **189.11x faster**) | `13.36 ns` (🚀 **367.12x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.37 us` (✅ **1.00x**) | `3.37 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.40 us` (✅ **1.00x**) | `3.39 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.24 us` (✅ **1.00x**) | `2.24 us` (✅ **1.00x faster**) | `54.77 ns` (🚀 **40.85x faster**) | `118.28 ns` (🚀 **18.92x faster**) | `19.35 ns` (🚀 **115.60x faster**) | `7.14 ns` (🚀 **313.26x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.76 ms` (✅ **1.00x**) | `1.76 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `70.31 ns` (❌ *3.75x slower*)    | `119.54 ns` (❌ *6.38x slower*)    | `22.48 ns` (❌ *1.20x slower*)     | `18.75 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.49 us` (❌ *32.69x slower*)    | `7.96 us` (❌ *104.47x slower*)    | `305.45 ns` (❌ *4.01x slower*)    | `76.21 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.80 us` (❌ *26.93x slower*)    | `5.56 us` (❌ *83.36x slower*)     | `244.66 ns` (❌ *3.67x slower*)    | `66.68 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.84 us` (❌ *3.59x slower*)    | `60.96 us` (❌ *4.23x slower*)     | `47.62 us` (❌ *3.30x slower*)     | `14.42 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.13 us` (❌ *43.71x slower*)    | `16.20 us` (❌ *138.04x slower*)   | `419.22 ns` (❌ *3.57x slower*)    | `117.37 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.10 us` (❌ *30.91x slower*)    | `16.10 us` (❌ *97.63x slower*)    | `648.50 ns` (❌ *3.93x slower*)    | `164.93 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.63 ns` (✅ **1.00x**)  | `17.15 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.28 ns` (✅ **1.00x**) | `21.87 ns` (❌ *2.13x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.96 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**)  | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `518.97 ns` (✅ **1.00x**) | `518.37 ns` (✅ **1.00x faster**) | `55.45 ns` (🚀 **9.36x faster**)     | `173.12 ns` (🚀 **3.00x faster**)    | `512.39 ns` (✅ **1.01x faster**)  | `1.12 us` (❌ *2.16x slower*)      |
| **`serialize_uncompressed`**             | `699.35 ns` (✅ **1.00x**) | `699.27 ns` (✅ **1.00x faster**) | `55.91 ns` (🚀 **12.51x faster**)    | `173.18 ns` (🚀 **4.04x faster**)    | `512.61 ns` (✅ **1.36x faster**)  | `1.13 us` (❌ *1.61x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.60 ms` (✅ **1.00x slower**)   | `93.32 ns` (🚀 **17090.10x faster**) | `342.35 ns` (🚀 **4658.57x faster**) | `1.04 us` (🚀 **1528.49x faster**) | `2.10 us` (🚀 **760.31x faster**)  |
| **`deserialize_compressed_unchecked`**   | `293.29 us` (✅ **1.00x**) | `293.54 us` (✅ **1.00x slower**) | `93.43 ns` (🚀 **3139.04x faster**)  | `341.87 ns` (🚀 **857.88x faster**)  | `1.04 us` (🚀 **281.07x faster**)  | `2.10 us` (🚀 **139.81x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x slower**)   | `93.52 ns` (🚀 **13916.57x faster**) | `341.96 ns` (🚀 **3805.78x faster**) | `1.04 us` (🚀 **1247.58x faster**) | `2.10 us` (🚀 **620.00x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `781.79 ns` (✅ **1.00x**) | `780.48 ns` (✅ **1.00x faster**) | `93.59 ns` (🚀 **8.35x faster**)     | `342.15 ns` (🚀 **2.28x faster**)    | `1.04 us` (❌ *1.33x slower*)      | `2.10 us` (❌ *2.68x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.38 s` (✅ **1.00x**) | `12.41 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.43 us` (✅ **1.00x**) | `292.14 us` (❌ *4.33x slower*)   | `6.99 ms` (❌ *103.61x slower*)    |
| **`legendre_for_qr`**    | `31.75 us` (✅ **1.00x**) | `291.15 us` (❌ *9.17x slower*)   | `298.21 us` (❌ *9.39x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.89 ns` (✅ **1.00x**) | `192.97 ns` (❌ *2.30x slower*)    |
| **`from_big-endian_bits`**    | `83.98 ns` (✅ **1.00x**) | `192.18 ns` (❌ *2.29x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)  | `5.10 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `76.22 ns` (✅ **1.00x**) | `312.79 ns` (❌ *4.10x slower*)    |
| **`into_bigint`** | `47.21 ns` (✅ **1.00x**) | `158.79 ns` (❌ *3.36x slower*)    |

### pairing_for_bw6_761

|        | `g1_preparation_for_bw6_761`          | `g2_preparation_for_bw6_761`          | `miller_loop_for_bw6_761`           | `final_exponentiation_for_bw6_761`          | `full_pairing_for_bw6_761`           |
|:-------|:--------------------------------------|:--------------------------------------|:------------------------------------|:--------------------------------------------|:------------------------------------ |
|        | `19.73 ns` (✅ **1.00x**)              | `996.04 us` (❌ *50475.59x slower*)    | `3.53 ms` (❌ *178838.66x slower*)   | `4.21 ms` (❌ *213531.79x slower*)           | `8.75 ms` (❌ *443261.09x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

