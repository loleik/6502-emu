# 6502-emu
Emulator and disassembler for the MOS Technology 6502 microprocessor

Usage: `lolei_6502 <COMMAND>` or `cargo run -- <COMMAND>`

Commands:
* `disassemble`  Disassemble binaries
    * `<PATH>` Path to the target binary
    * `<START>` Start address of the binary
* `emulate`     Emulate 6502
    * `<PATH>` Path to the target binary
    * `<LOAD>` Start address for storing the program in memory
    * `<EXEC>` Start address for execution
* `help`         Print this message or the help of the given subcommand(s)

Options:
* `-h`, `--help`  Print help

## Notes
This is not an emulator of any specific system, so things like the amount of memory may seem strange and not align with any specific system. I may use this as a core for some other emulator later but this repository is just for 6502 emulation.

I have put in a simple solution for reset vectors and such for now. Later I will probably write a proper machine language monitor or something, as it does sound  very interesting to do.