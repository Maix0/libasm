;test 
%ifndef REGISTERS_MAC_S

%define REGISTERS_MAC_S

; define better names for registers
%define r0 rax
%define r1 rbx
%define r2 rcx
%define r3 rdx
%define r4 rsi
%define r5 rdi
%define r6 rsp
%define r7 rbp


; define better names for registers (32bit part)
%define r0d eax
%define r1d ebx
%define r2d ecx
%define r3d edx
%define r4d esi
%define r5d edi
%define r6d esp
%define r7d ebp

; define better names for registers (16bit part)
%define r0w ax
%define r1w bx
%define r2w cx
%define r3w dx
%define r4w si
%define r5w di
%define r6w sp
%define r7w bp

; define better names for registers (8bit part)
%define r0b al
%define r1b bl
%define r2b cl
%define r3b dl
%define r4b sil
%define r5b dil
%define r6b spl
%define r7b bpl

%endif
