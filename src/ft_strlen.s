global ft_strlen 
section .text

%include "args.s"

; pseudo code of the ft_strlen:
; 
; 	for (; ptr is not aligned; ptr++)
; 		if *ptr == 0;
; 			return original_ptr - ptr;
; 	while (NO_ZEROS(ptr)) ptr += 8;
; 	while (*ptr != 0) ptr++;
; 	return original_ptr - ptr

; size_t ft_strlen(char *s);
; Calling conventions: 
; 	- arg: rdi
ft_strlen:
	; define variables:
%define s_ptr _ARG1

%define w_ptr rsi
%define tmp  rdx
	; prelude
	push rbp
	mov rbp, rsp
	; end of prelude

	mov w_ptr, s_ptr ; make a copy into working ptr
	cmp _ARG1, 0
	jz .return_ptr
	
	; we want to align w_ptr to 16
	; we can cheat: check if w_ptr & 15 == 15, and then we will know that ++w_ptr will be aligned :)

	mov tmp, w_ptr
	and tmp, 0xF

.align_loop_start:
	cmp BYTE [w_ptr], 0
	je .return_ptr
	cmp tmp, 0xF
	je .aligned_before
	inc w_ptr
	inc tmp
	jmp .align_loop_start

.aligned_before:
	inc w_ptr

.aligned:
	movdqa    xmm0, [w_ptr]
	pxor      xmm1, xmm1         ; xmm1 = 0s
	pcmpeqb   xmm1, xmm0         ; compare each byte in xmm0 to 0, result: 0xFF if equal, 0x00 if not
	pmovmskb  eax, xmm1          ; move MSBs of each byte in xmm2 into eax (16-bit result)
	test      eax, eax           ; test if any bit is set
	jnz       .find_zero         ; at least one byte was zero
	add       w_ptr, 16          ; in the 16 bytes we checked (size of xmm0), none were zeros
	jmp       .aligned

.find_zero:
	cmp BYTE [w_ptr], 0
	je .return_ptr
	inc w_ptr
	jmp .find_zero

.return_ptr:
	sub w_ptr, s_ptr
	mov _RET, w_ptr
	; postlude ?
	pop rbp
	ret
	; end of postlude
