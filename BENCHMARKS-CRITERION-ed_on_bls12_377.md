# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_edonbls12_377](#sample_edonbls12_377)
    - [arithmetic_for_edonbls12_377](#arithmetic_for_edonbls12_377)
    - [serialization_for_edonbls12_377](#serialization_for_edonbls12_377)
    - [msm_for_edonbls12_377](#msm_for_edonbls12_377)
    - [squareroot_for_edonbls12_377](#squareroot_for_edonbls12_377)
    - [bitwise_operations_for_edonbls12_377](#bitwise_operations_for_edonbls12_377)
    - [conversions_for_edonbls12_377](#conversions_for_edonbls12_377)

## Benchmark Results

### sample_edonbls12_377

|        | `g_elements`              |
|:-------|:------------------------- |
|        | `64.39 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `492.50 ns` (✅ **1.00x**) | `10.33 ns` (🚀 **47.66x faster**) | `10.40 ns` (🚀 **47.37x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `530.07 ns` (✅ **1.00x**) | `12.50 ns` (🚀 **42.42x faster**) | `12.93 ns` (🚀 **41.01x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `502.97 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `516.00 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                    | `N/A`                           | `396.02 ns` (✅ **1.00x**) | `6.10 ns` (🚀 **64.94x faster**)  | `6.40 ns` (🚀 **61.85x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `158.79 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `6.83 ns` (✅ **1.06x faster**)   | `7.25 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `47.21 ns` (✅ **1.04x slower**)  | `45.34 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `35.59 ns` (✅ **1.00x faster**)  | `35.77 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `7.25 us` (✅ **1.08x slower**)   | `6.72 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `62.35 ns` (✅ **1.03x slower**)  | `60.75 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `94.84 ns` (✅ **1.00x slower**)  | `94.40 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.11 ns` (✅ **1.00x**)  | `7.64 ns` (✅ **1.06x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `10.04 ns` (✅ **1.00x**) | `10.21 ns` (✅ **1.02x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.48 ns` (✅ **1.00x**)  | `4.43 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.43 ns` (✅ **1.00x**)  | `4.23 ns` (✅ **1.05x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `93.28 ns` (✅ **1.00x**)  | `31.93 ns` (🚀 **2.92x faster**)    | `34.76 ns` (🚀 **2.68x faster**)     |
| **`serialize_uncompressed`**             | `118.24 ns` (✅ **1.00x**) | `34.01 ns` (🚀 **3.48x faster**)    | `33.58 ns` (🚀 **3.52x faster**)     |
| **`deserialize_compressed`**             | `205.35 us` (✅ **1.00x**) | `82.01 ns` (🚀 **2503.94x faster**) | `82.76 ns` (🚀 **2481.47x faster**)  |
| **`deserialize_compressed_unchecked`**   | `42.72 us` (✅ **1.00x**)  | `78.59 ns` (🚀 **543.58x faster**)  | `81.96 ns` (🚀 **521.20x faster**)   |
| **`deserialize_uncompressed`**           | `163.56 us` (✅ **1.00x**) | `75.95 ns` (🚀 **2153.40x faster**) | `81.95 ns` (🚀 **1995.75x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `192.45 ns` (✅ **1.00x**) | `75.78 ns` (🚀 **2.54x faster**)    | `84.38 ns` (🚀 **2.28x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.74 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.44 us` (✅ **1.00x**) | `32.62 us` (❌ *2.62x slower*)    |
| **`legendre_for_qr`**    | `12.58 us` (✅ **1.00x**) | `11.99 us` (✅ **1.05x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.76 ns` (✅ **1.00x**)   | `4.55 ns` (✅ **1.05x faster**)    |
| **`from_little-endian_bits`** | `179.27 ns` (✅ **1.00x**) | `171.72 ns` (✅ **1.04x faster**)  |
| **`from_big-endian_bits`**    | `172.39 ns` (✅ **1.00x**) | `172.92 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.54 ns` (✅ **1.00x**)   | `4.55 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `4.77 ns` (✅ **1.00x**)   | `4.82 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `4.25 ns` (✅ **1.00x**)   | `4.24 ns` (✅ **1.00x faster**)    |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.52 ns` (✅ **1.00x**) | `42.69 ns` (✅ **1.02x faster**)  |
| **`into_bigint`** | `28.82 ns` (✅ **1.00x**) | `29.82 ns` (✅ **1.03x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

