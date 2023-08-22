+++
title = "makefile"
date = 2023-09-22   
weight = 1
+++


A [Makefile](https://www.gnu.org/software/make/manual/html_node/Introduction.html) is a special file used in software development to define a set of build rules and dependencies for compiling and building programs or projects. It first appeared  in 1976 and did not changed since.

```
hellomake: hellomake.c hellofunc.c
     gcc -o hellomake hellomake.c hellofunc.c -I.
```
In this example, typing `make` will compile hellomake using GCC