#!/bin/bash

./rusthon test.pll
nasm -f elf64 -g -F dwarf out.asm -o out.o
ld -o out out.o
./out
