segment .text
	global ft_strlen
	

%include "args.mac.s"

; size_t ft_strlen(char *s);
ft_strlen:
; define variables:
%define SRC_PTR _ARG1

%define WORK_PTR rsi
%define _TMP_ rdx
; prelude
        push rbp
        mov rbp, rsp
; end of prelude

        mov WORK_PTR, SRC_PTR          ; make a copy into working ptr
        cmp _ARG1, 0
        je .return_ptr

; we want to align WORK_PTR to 16
; we can cheat: check if WORK_PTR & 15 == 15, and then we will know that ++WORK_PTR will be aligned :)

        mov _TMP_, WORK_PTR
        and _TMP_, 0xF

.align_loop_start:
        cmp BYTE [WORK_PTR], 0
        je .return_ptr
        cmp _TMP_, 0xF
        je .aligned_before
        inc WORK_PTR
        inc _TMP_
        jmp .align_loop_start

.aligned_before:
        inc WORK_PTR

.aligned:
        movdqa xmm0, [WORK_PTR]
        pxor xmm1, xmm1                ; xmm1 = 0s
        pcmpeqb xmm1, xmm0             ; compare each byte in xmm0 to 0, result: 0xFF if equal, 0x00 if not
        pmovmskb eax, xmm1             ; move MSBs of each byte in xmm2 into eax (16-bit result)
        test eax, eax                  ; test if any bit is set
        jnz .find_zero                 ; at least one byte was zero
        add WORK_PTR, 16               ; in the 16 bytes we checked (size of xmm0), none were zeros
        jmp .aligned

.find_zero:
        cmp BYTE [WORK_PTR], 0
        je .return_ptr
        inc WORK_PTR
        jmp .find_zero

.return_ptr:
        sub WORK_PTR, SRC_PTR
        mov _RET, WORK_PTR
; postlude ?
        pop rbp
        ret
; end of postlude
