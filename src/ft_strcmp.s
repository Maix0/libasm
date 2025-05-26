segment .text
	global ft_strcmp
	extern ft_strlen
  extern ft_memcmp
	

%include "args.mac.s"

%define S1 [rsp + 0]
%define S2 [rsp + 8]
%define S1_LEN [rsp + 16]
%define S2_LEN [rsp + 24]

; int ft_strcmp(const char *s1, const char *s2);
ft_strcmp:
        push rbp
        mov rbp, rsp
; end of prelude
        mov _RET, 0
; pointers are the same, so strcmp is good. _RET already eq 0
        test _ARG1, _ARG2
        jne .check_s1

.check_s1:
; check S1 for NULL
        cmp _ARG1, 0
        jne .check_arg2
        mov _RET, 256
		pop rbp
		ret

.check_arg2:
        cmp _ARG2, 0
        jne .end_check
        mov _RET, -256
		pop rbp
		ret
; end checks
.end_check:
        sub rsp, 32
        mov S1, _ARG1
        mov S2, _ARG2	
		; S1_LEN = strlen(S1) + 1
		call ft_strlen wrt ..plt
		inc _RET
		mov S1_LEN, _RET

		; S2_LEN = strlen(S2) + 1
		mov _ARG1, S2
		call ft_strlen wrt ..plt
		inc _RET
		mov S2_LEN, _RET

		mov _ARG1, S1_LEN
		mov _ARG2, S2_LEN

		cmp _ARG1, _ARG2
		cmova _ARG1, _ARG2
		mov _ARG3, _ARG1
		mov _ARG1, S1
		mov _ARG2, S2
		add rsp, 32
		pop rbp
		jmp ft_memcmp wrt ..plt
