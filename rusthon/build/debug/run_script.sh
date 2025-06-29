#!/bin/bash

./rusthon test.pll
nasm -f elf64 out.asm
ld -o out out.o
./out
