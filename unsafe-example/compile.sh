#!/bin/bash
gcc -c -fPIC add.c -o add.o
gcc -shared -o libadd.so add.o