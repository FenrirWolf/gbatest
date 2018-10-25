REM Run this any time crt0.s gets changed.
REM the `arm-none-eabi-as` is from devkitpro

arm-none-eabi-as crt0.s -o crt0.o
