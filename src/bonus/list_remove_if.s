segment .note.GNU-stack
segment .text
  global ft_list_remove_if
  extern free

%include "args.mac.s"
%include "bonus/list_struct.mac.s"

%define P_data   rbx

%define L_cursor r12
%define L_next   r13

%define F_cmp    r14
%define F_free   r15

;void ft_list_remove_if(t_list **begin_list, void *passthru, int (*cmp)(void *data, void *passthru), void (*free_func)(void *data))
ft_list_remove_if:
  push rbp
  mov rbp, rsp
  ; callee saved
  push r12
  push r13
  push r14
  push r15
  push rbx
  sub rsp, 40

; if (begin_list == NULL)
  test _ARG1, _ARG1
  je .end
; if (cmp == NULL)
  test _ARG3, _ARG3
  je .end

  mov P_data, _ARG2
  mov F_cmp,  _ARG3
  mov F_free, _ARG4

  ; begin = begin_list
  mov [rsp], _ARG1,
  ; fake_head.data = 0
  mov qword [rsp + 8 + t_list.data], 0

  ; fake_head.next = *begin_list
  mov rax, [_ARG1]
  mov [rsp + 8 + t_list.next], rax

  lea L_cursor, [rsp + 8]

.while_cond:
  mov L_next, [L_cursor + t_list.next]
  test L_next, L_next
  je .while_end

.while_body:
  ; call cmp(next->data, P_data)
  mov _ARG1, [L_next + t_list.data]
  mov _ARG2, P_data
  call F_cmp
  test _RET, _RET
  jne .while_next

  ; cursor->next = next->next
  mov rax, [L_next + t_list.next]
  mov [L_cursor + t_list.next], rax
; 
  test F_free, F_free
  jz .free_next_node
  mov _ARG1, [L_next + t_list.data]
  call F_free

.free_next_node:
; free(next)
  mov _ARG1, L_next
  call free wrt ..plt


  jmp .while_cond
.while_next:
  mov L_cursor, L_next
  jmp .while_cond

.while_end:
  ; get the begin ptr
  mov _ARG1, [rsp]
  ; get the fake_head->next
  mov _ARG2, [rsp + 8 + t_list.next]
  mov [_ARG1], _ARG2


.end:
  add rsp, 40
  ; callee saved
  pop rbx
  pop r15
  pop r14
  pop r13
  pop r12
  pop rbp
  ret
