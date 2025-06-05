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

%define LIST_PTR r12 ; == begin_list
%define CMP_FUNC r13 ; == cmp
%define CUR_LIST r14 ; t_list *
%define ANY_MOVE r15 ; bool

ft_list_sort:
  push rbp
  mov rbp, rsp
  push r12 ; r12-r15 is callee saved
  push r13
  push r14
  push r15
  
  ; if (begin_list == NULL) return
  cmp _ARG1, 0
  je .end
  
  ; if (*begin_list == NULL) return
  mov rax, [_ARG1]
  cmp rax, 0
  je .end

  ; if (cmp == NULL) return
  cmp _ARG2, 0
  je .end 
  
  mov LIST_PTR, _ARG1
  mov CMP_FUNC, _ARG2
.start_loop:
  mov CUR_LIST, [LIST_PTR]
  xor ANY_MOVE, ANY_MOVE
.loop:
  ; checking that 
  mov _ARG1, CUR_LIST
  cmp _ARG1, 0
  je .end_loop
  mov _ARG2, [CUR_LIST + t_list.next]
  cmp _ARG2, 0
  je .end_loop
  mov _ARG1, [_ARG1 + t_list.data]
  mov _ARG2, [_ARG2 + t_list.data]
  xor _RET, _RET
  call CMP_FUNC
  or ANY_MOVE, _RET ; if _RET != 0, then need to swap, then we'll have to loop again
  cmp _RET, 0
  je .after_swap
  ; perform the swap
  mov _ARG1, CUR_LIST
  mov _ARG2, [CUR_LIST + t_list.next]
  call ft_list_swap


.after_swap:
  mov CUR_LIST, [CUR_LIST + t_list.next]
  jmp .loop

.end_loop:
  cmp ANY_MOVE, 0
  jne .start_loop

.end:
  pop r15
  pop r14
  pop r13
  pop r12
  pop rbp
  ret
; end of prelude
