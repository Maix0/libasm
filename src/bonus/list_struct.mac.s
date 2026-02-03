%ifndef LIST_STRUCT_MACRO
%define LIST_STRUCT_MACRO

struc t_list
	.data: resq 1
	.next: resq 1
endstruc

%endif
