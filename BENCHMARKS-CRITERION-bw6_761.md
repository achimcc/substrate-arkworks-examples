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
|        | `1.50 ms` (✅ **1.00x**)          | `1.49 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.16 us` (✅ **1.00x**) | `4.16 us` (✅ **1.00x slower**) | `79.32 ns` (🚀 **52.39x faster**) | `179.67 ns` (🚀 **23.13x faster**) | `30.04 ns` (🚀 **138.34x faster**) | `19.00 ns` (🚀 **218.76x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.21 us` (✅ **1.00x**) | `4.21 us` (✅ **1.00x faster**) | `72.00 ns` (🚀 **58.44x faster**) | `158.46 ns` (🚀 **26.55x faster**) | `27.85 ns` (🚀 **151.08x faster**) | `15.30 ns` (🚀 **274.98x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.95 us` (✅ **1.00x**) | `2.61 us` (✅ **1.13x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.94 us` (✅ **1.00x**) | `1.94 us` (✅ **1.00x faster**) | `68.19 ns` (🚀 **28.44x faster**) | `139.12 ns` (🚀 **13.94x faster**) | `21.32 ns` (🚀 **90.96x faster**)  | `11.18 ns` (🚀 **173.52x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.49 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `69.31 ns` (❌ *4.13x slower*)    | `124.26 ns` (❌ *7.40x slower*)    | `24.02 ns` (❌ *1.43x slower*)     | `16.79 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.16 us` (❌ *35.28x slower*)    | `6.88 us` (❌ *112.40x slower*)    | `271.80 ns` (❌ *4.44x slower*)    | `61.22 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.55 us` (❌ *29.79x slower*)    | `4.85 us` (❌ *92.94x slower*)     | `217.17 ns` (❌ *4.16x slower*)    | `52.18 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `49.95 us` (❌ *3.76x slower*)    | `57.91 us` (❌ *4.36x slower*)     | `46.61 us` (❌ *3.51x slower*)     | `13.28 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.45 us` (❌ *41.99x slower*)    | `14.05 us` (❌ *132.62x slower*)   | `401.09 ns` (❌ *3.79x slower*)    | `105.96 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.39 us` (❌ *31.64x slower*)    | `13.92 us` (❌ *100.21x slower*)   | `568.55 ns` (❌ *4.09x slower*)    | `138.89 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.91 ns` (✅ **1.00x**)  | `15.68 ns` (❌ *2.27x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.63 ns` (✅ **1.00x**) | `18.74 ns` (❌ *1.76x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)  | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `469.97 ns` (✅ **1.00x**) | `412.75 ns` (✅ **1.14x faster**) | `50.31 ns` (🚀 **9.34x faster**)     | `157.26 ns` (🚀 **2.99x faster**)    | `467.24 ns` (✅ **1.01x faster**)    | `990.19 ns` (❌ *2.11x slower*)    |
| **`serialize_uncompressed`**             | `628.80 ns` (✅ **1.00x**) | `628.83 ns` (✅ **1.00x slower**) | `50.09 ns` (🚀 **12.55x faster**)    | `139.08 ns` (🚀 **4.52x faster**)    | `467.25 ns` (✅ **1.35x faster**)    | `873.13 ns` (❌ *1.39x slower*)    |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)   | `1.36 ms` (✅ **1.00x slower**)   | `93.73 ns` (🚀 **14542.47x faster**) | `271.00 ns` (🚀 **5029.71x faster**) | `943.50 ns` (🚀 **1444.66x faster**) | `1.68 us` (🚀 **811.06x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.72 us` (✅ **1.00x**) | `222.93 us` (✅ **1.13x faster**) | `93.77 ns` (🚀 **2695.14x faster**)  | `305.69 ns` (🚀 **826.73x faster**)  | `947.60 ns` (🚀 **266.69x faster**)  | `1.91 us` (🚀 **132.13x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x slower**)   | `93.25 ns` (🚀 **11913.84x faster**) | `305.76 ns` (🚀 **3633.52x faster**) | `947.85 ns` (🚀 **1172.12x faster**) | `1.90 us` (🚀 **583.29x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `611.57 ns` (✅ **1.00x**) | `697.98 ns` (❌ *1.14x slower*)   | `93.26 ns` (🚀 **6.56x faster**)     | `305.79 ns` (🚀 **2.00x faster**)    | `943.40 ns` (❌ *1.54x slower*)      | `1.90 us` (❌ *3.11x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `9.93 s` (✅ **1.00x**)  | `11.22 s` (❌ *1.13x slower*)    |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.89 us` (✅ **1.00x**) | `251.44 us` (❌ *3.87x slower*)   | `5.98 ms` (❌ *92.21x slower*)     |
| **`legendre_for_qr`**    | `29.04 us` (✅ **1.00x**) | `251.71 us` (❌ *8.67x slower*)   | `257.67 us` (❌ *8.87x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `3.75 ns` (✅ **1.12x faster**)    |
| **`from_little-endian_bits`** | `107.57 ns` (✅ **1.00x**) | `210.42 ns` (❌ *1.96x slower*)    |
| **`from_big-endian_bits`**    | `107.60 ns` (✅ **1.00x**) | `186.15 ns` (❌ *1.73x slower*)    |
| **`comparison`**              | `4.23 ns` (✅ **1.00x**)   | `4.20 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `4.71 ns` (✅ **1.00x**)   | `4.64 ns` (✅ **1.02x faster**)    |
| **`is_zero`**                 | `4.01 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `69.32 ns` (✅ **1.00x**) | `246.59 ns` (❌ *3.56x slower*)    |
| **`into_bigint`** | `36.62 ns` (✅ **1.00x**) | `142.04 ns` (❌ *3.88x slower*)    |

### pairing_for_bw6_761

|        | `g1_preparation_for_bw6_761`          | `g2_preparation_for_bw6_761`          | `miller_loop_for_bw6_761`           | `final_exponentiation_for_bw6_761`          | `full_pairing_for_bw6_761`           |
|:-------|:--------------------------------------|:--------------------------------------|:------------------------------------|:--------------------------------------------|:------------------------------------ |
|        | `14.64 ns` (✅ **1.00x**)              | `751.97 us` (❌ *51370.48x slower*)    | `3.03 ms` (❌ *207177.46x slower*)   | `3.23 ms` (❌ *220380.39x slower*)           | `7.54 ms` (❌ *514902.52x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

