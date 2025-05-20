global ft_memcpy
section .text

%include "args.s"

; void *ft_memcpy(void *dest, void *src, uint64_t n);
; Calling conventions: 
; 	- dest: rdi
; 	- source: rsi
; 	- n: rdx
ft_memcpy:
; define variables:
%define dest_ptr      _ARG1
%define source_ptr    _ARG2
%define count         _ARG3

%define copy_buf      xmm0
%define dest_ptr_save rcx
%define tmp           r8b; only a single byte, located inside r8

%define COPY_SIZE 16

; rdi/rsi are the actuall arguments, meaning that we don't have to move anything...
; we try to keep as much as possible into the registers...
	; prelude
	push rbp
	mov rbp, rsp
	; end of prelude
	mov dest_ptr_save, dest_ptr
.copy:
	cmp count, COPY_SIZE ; check if at least 16 bytes to copy
	jge .last_copy       ; jump if less than 16 bytes
	
	movdqu    copy_buf, [source_ptr]
	movdqu    [dest_ptr], copy_buf
	add       source_ptr, COPY_SIZE
	add       dest_ptr, COPY_SIZE
	sub       count, COPY_SIZE
	jmp       .copy

.last_copy:
	cmp count, 0
	jz .return
	mov tmp, [source_ptr]
	mov BYTE [dest_ptr], tmp
	inc source_ptr
	inc dest_ptr
	dec count
	jmp .last_copy

.return:
	mov _RET, dest_ptr_save
	; postlude ?
	pop rbp
	ret
	; end of postlude
