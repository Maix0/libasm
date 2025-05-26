segment .text
  global ft_list_new
  extern malloc

%include "args.mac.s"
%include "bonus/list_struct.mac.s"

; t_list *ft_list_new(void *data)

ft_list_new:
  push rbp
  mov rbp, rsp
  sub rsp, 16
  mov [rsp], _ARG1
  
  mov _ARG1, 16 ; 16 == sizeof(t_list)
  call malloc wrt ..plt

  cmp _RET, 0
  je .is_null
  
  mov _ARG1, [rsp]
  mov QWORD [_RET + t_list.next], 0
  mov QWORD [_RET + t_list.data], _ARG1

.is_null:
  add rsp, 16
  pop rbp
  ret
