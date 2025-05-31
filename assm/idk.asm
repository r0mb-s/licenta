section .data
    msg db '5'      ; The character to print
    len equ 1       ; Length of the message

section .text
    global _start

_start:
    ; sys_write(int fd, const void *buf, size_t count)
    mov eax, 4      ; syscall number for sys_write
    mov ebx, 1      ; file descriptor 1 = stdout
    mov ecx, msg    ; pointer to the message
    mov edx, len    ; message length
    int 0x80        ; make the kernel call

    ; sys_exit(int status)
    mov eax, 1      ; syscall number for sys_exit
    xor ebx, ebx    ; exit code 0
    int 0x80