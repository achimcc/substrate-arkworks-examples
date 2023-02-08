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
|        | `63.97 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `487.05 ns` (✅ **1.00x**) | `11.06 ns` (🚀 **44.06x faster**) | `11.11 ns` (🚀 **43.84x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `501.10 ns` (✅ **1.00x**) | `11.46 ns` (🚀 **43.73x faster**) | `11.54 ns` (🚀 **43.44x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `504.61 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `503.91 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                    | `N/A`                           | `397.14 ns` (✅ **1.00x**) | `10.73 ns` (🚀 **37.01x faster**) | `6.56 ns` (🚀 **60.54x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `163.04 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `8.27 ns` (✅ **1.02x slower**)   | `8.09 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `44.85 ns` (✅ **1.05x faster**)  | `46.99 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `39.30 ns` (✅ **1.03x slower**)  | `38.05 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `7.13 us` (✅ **1.02x slower**)   | `6.98 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `64.00 ns` (✅ **1.12x faster**)  | `71.38 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `98.98 ns` (✅ **1.03x slower**)  | `96.48 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.87 ns` (✅ **1.00x**)  | `7.85 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `10.72 ns` (✅ **1.00x**) | `10.48 ns` (✅ **1.02x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.58 ns` (✅ **1.00x**)  | `4.50 ns` (✅ **1.02x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.22 ns` (✅ **1.00x**)  | `4.25 ns` (✅ **1.01x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `81.97 ns` (✅ **1.00x**)  | `36.88 ns` (🚀 **2.22x faster**)    | `37.72 ns` (🚀 **2.17x faster**)     |
| **`serialize_uncompressed`**             | `68.90 ns` (✅ **1.00x**)  | `37.29 ns` (🚀 **1.85x faster**)    | `37.61 ns` (🚀 **1.83x faster**)     |
| **`deserialize_compressed`**             | `215.46 us` (✅ **1.00x**) | `60.73 ns` (🚀 **3547.88x faster**) | `58.86 ns` (🚀 **3660.64x faster**)  |
| **`deserialize_compressed_unchecked`**   | `42.46 us` (✅ **1.00x**)  | `59.98 ns` (🚀 **707.85x faster**)  | `59.40 ns` (🚀 **714.76x faster**)   |
| **`deserialize_uncompressed`**           | `179.02 us` (✅ **1.00x**) | `60.41 ns` (🚀 **2963.39x faster**) | `62.29 ns` (🚀 **2874.12x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `187.20 ns` (✅ **1.00x**) | `58.45 ns` (🚀 **3.20x faster**)    | `60.61 ns` (🚀 **3.09x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.79 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `13.65 us` (✅ **1.00x**) | `34.31 us` (❌ *2.51x slower*)    |
| **`legendre_for_qr`**    | `13.88 us` (✅ **1.00x**) | `12.59 us` (✅ **1.10x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.45 ns` (✅ **1.00x**)  | `4.50 ns` (✅ **1.01x slower**)   |
| **`from_little-endian_bits`** | `74.15 ns` (✅ **1.00x**) | `74.10 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `72.75 ns` (✅ **1.00x**) | `73.24 ns` (✅ **1.01x slower**)  |
| **`comparison`**              | `4.55 ns` (✅ **1.00x**)  | `4.48 ns` (✅ **1.02x faster**)   |
| **`equality`**                | `4.89 ns` (✅ **1.00x**)  | `5.05 ns` (✅ **1.03x slower**)   |
| **`is_zero`**                 | `4.23 ns` (✅ **1.00x**)  | `4.20 ns` (✅ **1.01x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `44.04 ns` (✅ **1.00x**) | `42.36 ns` (✅ **1.04x faster**)  |
| **`into_bigint`** | `26.12 ns` (✅ **1.00x**) | `27.75 ns` (✅ **1.06x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

