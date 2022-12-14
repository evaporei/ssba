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

In Skylake's architecture, it can take up to 42 CPU cycles (+ 51 ns) to get something from RAM! Some other architectures get up to 240 CPU cycles.

- [x] https://www.7-cpu.com/cpu/Skylake.html

## Further Resources

- [x] Fetch Decode Execute Cycle in more detail: https://www.youtube.com/watch?v=jFDMZpkUWCw
- [ ] Richard Feymann lecture: https://www.youtube.com/watch?v=EKWGGDXe5MA
- [ ] Computer Science 61C - 2015-01-20: https://archive.org/details/ucberkeley_webcast_gJJeUFyuvvg (55:51 onwards)
- [x] Casey (Molly Rocket): "Simple Code" Follow-up Part 1: A (Very) Simplified CPU Diagram https://www.youtube.com/watch?v=8VakkEFOiJc

## Notes

Main points learned:

- `mov` instruction is turing complete https://github.com/xoreaxeaxeax/movfuscator
- x86 has compact instruction encoding (eg: one instruction could take 2, 3, 1 byte[s])
- arm has fixed size instruction encoding (because it's more modern and computers have more space I guess)
- OS somehow copies register state right after pausing each process/program, so it can: debug (eg gdb/lldb) + context switch
- `rax` vs `eax`, one is the full thing, not sure which. Fun fact `e` stands for `extended` https://keleshev.com/eax-x86-register-meaning-and-history/
