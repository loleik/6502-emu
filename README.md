# 6502-emu
Emulator and disassembler for the MOS Technology 6502 microprocessor

Usage: `lolei_6502 <COMMAND>` or `cargo run -- <COMMAND>`

Commands:
* `disassemble`  Disassemble binaries
    * `<PATH>` Path to the target binary
    * `<START>` Start address of the binary
* `emulate`     Emulate 6502 - Starts a fake shell, run `help` within it for the help message
* `help`         Print this message or the help of the given subcommand(s)

Options:
* `-h`, `--help`  Print help

All opcodes and addressing modes are now implemented, next is debugging which is quite daunting to be honest.

## Notes
This is not an emulator of any specific system, so things like the amount of memory may seem strange and not align with any specific system. I may use this as a core for some other emulator later but this repository is just for 6502 emulation.

Some resources I am finding helpful:
* [Nesdev 6502 Reference](https://www.nesdev.org/obelisk-6502-guide/), this has resources for instructions, address modes, flags, etc.
* [Masswerk 6502 Instruction Reference](https://www.masswerk.at/6502/6502_instruction_set.html), similar to the Nesdev resource but written by someone else. NOTE: The documentation for the DEC instruction here contradicts other sources I've been using. I've followed the more commonly documented version, which is decrementing memory using zero page, zero page x, absolute, and absolute x addressing. This site seems to say DEC is for decrementing the accumulator.
* [6502 Wikipedia Article](https://en.wikipedia.org/wiki/MOS_Technology_6502), bit obvious I know.
* [6502.org Forum](http://forum.6502.org/), the website as a whole is helpful, but particularly the forum.
* [6502_65C02_functional_tests](https://github.com/Klaus2m5/6502_65C02_functional_tests), a repository of test binaries for the 6502, 65C02, etc.

## Possible Issues
These are things that I am worried about, but for now they may be fine. This list is mainly for me to have places to look if things go wrong later.
* `SBC` and `ADC` functions. Specifically overflow and carry flag handling.
* Memory addressing order.
* Program counter behaviour in BCS.
* Break flag behaviour.
* Reordered status bits to be in line with documentation.
* BCD not yet implemented. I've tried a bit but will come back to it instead.