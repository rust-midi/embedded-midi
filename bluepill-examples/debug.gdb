target remote :3333
set print asm-demangle on
monitor arm semihosting enable

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind

# flash our code onto the stm32 and reset
load

# set a breakpoint on main and start the program
break main
continue
