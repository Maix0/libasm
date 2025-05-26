segment .text
	global ft_memcmp
	

%include "args.mac.s"

%define S1 _ARG1
%define S2 _ARG2
%define LEN _ARG3

%define XMM_SIZE 16

; int ft_memcmp(const void *s1, const void *s2, uint64_t)

ft_memcmp:
        push rbp
        mov rbp, rsp
        mov _RET, 0
; end of prelude

; pointers are the same, so memcmp is good. _RET already eq 0
        cmp S1, S2
        je .end

; check S1 for NULL
        test S1, S1
        jne .check_s2
        mov ax, 256
        jmp .end

.check_s2:
        test S2, S2
        jne .check
        mov ax, -256
        jmp .end

.check:
        cmp LEN, XMM_SIZE              ; check if at least 16 bytes to copy
        jbe .last_check                ; jump if less than 16 bytes

        pxor xmm4, xmm4                ; xmm4 = 0{16}
        movdqu xmm0, [S1]              ; xmm0 = S1[0:16]
        movdqu xmm1, [S2]              ; xmm0 = S2[0:16]
        psubb xmm0, xmm1               ; xmm0 = xmm0 - xmm1 (bytewise)

        pcmpeqb xmm4, xmm0             ; xmm4 = 0xFF if 0 else 0x00 (bytewise)
        pmovmskb eax, xmm4             ; every MSB of xmm4 is set to eax
        cmp eax, 0xFFFF                ; do the check

        jne .last_check                ; aka something wasn't eq, do the check in .last_check

; move everything by XMM_SIZE
        add S1, XMM_SIZE
        add S2, XMM_SIZE
; dec LEN by XMM_SIZE
        sub LEN, XMM_SIZE
        jmp .check

; bytewise check one by one

.last_check:
; check the loop condition: LEN != 0
        cmp LEN, 0
        jz .end                        ; jump to the end if LEN == 0
; end

; get *S1 and *S2 into registers
		xor rax, rax
		xor r8, r8
        mov al , [S2]
        mov r8b, [S1]
        sub ax, r8w                   ; _RET = *S1 - *S2
		jnz .end                       ; if _RET == 0 => jump to end
        inc S1                         ; S1++
        inc S2                         ; S2++
        dec LEN                        ; LEN--
        jmp .last_check

.end:
        movsx rax, ax
        neg rax
.qend:
        pop rbp
        ret
