segment .text
  global ft_write
  extern __errno_location

%include "args.mac.s"


; prototype: int64_t ft_read(int fd, void *buf[.count], uint64_t count)
ft_write:
   push rbp
   mov rbp, rsp
   mov    rax, 1
   ; system call 1 is write
   ; things are already in the correct registers...
   ; _ARG1 == fd
   ; _ARG2 == buf
   ; _ARG3 == len
   syscall
   cmp rax, 0 
   je .end
   neg rax
   mov r11, rax ; r11 is not clobbered by a call
   call __errno_location
   mov [rax], r11
   mov rax, r11
.end:
   pop rbp
   ret

