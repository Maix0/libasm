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
%define HEAD     r14 ; t_list *
%define ANY_MOVE r15 ; bool


; void ft_list_sort(t_list **begin, int (*cmp)(void *, void *));
; basic code:
; do the check here:
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

  mov LIST_PTR, [_ARG1]
  mov CMP_FUNC, _ARG2

  jmp .do_while_body

.do_while_cond: 
	cmp ANY_MOVE, 0
	jz .end

.do_while_body:
	xor ANY_MOVE, ANY_MOVE
	mov HEAD, LIST_PTR

.while_cond:
	mov _ARG1, HEAD
	cmp _ARG1, 0
	je .while_after

.while_body:
	; head == NULL
	mov _ARG1, HEAD
	cmp _ARG1, 0
	je .while_after

	; head->next == NULL
	mov _ARG2, [HEAD + t_list.next]
	cmp _ARG2, 0
	je .while_after

	mov _ARG1, [_ARG1 + t_list.data]
	mov _ARG2, [_ARG2 + t_list.data]

	call CMP_FUNC
	test eax, eax
	jle .after_swap
	
	mov ANY_MOVE, 1
	mov _ARG3, [HEAD + t_list.data] ; getting head->data into _ARG3
	
	mov _ARG1, [HEAD + t_list.next]  ; getting head->next->data into _ARG4
	mov _ARG4, [_ARG1 + t_list.data] ; ditto

	mov [HEAD + t_list.data], _ARG4  ; swapping
	mov [_ARG1 + t_list.data], _ARG3 ; swapping
.after_swap:
	mov HEAD, [HEAD + t_list.next]
	jmp .while_cond

.while_after:
.do_while_end:
	jmp .do_while_cond

.end:
  pop r15
  pop r14
  pop r13
  pop r12
  pop rbp
  ret
; end of prelude
