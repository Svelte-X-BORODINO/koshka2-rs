; SOON(MAYBE).
.pos $05000
.entry main

puts:
    push $1
.loop:
    ldb ax adv
    cmp ax $00
    gz .done
    invoke 1
    goto .loop

main:
    mov adv hw_msg
    gsub puts
    hlt

hw_msg: .db "Hello, KRSII World!", 0

.pos $53FE
.dw $E4EE