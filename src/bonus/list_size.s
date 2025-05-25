segment .text
  global ft_list_size
  
%include "args.mac.s"
%include "bonus/list_struct.mac.s"

; int ft_list_size(t_list *lst)

ft_list_size:
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
