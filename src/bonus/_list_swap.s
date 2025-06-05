segment .note.GNU-stack
segment .text
  global ft_list_swap
  extern malloc

%include "args.mac.s"
%include "bonus/list_struct.mac.s"

; void ft_list_swap(t_list **lhs, t_list **rhs)

%define _LHS  [rsp + 8 ]
%define _RHS  [rsp + 16]

%define _LNXT [rsp + 24]
%define _RNXT [rsp + 32]

%define _LCPY [rsp + 48]
%define _RCPY [rsp + 64]


; void ft_list_swap(t_list *lhs, t_list *rhs);
ft_list_swap:
  push rbp
  mov rbp, rsp
  ; prelude end

; (lhs == NULL || rhs == NULL) => return;
  cmp     _ARG1, 0
  je      .ret
  cmp     _ARG2, 0
  je      .ret
; (*lhs == *rhs)
  cmp     _ARG1, _ARG2
  je      .ret

  mov     r8, [_ARG1 + t_list.data]
  mov     r9, [_ARG2 + t_list.data]
  mov     [_ARG1 + t_list.data], r9
  mov     [_ARG2 + t_list.data], r8
.ret:
  pop rbp
  ret
