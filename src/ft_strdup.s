segment .text
	global ft_strdup
	extern ft_strlen, ft_memcpy, malloc
	default rel

%include "args.s"

; char *ft_strdup(char *src);
; Calling conventions:
; - src: _ARG1
; - @ret: _RET
ft_strdup:
        push rbp
        mov rbp, rsp
        sub rsp, 32

; check for src == NULL
        cmp _ARG1, 0
        je .return_null
; end

; calling strlen(src)
        mov [rsp + 8], _ARG1
        call ft_strlen
        inc _RET
; end

; calling malloc(strlen(src) + 1)
        mov [rsp + 16], _RET
        mov _ARG1, _RET
        call malloc
; end

; check for malloc() == NULL
        cmp _RET, 0
        jz .return_null
; end

        mov _ARG2, [rsp + 8]
        mov _ARG3, [rsp + 16]
        mov _ARG1, _RET
        add rsp, 32

; tail call into ft_memcpy
        pop rbp
        jmp ft_memcpy

.return_null:
        mov _RET, 0
        add rsp, 32
        pop rbp
        ret
