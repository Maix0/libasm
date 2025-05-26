segment .note.GNU-stack
segment .text
  global ft_list_push_front
  extern ft_list_new

%include "args.mac.s"
%include "bonus/list_struct.mac.s"

; void ft_list_push_front(t_list **begin_list, void *data);

%define _BEGIN_LIST [rsp]
%define _DATA_PTR   [rsp + 8]

ft_list_push_front:
  push rbp
  mov rbp, rsp
  sub rsp, 32
 
  cmp _ARG1, 0 ; is begin_list == 0
  je .end
  mov _BEGIN_LIST, _ARG1
  mov _DATA_PTR, _ARG2
  
  ; setting up to call ft_list_new(data)
  mov _ARG1, _ARG2
  call ft_list_new wrt ..plt
  cmp _RET, 0
  je .end

  mov _ARG1, _BEGIN_LIST
  mov _ARG2, [_ARG1]               ; _ARG2 == *begin_list == what we want to be the next node
  mov [_RET + t_list.next], _ARG2  ; _RET->next = *begin_list
  mov [_ARG1], _RET                ; *begin_list = _RET

.end:
  add rsp, 32
  pop rbp
  ret
; end of prelude
