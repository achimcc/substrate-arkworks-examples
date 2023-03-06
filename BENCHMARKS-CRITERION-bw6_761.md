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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.83 us` (✅ **1.00x**) | `4.83 us` (✅ **1.00x slower**) | `76.22 ns` (🚀 **63.32x faster**) | `160.79 ns` (🚀 **30.02x faster**) | `27.41 ns` (🚀 **176.10x faster**) | `12.48 ns` (🚀 **386.75x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.89 us` (✅ **1.00x**) | `4.89 us` (✅ **1.00x slower**) | `77.21 ns` (🚀 **63.35x faster**) | `153.92 ns` (🚀 **31.78x faster**) | `25.85 ns` (🚀 **189.24x faster**) | `13.05 ns` (🚀 **374.89x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.35 us` (✅ **1.00x**) | `3.35 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.41 us` (✅ **1.00x**) | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.24 us` (✅ **1.00x**) | `2.24 us` (✅ **1.00x slower**) | `54.05 ns` (🚀 **41.35x faster**) | `117.27 ns` (🚀 **19.06x faster**) | `19.18 ns` (🚀 **116.54x faster**) | `7.14 ns` (🚀 **313.04x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.75 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `71.96 ns` (❌ *3.98x slower*)    | `127.42 ns` (❌ *7.04x slower*)    | `23.49 ns` (❌ *1.30x slower*)     | `18.10 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.49 us` (❌ *32.97x slower*)    | `7.95 us` (❌ *105.19x slower*)    | `301.66 ns` (❌ *3.99x slower*)    | `75.61 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.79 us` (❌ *26.81x slower*)    | `5.57 us` (❌ *83.30x slower*)     | `243.83 ns` (❌ *3.64x slower*)    | `66.92 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.64 us` (❌ *3.56x slower*)    | `60.69 us` (❌ *4.19x slower*)     | `47.46 us` (❌ *3.27x slower*)     | `14.49 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.13 us` (❌ *43.75x slower*)    | `16.23 us` (❌ *138.51x slower*)   | `418.90 ns` (❌ *3.57x slower*)    | `117.18 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.07 us` (❌ *30.88x slower*)    | `16.11 us` (❌ *98.17x slower*)    | `646.43 ns` (❌ *3.94x slower*)    | `164.09 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)  | `17.16 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.38 ns` (✅ **1.00x**) | `21.77 ns` (❌ *2.10x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)  | `4.54 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `517.36 ns` (✅ **1.00x**) | `517.94 ns` (✅ **1.00x slower**) | `55.88 ns` (🚀 **9.26x faster**)     | `171.06 ns` (🚀 **3.02x faster**)    | `518.00 ns` (✅ **1.00x slower**)  | `1.07 us` (❌ *2.08x slower*)      |
| **`serialize_uncompressed`**             | `696.61 ns` (✅ **1.00x**) | `696.53 ns` (✅ **1.00x faster**) | `55.82 ns` (🚀 **12.48x faster**)    | `172.27 ns` (🚀 **4.04x faster**)    | `517.64 ns` (✅ **1.35x faster**)  | `1.07 us` (❌ *1.54x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `93.17 ns` (🚀 **17057.88x faster**) | `340.17 ns` (🚀 **4671.94x faster**) | `1.04 us` (🚀 **1525.67x faster**) | `2.09 us` (🚀 **761.57x faster**)  |
| **`deserialize_compressed_unchecked`**   | `292.17 us` (✅ **1.00x**) | `292.29 us` (✅ **1.00x slower**) | `93.12 ns` (🚀 **3137.73x faster**)  | `340.32 ns` (🚀 **858.52x faster**)  | `1.04 us` (🚀 **280.45x faster**)  | `2.09 us` (🚀 **140.02x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x slower**)   | `93.10 ns` (🚀 **13940.23x faster**) | `340.15 ns` (🚀 **3815.63x faster**) | `1.04 us` (🚀 **1245.78x faster**) | `2.09 us` (🚀 **621.95x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `776.52 ns` (✅ **1.00x**) | `771.31 ns` (✅ **1.01x faster**) | `93.01 ns` (🚀 **8.35x faster**)     | `340.25 ns` (🚀 **2.28x faster**)    | `1.04 us` (❌ *1.34x slower*)      | `2.09 us` (❌ *2.69x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.45 s` (✅ **1.00x**) | `12.47 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.41 us` (✅ **1.00x**) | `290.64 us` (❌ *4.31x slower*)   | `6.97 ms` (❌ *103.36x slower*)    |
| **`legendre_for_qr`**    | `31.92 us` (✅ **1.00x**) | `291.22 us` (❌ *9.12x slower*)   | `298.94 us` (❌ *9.36x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.13 ns` (✅ **1.00x**) | `172.79 ns` (❌ *2.08x slower*)    |
| **`from_big-endian_bits`**    | `82.96 ns` (✅ **1.00x**) | `173.97 ns` (❌ *2.10x slower*)    |
| **`comparison`**              | `5.09 ns` (✅ **1.00x**)  | `5.10 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.76 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.05 ns` (✅ **1.00x**) | `312.88 ns` (❌ *4.17x slower*)    |
| **`into_bigint`** | `46.95 ns` (✅ **1.00x**) | `155.98 ns` (❌ *3.32x slower*)    |

### pairing_for_bw6_761

|        | `g1_preparation_for_bw6_761`          | `g2_preparation_for_bw6_761`          | `miller_loop_for_bw6_761`           | `final_exponentiation_for_bw6_761`          | `full_pairing_for_bw6_761`           |
|:-------|:--------------------------------------|:--------------------------------------|:------------------------------------|:--------------------------------------------|:------------------------------------ |
|        | `18.52 ns` (✅ **1.00x**)              | `994.39 us` (❌ *53695.81x slower*)    | `3.53 ms` (❌ *190357.93x slower*)   | `4.21 ms` (❌ *227331.23x slower*)           | `8.74 ms` (❌ *471719.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

