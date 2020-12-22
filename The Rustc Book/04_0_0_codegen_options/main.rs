fn main() {
    println!("Hello codegen!");
}

/*
 *
    -C                       ar=val -- this option is deprecated and does nothing
    -C               code-model=val -- choose the code model to use (`rustc --print code-models` for details)
    -C            codegen-units=val -- divide crate into N units to optimize in parallel
    -C       control-flow-guard=val -- use Windows Control Flow Guard (default: no)
    -C         debug-assertions=val -- explicitly enable the `cfg(debug_assertions)` directive
    -C                debuginfo=val -- debug info emission level (0 = no debug info, 1 = line tables only, 2 = full debug info with variable and type information; default: 0)
    -C default-linker-libraries=val -- allow the linker to link its default libraries (default: no)
    -C            embed-bitcode=val -- emit bitcode in rlibs (default: yes)
    -C           extra-filename=val -- extra data to put in each output filename
    -C     force-frame-pointers=val -- force use of the frame pointers
    -C      force-unwind-tables=val -- force use of unwind tables
    -C              incremental=val -- enable incremental compilation
    -C         inline-threshold=val -- set the threshold for inlining a function
    -C                 link-arg=val -- a single extra argument to append to the linker invocation (can be used several times)
    -C                link-args=val -- extra arguments to append to the linker invocation (space separated)
    -C           link-dead-code=val -- keep dead code at link time (useful for code coverage) (default: no)
    -C      link-self-contained=val -- control whether to link Rust provided C objects/libraries or rely
        on C toolchain installed in the system
    -C                   linker=val -- system linker to link outputs with
    -C            linker-flavor=val -- linker flavor
    -C        linker-plugin-lto=val -- generate build artifacts that are compatible with linker-based LTO
    -C                llvm-args=val -- a list of arguments to pass to LLVM (space separated)
    -C                      lto=val -- perform LLVM link-time optimizations
    -C                 metadata=val -- metadata to mangle symbol names with
    -C    no-prepopulate-passes=val -- give an empty list of passes to the pass manager
    -C               no-redzone=val -- disable the use of the redzone
    -C           no-stack-check=val -- this option is deprecated and does nothing
    -C       no-vectorize-loops=val -- disable loop vectorization optimization passes
    -C         no-vectorize-slp=val -- disable LLVM's SLP vectorization pass
    -C                opt-level=val -- optimization level (0-3, s, or z; default: 0)
    -C          overflow-checks=val -- use overflow checks for integer arithmetic
    -C                    panic=val -- panic strategy to compile crate with
    -C                   passes=val -- a list of extra LLVM passes to run (space separated)
    -C           prefer-dynamic=val -- prefer dynamic linking to static linking (default: no)
    -C         profile-generate=val -- compile the program with profiling instrumentation
    -C              profile-use=val -- use the given `.profdata` file for profile-guided optimization
    -C         relocation-model=val -- control generation of position-independent code (PIC) (`rustc --print relocation-models` for details)
    -C                   remark=val -- print remarks for these optimization passes (space separated, or "all")
    -C                    rpath=val -- set rpath values in libs/exes (default: no)
    -C               save-temps=val -- save all temporary output files during compilation (default: no)
    -C               soft-float=val -- use soft float ABI (*eabihf targets only) (default: no)
    -C               target-cpu=val -- select target processor (`rustc --print target-cpus` for details)
    -C           target-feature=val -- target specific attributes. (`rustc --print target-features` for details). This feature is unsafe.
 * 
 */