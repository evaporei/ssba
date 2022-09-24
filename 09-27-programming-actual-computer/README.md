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

## m1

```
xcrun -sdk macosx --show-sdk-path # /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk

as -o hello_m1.o hello_m1.asm

ld -macosx_version_min 12.0.0 -o hello_m1 hello_m1.o -lSystem -syslibroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk -e _main -arch arm64

./hello_m1
```
