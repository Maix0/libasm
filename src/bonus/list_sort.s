segment .note.GNU-stack 
segment .text
  global ft_list_sort
  extern ft_list_swap
  
%include "args.mac.s"
%include "bonus/list_struct.mac.s"

; void ft_list_sort(t_list **begin_list, int (*cmp)());
; cmp returns <0 when (rhs <  lhs)
; cmp returns 0 when  (rhs == lhs)
; cmp returns >0 when (rhs >  lhs)
ft_list_sort:
        push rbp
        mov rbp, rsp
        xor _RET, _RET; set rax/_RET to 0
.loop:
        test _ARG1, _ARG1
        jz .end
        inc _RET
        mov _ARG1, [_ARG1 + t_list.next]
        jmp .loop
.end:
        pop rbp
        ret
; end of prelude
