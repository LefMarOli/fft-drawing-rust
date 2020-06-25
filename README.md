FFT Drawing project

Utilities to draw any closed path described by a set of coordinates using the Fast Fourier Transform algorithm.

Fast Fourier Transform implementation was taken from http://www.librow.com/articles/article-10 and adapeted to RUST.

Estimated curve can be drawned using a subset n of FFT components. The most contributing components are used first (a sorting is performed on the output of the FFT).
