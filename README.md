# Shinzou Assembler

## What is this?

This project is an assembler simulating the [IAS Machine](https://en.wikipedia.org/wiki/IAS_machine). I learned about it in a class on computer architecture and decided to take on a challenge.

## Requirements

The executable and the assembly code _or_ the machine code if you are up to the task. Note that the assembly code must have the `.asm` extension while a binary code must have the `.exe` extension.

### Optional parameters

**A memory file.** A 4096 bit long file (1024 words × 40 bits), which amounts to 512 bytes. Not sure if you can run DOOM with that, but it's enough for some fancy low level calculations. If none is provided when invoking the program, it will be automatically generated at `~/Documents/ias.mem`. You can provide a preffered path using either the `-m` or `--memory` arguments.

**A memory dump file.** The memory file will always be reset to null at the end of execution. Even if you manage catch a snapshot of it, it will be a binary file. If you want to see the final state of memory, add either the `-d` or `--dump` arguments followed by the path to the text file.

_Fot the future:_ add a custom directive to output the whole state of memory at runtime, sort of like debugging.

_For the future:_ show a visualization for the working machine and its components.

**An I/O stream.** While the IAS spec has an I/O module, it does not list a right API for accessing it - at least I didn't find it ;-;. As this is my project, I will be adding a brand new instruction:

| Representação | Código   | Descição                                                          |
| ------------- | -------- | ----------------------------------------------------------------- |
| OUT M(X)      | 10000001 | Outputs the contents of the memory address of X to the I/O stream |

The I/O stream will be set to stdout by default (your terminal), but it can be set to a file using either the `-s` or `--stream` arguments followed by the path to the file.

## How to run

```sh
shinzou code.asm -m ~/Desktop/log/mem.mem
```
