---
title: Everyone is better at development than me
subtitle: Creating a modular IDE
author: Nils Michael Fitjar
date: 2025-01-30
---

# Introduction

# Creating an IDE

## What is an IDE

Why `Beamer`?

>- It's nice.
>- It has incremental bullets.

## Why create a _new_ IDE

| コンピュータ | 年代 | メモリ | コア数 | クロック周波数 |
|:---:|:---|:---|:---|:---|
| AGC[^1] | 1960s | 4KB | 1 | 0.043MHz |
| P | 2010s | 6GB | 8 | 2.96GHz |


[^1]: https://history.nasa.gov/computers/Ch2-5.html

## Why modular?

Suppose $R$ is the radius of a sephere.

Then the volume can be calculated with the following formula:

$$V = \frac{4\pi}{3} R^3$$

# Features and Challenges

## Modularization

```hs
data Maybe a = Just a | Nothing
```

\begin{center}
  \emph{Hello, World!}
\end{center}

## Tech stack

\columnsbegin
\column{.5\textwidth}

\footnotesize

```haskell
-- code font size is smaller
-- with \footnotesize
class Functor f where
  fmap :: (a -> b) -> f a -> f b
  (<$) :: a -> f b -> f a
```

\normalsize

\column{.5\textwidth}

Related packages

- (base) Prelude
- (base) Data.Functor
- (base) Control.Monad

\columnsend

\extrafootnote{https://wiki.haskell.org/Functor}
\extrafootnote{Footnote without marker}

# Refactoring

## All my code sucks

FOOBAR

# Granularity and Module Families

## Granularity

GRANNY

## Module Families

Modularization Familiarization

# Conclusion

## I dont know

Call me Jon
