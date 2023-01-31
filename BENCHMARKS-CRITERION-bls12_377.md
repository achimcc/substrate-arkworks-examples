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
|        | `195.12 us` (✅ **1.00x**)                   | `2.05 ms` (❌ *10.51x slower*)                |

### arithmetic

|                                       | `for bls12_377::fr::bigint`          | `for bls12_377::fq::bigint`          | `for bls12_377::g1projective`          | `for bls12_377::g2projective`          | `for bls12_377::fq2`             | `for bls12_377::fq12`             | `for bls12_377::fq`               | `for bls12_377::fr`               |
|:--------------------------------------|:-------------------------------------|:-------------------------------------|:---------------------------------------|:---------------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                                | `N/A`                                | `1.27 us` (✅ **1.00x**)                | `4.87 us` (❌ *3.82x slower*)           | `22.96 ns` (🚀 **55.51x faster**) | `202.59 ns` (🚀 **6.29x faster**)  | `12.38 ns` (🚀 **102.96x faster**) | `8.81 ns` (🚀 **144.72x faster**)  |
| **`subtraction`**                     | `N/A`                                | `N/A`                                | `1.31 us` (✅ **1.00x**)                | `4.92 us` (❌ *3.77x slower*)           | `23.31 ns` (🚀 **56.01x faster**) | `185.35 ns` (🚀 **7.04x faster**)  | `13.44 ns` (🚀 **97.14x faster**)  | `9.01 ns` (🚀 **144.88x faster**)  |
| **`mixed addition`**                  | `N/A`                                | `N/A`                                | `912.10 ns` (✅ **1.00x**)              | `3.47 us` (❌ *3.80x slower*)           | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed subtraction`**               | `N/A`                                | `N/A`                                | `945.42 ns` (✅ **1.00x**)              | `3.52 us` (❌ *3.72x slower*)           | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                                | `N/A`                                | `597.68 ns` (✅ **1.00x**)              | `2.29 us` (❌ *3.83x slower*)           | `12.35 ns` (🚀 **48.41x faster**) | `149.37 ns` (🚀 **4.00x faster**)  | `7.20 ns` (🚀 **83.06x faster**)   | `5.80 ns` (🚀 **103.02x faster**)  |
| **`scalar multiplication`**           | `N/A`                                | `N/A`                                | `326.66 us` (✅ **1.00x**)              | `1.18 ms` (❌ *3.61x slower*)           | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `22.95 ns` (❌ *3.67x slower*)    | `115.62 ns` (❌ *18.47x slower*)   | `17.98 ns` (❌ *2.87x slower*)     | `6.26 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `275.50 ns` (❌ *6.21x slower*)   | `7.27 us` (❌ *164.00x slower*)    | `80.40 ns` (❌ *1.81x slower*)     | `44.35 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `241.79 ns` (❌ *6.74x slower*)   | `5.13 us` (❌ *142.88x slower*)    | `66.86 ns` (❌ *1.86x slower*)     | `35.90 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `15.18 us` (❌ *2.07x slower*)    | `27.75 us` (❌ *3.78x slower*)     | `14.82 us` (❌ *2.02x slower*)     | `7.34 us` (✅ **1.00x**)           |
| **`sum of products of size 2`**       | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `593.00 ns` (❌ *9.62x slower*)   | `14.91 us` (❌ *241.93x slower*)   | `121.92 ns` (❌ *1.98x slower*)    | `61.65 ns` (✅ **1.00x**)          |
| **`naive sum of products of size 2`** | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `582.04 ns` (❌ *6.48x slower*)   | `14.82 us` (❌ *165.12x slower*)   | `163.45 ns` (❌ *1.82x slower*)    | `89.77 ns` (✅ **1.00x**)          |
| **`addition with carry`**             | `7.60 ns` (✅ **1.00x**)              | `8.63 ns` (❌ *1.14x slower*)         | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction with borrow`**         | `8.75 ns` (✅ **1.00x**)              | `9.83 ns` (❌ *1.12x slower*)         | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication by 2`**             | `4.69 ns` (✅ **1.00x**)              | `4.87 ns` (✅ **1.04x slower**)       | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division by 2`**                   | `4.81 ns` (✅ **1.00x**)              | `4.81 ns` (✅ **1.00x faster**)       | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization

|                                          | `for bls12_377::g1projective`          | `for bls12_377::g2projective`          | `for bls12_377::fr`                | `for bls12_377::fq`                 | `for bls12_377::fq2`                | `for bls12_377::fq12`             |
|:-----------------------------------------|:---------------------------------------|:---------------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize compressed`**               | `157.22 ns` (✅ **1.00x**)              | `222.70 ns` (❌ *1.42x slower*)         | `28.85 ns` (🚀 **5.45x faster**)    | `55.35 ns` (🚀 **2.84x faster**)     | `109.71 ns` (✅ **1.43x faster**)    | `709.48 ns` (❌ *4.51x slower*)    |
| **`serialize uncompressed`**             | `208.19 ns` (✅ **1.00x**)              | `338.39 ns` (❌ *1.63x slower*)         | `28.84 ns` (🚀 **7.22x faster**)    | `55.30 ns` (🚀 **3.76x faster**)     | `109.78 ns` (🚀 **1.90x faster**)    | `708.96 ns` (❌ *3.41x slower*)    |
| **`deserialize compressed`**             | `316.30 us` (✅ **1.00x**)              | `1.08 ms` (❌ *3.41x slower*)           | `56.90 ns` (🚀 **5558.70x faster**) | `100.77 ns` (🚀 **3138.86x faster**) | `210.89 ns` (🚀 **1499.88x faster**) | `1.36 us` (🚀 **233.40x faster**)  |
| **`deserialize compressed unchecked`**   | `67.88 us` (✅ **1.00x**)               | `183.54 us` (❌ *2.70x slower*)         | `57.02 ns` (🚀 **1190.52x faster**) | `100.72 ns` (🚀 **674.00x faster**)  | `211.16 ns` (🚀 **321.48x faster**)  | `1.34 us` (🚀 **50.49x faster**)   |
| **`deserialize uncompressed`**           | `248.58 us` (✅ **1.00x**)              | `884.92 us` (❌ *3.56x slower*)         | `56.90 ns` (🚀 **4368.73x faster**) | `100.67 ns` (🚀 **2469.24x faster**) | `210.61 ns` (🚀 **1180.29x faster**) | `1.36 us` (🚀 **183.01x faster**)  |
| **`deserialize uncompressed unchecked`** | `232.32 ns` (✅ **1.00x**)              | `492.18 ns` (❌ *2.12x slower*)         | `56.84 ns` (🚀 **4.09x faster**)    | `100.66 ns` (🚀 **2.31x faster**)    | `211.39 ns` (✅ **1.10x faster**)    | `1.35 us` (❌ *5.80x slower*)      |

