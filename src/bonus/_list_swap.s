segment .note.GNU-stack
segment .text
  global ft_list_swap
  extern malloc

%include "args.mac.s"
%include "bonus/list_struct.mac.s"

; use _ARG1 and _ARG2, both are call saved
; prototype: void swap_ptr(void **lhs, void** rhs)
; this macro does nothing if any of the arguments are NULL or if they point to the same value
%macro swap_ptr 0
	push r8
	push r9
	
	cmp _ARG1, _ARG2
	je %%skip
	cmp _ARG1, 0
	je %%skip
	cmp _ARG2, 0
	je %%skip
	mov r8, [_ARG1]
	mov r9, [_ARG2]

	mov [_ARG1], r9
	mov [_ARG2], r8
%%skip:
	pop r9
	pop r8
%endmacro


; void ft_list_swap(t_list **lhs, t_list **rhs);
ft_list_swap:
  push rbp
  mov rbp, rsp
  ; prelude end

  cmp     _ARG1, _ARG2 ; lhs == rhs
  je      .ret
  
  test    _ARG1, _ARG1 ; lhs == NULL
  je      .ret
  
  test    _ARG2, _ARG2 ; rhs == NULL
  je      .ret

  mov     rax, [_ARG1] ; rax = *lhs
  test    rax, rax     ; *lhs == NULL
  je      .ret
  
  mov     rcx, [_ARG2] ; rcx = *rhs
  test    rcx, rcx     ; *rhs == NULL
  je      .ret
  
  cmp     rax, rcx     ; *lhs == *rhs
  je     .ret

  
  mov     _ARG1, [rax + t_list.data] ; _ARG1 = (*lhs)->data
  mov     _ARG2, [rcx + t_list.data] ; _ARG2 = (*rhs)->data
  mov     [rax + t_list.data], _ARG2 ; (*lhs)->data = _ARG2
  mov     [rcx + t_list.data], _ARG1 ; (*rhs)->data = _ARG1

.ret:
  pop rbp
  ret
