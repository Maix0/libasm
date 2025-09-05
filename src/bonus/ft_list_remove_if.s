segment .note.GNU-stack
segment .text
  global ft_list_remove_if
  extern ft_list_new

%include "args.mac.s"
%include "bonus/list_struct.mac.s"

; int ft_list_remove_if(TODO);

; stub
ft_list_remove_if:
	xor rax, rax
	ret
