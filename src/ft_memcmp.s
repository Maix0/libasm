global ft_strlen 
section .text

%include "args.s"


%define S1     _ARG1
%define S2     _ARG2
%define LEN    _ARG3

%define XMM_SIZE 16

; int ft_memcmp(const void *s1, const void *s2, uint64_t);
ft_strcmp:
	push rbp
	mov rbp, rsp
	mov _RET, 0
	; end of prelude
	
	; pointers are the same, so memcmp is good. _RET already eq 0
	test S1, S2
	je .end

	; check S1 for NULL
	cmp S1, 0
	jnz .check_s2
	mov _RET, 255
	jump .end

.check_s2
	cmp S2, 0
	jnz .check
	mov _RET, 255
	jump .end


.check:
	cmp LEN, XMM_SIZE    ; check if at least 16 bytes to copy
	jge .last_check       ; jump if less than 16 bytes
	
	pxor      xmm4, xmm4 ; xmm4 = 0{16}
	movdqu    xmm0, [S1] ; xmm0 = S1[0:16]
	movdqu    xmm1, [S2] ; xmm0 = S2[0:16]
	psubb     xmm0, xmm1 ; xmm0 = xmm0 - xmm1 (bytewise)
	
	pcmpeqb   xmm4, xmm0 ; xmm4 = 0xFF if 0 else 0x00 (bytewise)
	pmovmskb  eax, xmm4  ; every MSB of xmm4 is set to eax
	not       eax        ; invert eax: we want to see if there was stuff that was != 0
	test      eax, eax   ; do the check
	
	jnz  .last_check ; aka something wasn't eq, do the check in .last_check

	; move everything by XMM_SIZE
	add       S1,  XMM_SIZE
	add       S2,  XMM_SIZE
	; dec LEN by XMM_SIZE
	sub       LEN, XMM_SIZE
	jmp       .check


; bytewise check one by one
.last_check:
; check the loop condition: LEN != 0
	cmp LEN, 0
	jz .end; jump to the end if LEN == 0
;end

;  get *S1 and *S2 into registers
	mov [S1], _RET
	mov [S2], r8
	sub _RET, r8 ; _RET = *S1 - *S2
	jnz .end     ; if _RET == 0 => jump to end
	inc S1       ; S1++
	inc S1       ; S2++
	dec LEN      ; LEN--
	jmp .last_check 

.end:
	pop rbp
	ret

