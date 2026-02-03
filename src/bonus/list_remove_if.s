segment .note.GNU-stack
segment .text
  global ft_list_remove_if
  extern free

%include "args.mac.s"
%include "bonus/list_struct.mac.s"

%define STACK_SIZE     128       ;   16 ptr
; stack rsp
; 8  [cursor]   : t_list *
; 16 [next]     : t_list *
; 24 [fake_head]: t_list
; 32 -----------: t_list
; 40 [free_func]: void (*free_func)(void*)
; 48 [cmp]      : void (*cmp)(void*, void*)
; 56 [passthru] : void *
; 64 [begin]    : t_list **
; -- end of stack

%define next      rbp - 16
%define fake_head rbp - 24
%define free_func rbp - 40
%define cmp_func  rbp - 48
%define passthru  rbp - 56
%define begin     rbp - 64

%define cursor_reg r12
%define next_reg   r13

; void ft_list_remove_if(t_list **begin_list, void *passthru, int (*cmp)(void *data, void *passthru), void (*free_func)(void *data))
ft_list_remove_if:
  push rbp
  mov rbp, rsp
  push r12 ; r12-r15 is callee saved
  push r13
  push r14
  push r15
  sub rsp, STACK_SIZE
  
  test _ARG3, _ARG3
  je .end
  test _ARG1, _ARG1
  je .end

  mov       [begin],                    _ARG1
  mov       _ARG1,                     [_ARG1]
  mov       [fake_head + t_list.next],  _ARG1
  mov       [passthru],                 _ARG2
  mov       [cmp_func],                 _ARG3
  mov       [free_func],                _ARG4

  lea       cursor_reg,                 [fake_head]
  mov       next_reg,                   [cursor_reg + t_list.next]

  ; now we actually do the thingy

.while_cond:
; cursor->next != NULL
  mov next_reg, [cursor_reg + t_list.next]
  cmp next_reg, 0
  ; here:
  ; cursor_reg = cursor
  ; next_reg = cursor->next
  jz .while_after

.while_body:
; if (cmp(next->data, passthru) == 0)
  mov _ARG1, [next_reg + t_list.data]
  mov _ARG2, [passthru]
  call [cmp_func]
  cmp eax, 0
  jz .remove_element

; == 0
; cursor = next;
  mov cursor_reg, next_reg
  jmp .while_cond


.remove_element:
  ; cursor->next = next->next;
  mov _ARG2, [next_reg + t_list.next]
  mov [cursor_reg + t_list.next], _ARG2
	
  cmp qword [free_func], 0
  jz .skip_free_elem
  ; call free_func(next->data);
  mov _ARG1, [next_reg + t_list.data]
  call [free_func]
.skip_free_elem:
  mov _ARG1, next_reg
  call free wrt ..plt

  jmp .while_cond
.while_after:
; *begin = fake_head.next;
  mov _ARG1, [begin]
  mov _ARG2, [fake_head + t_list.next]
  mov [_ARG1], _ARG2

.end:
  add rsp, STACK_SIZE
  pop r15
  pop r14
  pop r13
  pop r12
  xor _RET, _RET
  pop rbp
.dummy_free:
  ret
