section .data
a_len dd 0
i dd 0
j dd 0
aux dd 0
newline db 0xA

section .bss
a resd 10
buffer resb 12

section .text
global _start

_start:
mov eax, 10
mov [a_len], eax
mov eax, 0
mov [i], eax
mov eax, 0
mov [j], eax
mov eax, 0
mov [aux], eax
lea esi, [a]
mov eax, 0
mov ecx, eax
mov eax, 87
mov [esi + ecx*4], eax
lea esi, [a]
mov eax, 1
mov ecx, eax
mov eax, 103
mov [esi + ecx*4], eax
lea esi, [a]
mov eax, 2
mov ecx, eax
mov eax, 21
mov [esi + ecx*4], eax
lea esi, [a]
mov eax, 3
mov ecx, eax
mov eax, 16
mov [esi + ecx*4], eax
lea esi, [a]
mov eax, 4
mov ecx, eax
mov eax, 32
mov [esi + ecx*4], eax
lea esi, [a]
mov eax, 5
mov ecx, eax
mov eax, 76
mov [esi + ecx*4], eax
lea esi, [a]
mov eax, 6
mov ecx, eax
mov eax, 2
mov [esi + ecx*4], eax
lea esi, [a]
mov eax, 7
mov ecx, eax
mov eax, 13
mov [esi + ecx*4], eax
lea esi, [a]
mov eax, 8
mov ecx, eax
mov eax, 334
mov [esi + ecx*4], eax
lea esi, [a]
mov eax, 9
mov ecx, eax
mov eax, 9
mov [esi + ecx*4], eax
mov eax, 0
mov [i], eax
while_start_0:
mov eax, 1
push eax
mov eax, [a_len]
pop ebx
sub eax, ebx
push eax
mov eax, [i]
pop ebx
cmp eax, ebx
jge while_end_1
mov eax, [i]
lea esi, [a]
mov eax, [esi + eax*4]
call print_eax
mov eax, 1
push eax
mov eax, [i]
pop ebx
add eax, ebx
mov [i], eax
jmp while_start_0
while_end_1:
mov eax, 0
mov [i], eax
while_start_2:
mov eax, 1
push eax
mov eax, [a_len]
pop ebx
sub eax, ebx
push eax
mov eax, [i]
pop ebx
cmp eax, ebx
jge while_end_3
mov eax, 1
push eax
mov eax, [i]
pop ebx
add eax, ebx
mov [j], eax
while_start_4:
mov eax, [a_len]
push eax
mov eax, [j]
pop ebx
cmp eax, ebx
jge while_end_5
mov eax, [j]
lea esi, [a]
mov eax, [esi + eax*4]
push eax
mov eax, [i]
lea esi, [a]
mov eax, [esi + eax*4]
pop ebx
cmp eax, ebx
jle endif_6
mov eax, [i]
lea esi, [a]
mov eax, [esi + eax*4]
mov [aux], eax
lea esi, [a]
mov eax, [i]
mov ecx, eax
mov eax, [j]
lea esi, [a]
mov eax, [esi + eax*4]
mov [esi + ecx*4], eax
lea esi, [a]
mov eax, [j]
mov ecx, eax
mov eax, [aux]
mov [esi + ecx*4], eax
endif_6:
mov eax, 1
push eax
mov eax, [j]
pop ebx
add eax, ebx
mov [j], eax
jmp while_start_4
while_end_5:
mov eax, 1
push eax
mov eax, [i]
pop ebx
add eax, ebx
mov [i], eax
jmp while_start_2
while_end_3:
mov eax, 99999
call print_eax
mov eax, 0
mov [i], eax
while_start_7:
mov eax, [a_len]
push eax
mov eax, [i]
pop ebx
cmp eax, ebx
jge while_end_8
mov eax, [i]
lea esi, [a]
mov eax, [esi + eax*4]
call print_eax
mov eax, 1
push eax
mov eax, [i]
pop ebx
add eax, ebx
mov [i], eax
jmp while_start_7
while_end_8:

mov eax, 1
xor ebx, ebx
int 0x80
print_eax:
push ecx
push edx
mov edi, buffer + 11
mov byte [edi], 0
mov ebx, 10
.convert_loop:
dec edi
xor edx, edx
div ebx
add dl, '0'
mov [edi], dl
test eax, eax
jnz .convert_loop
mov eax, 4
mov ebx, 1
mov ecx, edi
mov edx, buffer + 11
sub edx, edi
int 0x80
mov eax, 4
mov ebx, 1
mov ecx, newline
mov edx, 1
int 0x80
pop edx
pop ecx
ret

