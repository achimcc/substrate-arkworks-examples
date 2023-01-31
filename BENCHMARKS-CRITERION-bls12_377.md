# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample](#sample)
    - [arithmetic](#arithmetic)
    - [serialization](#serialization)
    - [msm](#msm)
    - [squareroot](#squareroot)
    - [bitwise](#bitwise)
    - [conversions](#conversions)

## Benchmark Results

### sample

|        | `bls12_377::g1projective elements`          | `bls12_377::g2projective elements`           |
|:-------|:--------------------------------------------|:-------------------------------------------- |
|        | `195.23 us` (✅ **1.00x**)                   | `2.05 ms` (❌ *10.50x slower*)                |

### arithmetic

|                                       | `for bls12_377::fr::bigint`          | `for bls12_377::fq::bigint`          | `for bls12_377::g1projective`          | `for bls12_377::g2projective`          | `for bls12_377::fq2`             | `for bls12_377::fq12`             | `for bls12_377::fq`               | `for bls12_377::fr`               |
|:--------------------------------------|:-------------------------------------|:-------------------------------------|:---------------------------------------|:---------------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                                | `N/A`                                | `1.27 us` (✅ **1.00x**)                | `4.88 us` (❌ *3.83x slower*)           | `22.98 ns` (🚀 **55.41x faster**) | `201.85 ns` (🚀 **6.31x faster**)  | `12.40 ns` (🚀 **102.73x faster**) | `8.81 ns` (🚀 **144.60x faster**)  |
| **`subtraction`**                     | `N/A`                                | `N/A`                                | `1.33 us` (✅ **1.00x**)                | `4.92 us` (❌ *3.71x slower*)           | `23.30 ns` (🚀 **56.89x faster**) | `187.19 ns` (🚀 **7.08x faster**)  | `13.45 ns` (🚀 **98.55x faster**)  | `9.01 ns` (🚀 **147.13x faster**)  |
| **`mixed addition`**                  | `N/A`                                | `N/A`                                | `913.08 ns` (✅ **1.00x**)              | `3.49 us` (❌ *3.82x slower*)           | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed subtraction`**               | `N/A`                                | `N/A`                                | `946.39 ns` (✅ **1.00x**)              | `3.54 us` (❌ *3.74x slower*)           | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                                | `N/A`                                | `598.36 ns` (✅ **1.00x**)              | `2.30 us` (❌ *3.84x slower*)           | `12.33 ns` (🚀 **48.52x faster**) | `151.35 ns` (🚀 **3.95x faster**)  | `7.20 ns` (🚀 **83.11x faster**)   | `5.81 ns` (🚀 **103.05x faster**)  |
| **`scalar multiplication`**           | `N/A`                                | `N/A`                                | `326.74 us` (✅ **1.00x**)              | `1.18 ms` (❌ *3.61x slower*)           | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `22.07 ns` (❌ *3.53x slower*)    | `118.85 ns` (❌ *19.00x slower*)   | `17.97 ns` (❌ *2.87x slower*)     | `6.26 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `274.63 ns` (❌ *6.30x slower*)   | `7.28 us` (❌ *166.87x slower*)    | `80.54 ns` (❌ *1.85x slower*)     | `43.62 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `241.13 ns` (❌ *6.71x slower*)   | `5.14 us` (❌ *143.01x slower*)    | `66.87 ns` (❌ *1.86x slower*)     | `35.92 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `15.20 us` (❌ *2.07x slower*)    | `27.75 us` (❌ *3.78x slower*)     | `14.85 us` (❌ *2.03x slower*)     | `7.33 us` (✅ **1.00x**)           |
| **`sum of products of size 2`**       | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `592.77 ns` (❌ *9.61x slower*)   | `14.91 us` (❌ *241.59x slower*)   | `121.80 ns` (❌ *1.97x slower*)    | `61.71 ns` (✅ **1.00x**)          |
| **`naive sum of products of size 2`** | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `582.06 ns` (❌ *6.48x slower*)   | `14.84 us` (❌ *165.23x slower*)   | `163.45 ns` (❌ *1.82x slower*)    | `89.80 ns` (✅ **1.00x**)          |
| **`addition with carry`**             | `7.61 ns` (✅ **1.00x**)              | `8.63 ns` (❌ *1.13x slower*)         | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction with borrow`**         | `8.74 ns` (✅ **1.00x**)              | `9.83 ns` (❌ *1.12x slower*)         | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication by 2`**             | `4.69 ns` (✅ **1.00x**)              | `4.87 ns` (✅ **1.04x slower**)       | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division by 2`**                   | `4.71 ns` (✅ **1.00x**)              | `4.76 ns` (✅ **1.01x slower**)       | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization

|                                          | `for bls12_377::g1projective`          | `for bls12_377::g2projective`          | `for bls12_377::fr`                | `for bls12_377::fq`                | `for bls12_377::fq2`                | `for bls12_377::fq12`             |
|:-----------------------------------------|:---------------------------------------|:---------------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize compressed`**               | `156.97 ns` (✅ **1.00x**)              | `222.01 ns` (❌ *1.41x slower*)         | `28.77 ns` (🚀 **5.46x faster**)    | `55.48 ns` (🚀 **2.83x faster**)    | `109.64 ns` (✅ **1.43x faster**)    | `706.57 ns` (❌ *4.50x slower*)    |
| **`serialize uncompressed`**             | `208.18 ns` (✅ **1.00x**)              | `337.95 ns` (❌ *1.62x slower*)         | `28.76 ns` (🚀 **7.24x faster**)    | `55.59 ns` (🚀 **3.74x faster**)    | `109.58 ns` (🚀 **1.90x faster**)    | `706.87 ns` (❌ *3.40x slower*)    |
| **`deserialize compressed`**             | `316.30 us` (✅ **1.00x**)              | `1.07 ms` (❌ *3.38x slower*)           | `57.15 ns` (🚀 **5534.98x faster**) | `99.78 ns` (🚀 **3170.05x faster**) | `210.51 ns` (🚀 **1502.56x faster**) | `1.37 us` (🚀 **230.70x faster**)  |
| **`deserialize compressed unchecked`**   | `67.89 us` (✅ **1.00x**)               | `183.59 us` (❌ *2.70x slower*)         | `57.29 ns` (🚀 **1185.05x faster**) | `99.86 ns` (🚀 **679.81x faster**)  | `210.84 ns` (🚀 **321.98x faster**)  | `1.35 us` (🚀 **50.10x faster**)   |
| **`deserialize uncompressed`**           | `248.53 us` (✅ **1.00x**)              | `884.57 us` (❌ *3.56x slower*)         | `56.99 ns` (🚀 **4360.57x faster**) | `99.73 ns` (🚀 **2492.07x faster**) | `209.76 ns` (🚀 **1184.85x faster**) | `1.37 us` (🚀 **181.70x faster**)  |
| **`deserialize uncompressed unchecked`** | `233.54 ns` (✅ **1.00x**)              | `497.28 ns` (❌ *2.13x slower*)         | `57.04 ns` (🚀 **4.09x faster**)    | `99.86 ns` (🚀 **2.34x faster**)    | `210.74 ns` (✅ **1.11x faster**)    | `1.36 us` (❌ *5.81x slower*)      |

### msm

|        | `for bls12_377::g1projective`          | `for bls12_377::g2projective`           |
|:-------|:---------------------------------------|:--------------------------------------- |
|        | `2.43 s` (✅ **1.00x**)                 | `8.50 s` (❌ *3.51x slower*)             |

### squareroot

|                          | `for bls12_377::fr`          | `for bls12_377::fq`             | `for bls12_377::fq2`              |
|:-------------------------|:-----------------------------|:--------------------------------|:--------------------------------- |
| **`square root for qr`** | `31.16 us` (✅ **1.00x**)     | `67.40 us` (❌ *2.16x slower*)   | `182.56 us` (❌ *5.86x slower*)    |
| **`legendre for qr`**    | `10.96 us` (✅ **1.00x**)     | `31.39 us` (❌ *2.86x slower*)   | `31.81 us` (❌ *2.90x slower*)     |

### bitwise

|                               | `operations for bls12_377::fr::bigint`          | `operations for bls12_377::fq::bigint`           |
|:------------------------------|:------------------------------------------------|:------------------------------------------------ |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                         | `5.03 ns` (✅ **1.04x slower**)                   |
| **`from little-endian bits`** | `131.17 ns` (✅ **1.00x**)                       | `190.30 ns` (❌ *1.45x slower*)                   |
| **`from big-endian bits`**    | `130.98 ns` (✅ **1.00x**)                       | `190.18 ns` (❌ *1.45x slower*)                   |
| **`comparison`**              | `4.92 ns` (✅ **1.00x**)                         | `5.08 ns` (✅ **1.03x slower**)                   |
| **`equality`**                | `5.40 ns` (✅ **1.00x**)                         | `5.67 ns` (✅ **1.05x slower**)                   |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                         | `5.21 ns` (✅ **1.06x slower**)                   |

### conversions

|                   | `for bls12_377::fr`          | `for bls12_377::fq`              |
|:------------------|:-----------------------------|:-------------------------------- |
| **`from bigint`** | `40.34 ns` (✅ **1.00x**)     | `75.31 ns` (❌ *1.87x slower*)    |
| **`into bigint`** | `26.46 ns` (✅ **1.00x**)     | `46.84 ns` (❌ *1.77x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

