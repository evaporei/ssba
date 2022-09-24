# programming-actual-computer

## linux

```
sudo apt-get install gcc make nasm

nasm -felf64 hello_linux.asm && ld hello_linux.o && ./a.out
```

## macos

```
xcode-select --install

brew install nasm

nasm -fmacho64 hello_mac.asm && ld -L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib -lSystem hello_mac.o && ./a.out
```
