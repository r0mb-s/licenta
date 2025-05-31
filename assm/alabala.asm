section .data
    newline db 0xA     ; newline character

section .bss
    buffer resb 12     ; space to store string representation of number

section .text
    global _start

_start:
    mov ecx, 1          ; counter = 1

print_loop:
    cmp ecx, 11         ; if counter == 11, end loop
    je end

    mov eax, ecx        ; move current number into eax
    call print_eax      ; print number in eax

    inc ecx             ; counter++
    jmp print_loop

end:
    ; Exit program
    mov eax, 1          ; sys_exit
    xor ebx, ebx
    int 0x80

; ------------------------------------------------------------
; print_eax: prints the number in eax as decimal + newline
; ------------------------------------------------------------
print_eax:
    push ecx             ; Save the original value of ecx (the loop counter)
    push edx             ; Save edx as it's used and modified

    ; Convert eax to string
    mov edi, buffer + 11 ; point to end of buffer
    mov byte [edi], 0    ; null-terminate string

    mov ebx, 10

.convert_loop:
    dec edi
    xor edx, edx         ; Clear edx for division (remainder)
    div ebx              ; divide eax by 10, quotient in eax, remainder in edx
    add dl, '0'          ; convert digit to ASCII
    mov [edi], dl
    test eax, eax
    jnz .convert_loop

    ; Write string to stdout
    mov eax, 4           ; sys_write
    mov ebx, 1           ; stdout
    mov ecx, edi         ; address of string (this is where the converted number starts)
    mov edx, buffer + 11 ; Calculate length
    sub edx, edi
    int 0x80

    ; Write newline
    mov eax, 4
    mov ebx, 1
    mov ecx, newline
    mov edx, 1
    int 0x80

    pop edx              ; Restore edx
    pop ecx              ; Restore the original ecx (loop counter)
    ret