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
|        | `2.01 ms` (✅ **1.00x**)          | `2.01 ms` (✅ **1.00x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                             | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.96 us` (✅ **1.00x**) | `4.95 us` (✅ **1.00x faster**) | `106.90 ns` (🚀 **46.36x faster**) | `217.67 ns` (🚀 **22.77x faster**) | `35.83 ns` (🚀 **138.31x faster**) | `22.51 ns` (🚀 **220.17x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.99 us` (✅ **1.00x**) | `5.02 us` (✅ **1.01x slower**) | `99.75 ns` (🚀 **50.04x faster**)  | `199.96 ns` (🚀 **24.96x faster**) | `33.71 ns` (🚀 **148.09x faster**) | `18.27 ns` (🚀 **273.23x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.47 us` (✅ **1.00x**) | `3.50 us` (✅ **1.01x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.51 us` (✅ **1.00x**) | `3.55 us` (✅ **1.01x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.30 us` (✅ **1.00x**) | `2.30 us` (✅ **1.00x faster**) | `80.88 ns` (🚀 **28.49x faster**)  | `165.72 ns` (🚀 **13.91x faster**) | `25.31 ns` (🚀 **91.04x faster**)  | `13.13 ns` (🚀 **175.55x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.79 ms` (✅ **1.00x**) | `1.79 ms` (✅ **1.00x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `79.78 ns` (❌ *3.98x slower*)     | `144.73 ns` (❌ *7.22x slower*)    | `28.41 ns` (❌ *1.42x slower*)     | `20.05 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.55 us` (❌ *30.93x slower*)     | `8.18 us` (❌ *99.24x slower*)     | `320.42 ns` (❌ *3.89x slower*)    | `82.42 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.83 us` (❌ *26.12x slower*)     | `5.71 us` (❌ *81.30x slower*)     | `256.99 ns` (❌ *3.66x slower*)    | `70.23 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `60.13 us` (❌ *3.88x slower*)     | `69.58 us` (❌ *4.49x slower*)     | `57.19 us` (❌ *3.69x slower*)     | `15.48 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.33 us` (❌ *42.72x slower*)     | `16.97 us` (❌ *136.04x slower*)   | `480.12 ns` (❌ *3.85x slower*)    | `124.78 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.23 us` (❌ *28.00x slower*)     | `16.79 us` (❌ *89.98x slower*)    | `676.85 ns` (❌ *3.63x slower*)    | `186.64 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `9.39 ns` (✅ **1.00x**)  | `18.55 ns` (❌ *1.97x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `12.80 ns` (✅ **1.00x**) | `24.78 ns` (❌ *1.94x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.83 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.46 ns` (✅ **1.00x**)  | `4.45 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                 | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `560.96 ns` (✅ **1.00x**) | `558.98 ns` (✅ **1.00x faster**) | `59.63 ns` (🚀 **9.41x faster**)      | `186.53 ns` (🚀 **3.01x faster**)    | `551.66 ns` (✅ **1.02x faster**)  | `1.19 us` (❌ *2.12x slower*)      |
| **`serialize_uncompressed`**             | `750.17 ns` (✅ **1.00x**) | `750.61 ns` (✅ **1.00x slower**) | `59.98 ns` (🚀 **12.51x faster**)     | `186.34 ns` (🚀 **4.03x faster**)    | `552.45 ns` (✅ **1.36x faster**)  | `1.19 us` (❌ *1.59x slower*)      |
| **`deserialize_compressed`**             | `1.62 ms` (✅ **1.00x**)   | `1.63 ms` (✅ **1.01x slower**)   | `110.91 ns` (🚀 **14587.00x faster**) | `365.21 ns` (🚀 **4429.85x faster**) | `1.11 us` (🚀 **1453.24x faster**) | `2.29 us` (🚀 **707.14x faster**)  |
| **`deserialize_compressed_unchecked`**   | `297.80 us` (✅ **1.00x**) | `299.96 us` (✅ **1.01x slower**) | `109.65 ns` (🚀 **2715.84x faster**)  | `361.48 ns` (🚀 **823.84x faster**)  | `1.11 us` (🚀 **267.52x faster**)  | `2.25 us` (🚀 **132.11x faster**)  |
| **`deserialize_uncompressed`**           | `1.33 ms` (✅ **1.00x**)   | `1.32 ms` (✅ **1.01x faster**)   | `111.96 ns` (🚀 **11857.18x faster**) | `364.85 ns` (🚀 **3638.68x faster**) | `1.12 us` (🚀 **1180.50x faster**) | `2.29 us` (🚀 **579.00x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `807.64 ns` (✅ **1.00x**) | `815.31 ns` (✅ **1.01x slower**) | `111.51 ns` (🚀 **7.24x faster**)     | `362.20 ns` (🚀 **2.23x faster**)    | `1.13 us` (❌ *1.40x slower*)      | `2.28 us` (❌ *2.82x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `13.06 s` (✅ **1.00x**) | `13.13 s` (✅ **1.01x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `77.15 us` (✅ **1.00x**) | `299.53 us` (❌ *3.88x slower*)   | `7.15 ms` (❌ *92.64x slower*)     |
| **`legendre_for_qr`**    | `34.82 us` (✅ **1.00x**) | `300.62 us` (❌ *8.63x slower*)   | `304.06 us` (❌ *8.73x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)   | `5.00 ns` (✅ **1.00x faster**)    |
| **`from_little-endian_bits`** | `129.38 ns` (✅ **1.00x**) | `256.90 ns` (❌ *1.99x slower*)    |
| **`from_big-endian_bits`**    | `129.55 ns` (✅ **1.00x**) | `257.14 ns` (❌ *1.98x slower*)    |
| **`comparison`**              | `5.03 ns` (✅ **1.00x**)   | `4.99 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `5.58 ns` (✅ **1.00x**)   | `6.04 ns` (✅ **1.08x slower**)    |
| **`is_zero`**                 | `4.79 ns` (✅ **1.00x**)   | `4.87 ns` (✅ **1.02x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `92.83 ns` (✅ **1.00x**) | `331.56 ns` (❌ *3.57x slower*)    |
| **`into_bigint`** | `49.16 ns` (✅ **1.00x**) | `169.02 ns` (❌ *3.44x slower*)    |

### pairing_for_bw6_761

|        | `g1_preparation_for_bw6_761`          | `g2_preparation_for_bw6_761`          | `miller_loop_for_bw6_761`           | `final_exponentiation_for_bw6_761`          | `full_pairing_for_bw6_761`           |
|:-------|:--------------------------------------|:--------------------------------------|:------------------------------------|:--------------------------------------------|:------------------------------------ |
|        | `30.15 ns` (✅ **1.00x**)              | `1.02 ms` (❌ *33930.62x slower*)      | `3.65 ms` (❌ *120968.40x slower*)   | `4.41 ms` (❌ *146141.47x slower*)           | `9.07 ms` (❌ *300877.51x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

