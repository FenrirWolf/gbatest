REM Some emulators can use cargo's output directly (which is cool, because you
REM can keep debug symbols and stuff), but to make a "real" ROM we have to
REM also use the devkitpro tools to package patch up the file a bit

cargo xbuild --target thumbv4-none-eabi
arm-none-eabi-objcopy -O binary target/thumbv4-none-eabi/debug/gbatest output.gba
gbafix output.gba
