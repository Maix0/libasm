global ft_strcmp
section .text
extern ft_strlen

%include "args.s"


%define S1     [rsp + 8]
%define S2     [rsp + 16]
%define S1_LEN [rsp + 24]
%define S2_LEN [rsp + 32]

; int ft_strcmp(const char *s1, const char *s2);
ft_strcmp:
	push rbp
	mov rbp, rsp
	; end of prelude
	sub rsp, 32
	mov S1, _ARG1
	mov S2, _ARG2
	call ft_strlen


	

