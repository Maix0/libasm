%ifndef LIST_STRUCT_MACRO
%define LIST_STRUCT_MACRO

struc t_list
    .next: resq 1
    .data: resq 1
endstruc

%endif
