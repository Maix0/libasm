segment .note.GNU-stack
segment .text
  global ft_atoi_base
  extern _ft_strchr
  extern ft_strlen

%include "registers.mac.s"
%include "args.mac.s"
%include "bonus/list_struct.mac.s"


; int ft_atoi_base(char *s, char *base);
; s    => rdi => r3
; base => rsi => r4

%define BASE_LEN r12
%define BASE     r13
%define STR      r14
%define OUT      r15


ft_atoi_base:
  push rbp
  mov rbp, rsp
  push r12 ; r12-r15 is callee saved
  push r13
  push r14
  push r15
  mov STR, _ARG1
  mov BASE, _ARG2
  xor _RET, _RET


  ; if (str == 0)  return 0
  ; if (base == 0) return 0
  cmp STR, 0
  je .end
  cmp BASE, 0
  je .end
  
  ; if ((BASE_LEN = _ft_check_base(base)) < 2) return 0;
  mov _ARG1, BASE
  call _ft_check_base
  mov BASE_LEN, _RET
  xor _RET, _RET
  cmp BASE_LEN, 2
  jb .end

.skip_whitespace_cond:
  xor r4b, r4b
  mov r4b, [STR]
  cmp r4b, 0
  je .skip_whitespace_end
  lea _ARG1, [rel spaces_chars]
  call _ft_strchr wrt ..plt
  cmp _RET, 0
  je .skip_whitespace_end
  inc STR
  jmp .skip_whitespace_cond
.skip_whitespace_end:

  mov r15, 1
.sign_cond:
  xor r4b, r4b
  mov r4b, [STR]
  cmp r4b, 0
  je .sign_end
  lea _ARG1, [rel sign_chars]
  call _ft_strchr wrt ..plt
  cmp _RET, 0
  je .sign_end
  xor r4b, r4b
  mov r4b, [STR]
  inc STR
  sub r4, ','
  neg r4
  movsx _RET, r4b
  imul r15, _RET
  jmp .sign_cond
.sign_end:

  push r15
  push r15
  xor OUT, OUT
.main_loop_cond:
  xor r4b, r4b
  mov r4b, [STR]
  cmp r4b, 0
  je .main_loop_end
  mov _ARG1, BASE
  call _ft_strchr wrt ..plt
  cmp _RET, 0
  je .main_loop_end
  imul OUT, BASE_LEN
  sub _RET, BASE
  add OUT, _RET
  inc STR
  jmp .main_loop_cond
.main_loop_end:
  pop _RET
  pop _RET; == sign
  imul eax, r15d ; (int) _RET * (int) OUT
.end:
  pop r15
  pop r14
  pop r13
  pop r12 
  pop rbp
  ret




; unsigned long check_base(char *base)
; will check that the base is good:
; strlen(base) >= 2
; no illegal chars
; no duplicated chars
_ft_check_base:
  push rbp
  mov rbp, rsp
  push r12 ; r12-r15 is callee saved
  push r13
  push r14
  
  mov BASE, _ARG1
  call ft_strlen wrt ..plt
  mov BASE_LEN, _RET
  cmp _RET, 2
  jb .end ; if BASE_LEN < 2, we return direclty

  ; r14 == idx
  xor r14, r14
.check_base_cond:
  cmp r14, BASE_LEN
  jnb .check_base_after
.check_base_body:
  ; if (strchr(illegal_chars, *base) != 0) return 0
  xor r4b, r4b
  mov r4b, [BASE]
  lea _ARG1, [rel illegal_chars]
  call _ft_strchr
  mov _ARG1, _RET
  mov _RET, 0
  cmp _ARG1, 0
  jne .end
  
  ; if (strchr(base + 1, *base) != 0) return 0
  xor r4b, r4b
  mov r4b, [BASE]
  mov _ARG1, BASE
  inc _ARG1
  call _ft_strchr
  mov _ARG1, _RET
  mov _RET, 0
  cmp _ARG1, 0
  jne .end
  
  ; base++
  ; idx++
  inc BASE
  inc r14
  jmp .check_base_cond
.check_base_after:
  mov _RET, BASE_LEN
.end:
  pop r14
  pop r13
  pop r12
  pop rbp
  ret

segment .rodata
  illegal_chars: db " ", 0x09, 0x0A, 0x0B, 0x0C, 0x0D, "+-", 0
  spaces_chars:  db " ", 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0
  sign_chars:    db "+-", 0
