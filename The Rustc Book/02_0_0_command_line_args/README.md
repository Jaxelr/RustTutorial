# Command Line Args for rustc

-h help

--cfg turn on/off various configs

--crate-type a crate type to build
    - lib
    - rlib
    - staticlib
    - dylib
    - cdylib
    - bin
    - proc-macro

--crate-name name of the crate

--edition specify the edition to use

--emit specificies the type of output files to generate
    - asm
    - dep-info
    - link
    - llvm-bc
    - llvm-ir
    - metadata
    - mir
    - obj

-g include debug information

-o output filename

--test build a test harness

--explain detail explanation for an error

-W set lint warnings

-A set lint allowed

-D set lint denied

-F set lint forbidden

-Z set unstable options

--cap-lints set the most restrictive lint level

-C code generation options

-V version

-v verbose

--sysroot override the system root

--error-format control how errors are produced

--color configure color of output

--remap-path-prefix remap source names in output

--json configure json messages printed by the compiler

