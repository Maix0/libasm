; this is a file that is to be included, and provides %define for the calling conventions

default rel

%define __NEED_WARN_SILENCING \
	(__?NASM_MAJOR?__ == 2 && __?NASM_MINOR?__ >= 16) || \
	(__?NASM_MAJOR?__ >= 3)

%if  __NEED_WARN_SILENCING
; because this warning is more like a warning that you should be careful
; it will trigger even when nothing happens
[WARNING -reloc-rel-dword] 
; thanks to nasm 3.x, `struc/endstruc` produces some stuff that triggers this
[WARNING -unknown-warning]
%endif
%undef __NEED_WARN_SILENCING


%ifndef ARGS_MACROS

%define ARGS_MACROS
%define _ARG1 rdi
%define _ARG2 rsi
%define _ARG3 rdx
%define _ARG4 rcx
%define _ARG5 r8
%define _ARG6 r9

%define _RET rax

%endif
