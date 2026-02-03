segment .note.GNU-stack
segment .text
  global ft_atoi_base
  global _ft_strchr
  extern ft_strlen

%include "registers.mac.s"
%include "args.mac.s"

; char *_ft_strchr(char *s, char c)
; _ARG1 = rdi => r3
; _ARG2 = rsi (8bit) => sil => r4b
_ft_strchr:
	push rbp
	mov rbp, rsp

	xor _RET, _RET
	; if (s == NULL)
	cmp _ARG1, 0
	je .end_not_found
.loop:
	mov r2b, [_ARG1]
	cmp r2b, r4b
	je .end_found
	inc _ARG1
	cmp r2b, 0
	je .end_not_found
	jmp .loop
.end_found:
	mov _RET, _ARG1

.end_not_found:
	pop rbp
	ret

