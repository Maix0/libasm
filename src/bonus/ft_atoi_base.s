segment .note.GNU-stack
segment .text
  global ft_atoi_base
  extern ft_list_new

%include "args.mac.s"
%include "bonus/list_struct.mac.s"

; int ft_atoi_base(char *s, char *base);

; stub
ft_atoi_base:
	xor rax, rax
	ret
