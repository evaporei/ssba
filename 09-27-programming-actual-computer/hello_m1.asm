; ----------------------------------------------------------------------------------------
; Writes "Hello, World" to the console using only system calls. Runs on 64-bit m1 macOS only.
; To assemble and run:
;
;     xcrun -sdk macosx --show-sdk-path # /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk

;     as -o hello_m1.o hello_m1.asm

;     ld -macosx_version_min 12.0.0 -o hello_m1 hello_m1.o -lSystem -syslibroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk -e _main -arch arm64

;     ./hello_m1
;
; Derived from:
;   - https://cs.lmu.edu/~ray/notes/nasmtutorial/
;   - https://smist08.wordpress.com/2021/01/08/apple-m1-assembly-language-hello-world/
; ----------------------------------------------------------------------------------------

.global _main          ; provide program starting address to linker
.align 2

; setup the parameters to print hello world
; and then call the OS to do it.

_main: mov X0, #1      ; file handle 1 is stdout
       adr X1, message ; address of string to print
       mov X2, #13     ; length of our string, number of bytes
       mov X16, #4     ; MacOS write system call
       svc 0           ; invoke operating system to do the write

; setup the parameters to exit the program
; and then call the OS to do it.

       mov X0, #0      ; exit code 0
       mov X16, #1     ; MacOS exit system call (service command code 1 terminates this program)
       svc 0           ; invoke operating system to exit

message:      .ascii  "Hello World!\n"
