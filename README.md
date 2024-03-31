# Shinzou Assembler

## What is this?

This project is an assembler simulating the [IAS Machine](https://en.wikipedia.org/wiki/IAS_machine). I learned about it in a class on computer architecture and decided to take on a challenge.

## How to run

```sh
shinzou run finnish.asm
# or
shinzou compile finnish.asm -o ancient_sumerian.bin
shinzou ancient_sumerian.bin
```

## For the future

- Add a custom directive to output the whole state of memory at runtime, sort of like debugging.
- Show a web visualization for the working machine and its components.

## Updated command sheet

While the IAS spec has an I/O module, it does not list a right API for accessing it - at least I didn't find it. I used for reference [this document from Unicamp](https://www.ic.unicamp.br/~edson/disciplinas/mc404/2012-1s/anexos/programando_o_IAS.pdf). As this is _my_ project, I will be adding brand new instructions:

| Representation | Code     | Description                                                                |
| -------------- | -------- | -------------------------------------------------------------------------- |
| OUT M(X)       | 10000001 | Outputs the contents of the memory address of X to the I/O stream          |
| CHAR M(X)      | 10000010 | Outputs the contents of the memory address of X as ASCII to the I/O stream |
