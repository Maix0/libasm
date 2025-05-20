global ft_strcpy
section .text
extern ft_strlen
extern ft_memcpy

%include "args.s"

; char *ft_strcpy(char *dest, char *src);
; Calling conventions:
; 	- dest: _ARG1
; 	- src: _ARG2
; 	- @ret: _RET
ft_strcpy:
	; prelude
	push rbp
	mov rbp, rsp
	; end of prelude
	
	; check for nulls
	cmp _ARG1, 0
	jz .ret_dest
	cmp _ARG2, 0
	jz .ret_dest

	push _ARG2 ; we need to align the stack...
	; start of the actual function
	push _ARG2 ; src
	push _ARG1 ; dest
	; we call strlen(src)
	mov _ARG1, _ARG2
	call ft_strlen
	
	; call ft_memcpy
	pop _ARG1 ; dest
	pop _ARG2 ; source
	pop _ARG2 ; source (x2, remove out shit from the stack)
	mov _ARG3, _RET ; set count to arg3
	inc _ARG3
	pop rbp
	; we jump to memcpy
	; and since we didn't use call, the ret (of memcpy) will go back to the function calling us
	jmp ft_memcpy
.ret_dest:
	mov _RET, _ARG1
	pop rbp
	ret
