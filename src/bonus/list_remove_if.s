segment .note.GNU-stack
segment .text
  global ft_list_remove_if
  extern free

%include "args.mac.s"
%include "bonus/list_struct.mac.s"

%define SSIZE    64         ;   8 ptr
%define BEGIN    [rsp + 0]  ;   t_list **begin;
%define PASSTHRU [rsp + 8] ;   void *passthru
%define FHEAD    [rsp + 16] ;   t_list fake_head.next;
%define FDATA    [rsp + 32] ;   t_list fake_head.data;
%define NTMP     [rsp + 48] ;   t_list *tmp;

%define CURSOR r12 ; t_list     *cursor
%define CMP    r13 ; cmp_func   cmp;
%define FFUNC  r14 ; free_func  ffunc;
%define NEXT   r15 ; t_list     *next;

free_dummy:
	ret

; void ft_list_remove_if(t_list **begin_list, void *passthru, int (*cmp)(void *data, void *passthru), void (*free_func)(void *data))
ft_list_remove_if:
  push rbp
  mov rbp, rsp
  push r12 ; r12-r15 is callee saved
  push r13
  push r14
  push r15
  sub rsp, SSIZE
  ; do the checks
  cmp _ARG1, 0 ; begin_list == 0 ?
  je .end
  
  cmp _ARG3, 0 ; cmp == NULL
  je .end

  ;cmp _ARG4, 0 ; free_func == 0
  ;jne .skip_dummy
  ;mov _ARG4, free_dummy
;.skip_dummy:
  ; all checks are good !

  mov BEGIN,    _ARG1
  mov rax,      [_ARG1]
  mov FHEAD,    rax
  mov PASSTHRU, _ARG2
  lea CURSOR,   FHEAD
  mov CMP,      _ARG3
  mov FFUNC,    _ARG4

; while (cursor->next)
.while_cond:
	mov NEXT, [CURSOR + t_list.next]
	cmp NEXT, 0
	je .after_while

.while_body:

; if (cmp(next->data, passthru) == 0)
.if_cond:
	mov _ARG1, [NEXT + t_list.data]
	mov _ARG2, PASSTHRU
	call CMP
	cmp eax, 0 
	jne .else
; then
.if_body:
	mov NTMP, NEXT
	mov NEXT, [NEXT + t_list.next]
	mov [CURSOR + t_list.next], NEXT
	mov NEXT, NTMP
	cmp FFUNC, 0
	je .skip_ffunc
	mov _ARG1, [NEXT + t_list.data]
	call FFUNC
.skip_ffunc:
	mov _ARG1, NEXT
	call free wrt ..plt
	jmp .while_end
; else
.else:
	mov CURSOR, [CURSOR + t_list.next] ; cursor = cursor->next

.while_end:
	jmp .while_cond;
.after_while:
	mov _ARG1, BEGIN
	mov _ARG2, FHEAD
	mov [_ARG1], _ARG2
.end:
  add rsp, SSIZE
  pop r15
  pop r14
  pop r13
  pop r12
  xor _RET, _RET
  pop rbp
  ret
