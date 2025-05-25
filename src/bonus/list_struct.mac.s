%ifndef LIST_STRUCT_MACRO
%define LIST_STRUCT_MACRO

struc t_list
    .next: resw 1
    .data: resw 1
endstruc

%endif