### msm

|        | `for bls12_377::g1projective`          | `for bls12_377::g2projective`           |
|:-------|:---------------------------------------|:--------------------------------------- |
|        | `2.42 s` (✅ **1.00x**)                 | `8.43 s` (❌ *3.48x slower*)             |

### squareroot

|                          | `for bls12_377::fr`          | `for bls12_377::fq`             | `for bls12_377::fq2`              |
|:-------------------------|:-----------------------------|:--------------------------------|:--------------------------------- |
| **`square root for qr`** | `31.16 us` (✅ **1.00x**)     | `67.35 us` (❌ *2.16x slower*)   | `182.51 us` (❌ *5.86x slower*)    |
| **`legendre for qr`**    | `10.96 us` (✅ **1.00x**)     | `31.37 us` (❌ *2.86x slower*)   | `31.78 us` (❌ *2.90x slower*)     |

### bitwise

|                               | `operations for bls12_377::fr::bigint`          | `operations for bls12_377::fq::bigint`           |
|:------------------------------|:------------------------------------------------|:------------------------------------------------ |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                         | `5.02 ns` (✅ **1.04x slower**)                   |
| **`from little-endian bits`** | `133.59 ns` (✅ **1.00x**)                       | `189.47 ns` (❌ *1.42x slower*)                   |
| **`from big-endian bits`**    | `133.57 ns` (✅ **1.00x**)                       | `189.60 ns` (❌ *1.42x slower*)                   |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)                         | `5.08 ns` (✅ **1.03x slower**)                   |
| **`equality`**                | `5.39 ns` (✅ **1.00x**)                         | `5.69 ns` (✅ **1.06x slower**)                   |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                         | `5.21 ns` (✅ **1.06x slower**)                   |

### conversions

|                   | `for bls12_377::fr`          | `for bls12_377::fq`              |
|:------------------|:-----------------------------|:-------------------------------- |
| **`from bigint`** | `40.62 ns` (✅ **1.00x**)     | `75.24 ns` (❌ *1.85x slower*)    |
| **`into bigint`** | `26.51 ns` (✅ **1.00x**)     | `46.82 ns` (❌ *1.77x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

