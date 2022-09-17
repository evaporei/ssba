# Introduction

Exploring your architecture

## 1. Fetch-decode-execute cycle

a. fetch = get instruction from RAM into CPU (based of program counter)
b. decode = figure which instruction should be executed (it's parameters, etc). Also fetch needed data
c. execute = run the instruction (eg: pass it to ALU)

- [~] https://en.wikipedia.org/wiki/Instruction_cycle
- [x] https://www.brendangregg.com/blog/2014-04-26/the-noploop-cpu-benchmark.html
- [ ] https://www.agner.org/optimize/instruction_tables.pdf

## 2. Registers

32-bit computers can index up to 4GBs of RAM. That's because each register has the size of 32 bits, aka 4 bytes, aka 2^32, aka 4 billion numbers/addresses, that could represent a byte in computer RAM.

## 3. Memory access and caches

## Further Resources

- [x] Fetch Decode Execute Cycle in more detail: https://www.youtube.com/watch?v=jFDMZpkUWCw
- [ ] Richard Feymann lecture: https://www.youtube.com/watch?v=EKWGGDXe5MA
- [ ] Computer Science 61C - 2015-01-20: https://archive.org/details/ucberkeley_webcast_gJJeUFyuvvg (55:51 onwards)
- [x] Casey (Molly Rocket): "Simple Code" Follow-up Part 1: A (Very) Simplified CPU Diagram https://www.youtube.com/watch?v=8VakkEFOiJc
