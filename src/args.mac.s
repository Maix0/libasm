; this is a file that is to be included, and provides %define for the calling conventions

default rel

%if __?NASM_VER?__ > 2 && __?NASM_MINOR?__ >= 16
[warning -reloc-rel]
%endif


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
