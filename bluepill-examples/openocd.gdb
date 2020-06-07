target remote :3333
set print asm-demangle on
monitor arm semihosting enable

# detect unhandled exceptions, hard faults and panics
load
break DefaultHandler
break HardFault
break rust_begin_unwind

# set a breakpoint on main and lets go
break main
continue
