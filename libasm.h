#ifndef LIBASM_H
#define LIBASM_H

#ifndef BONUS
#	define BONUS 0
#endif

typedef unsigned long size_t;
typedef signed long	  ssize_t;

size_t ft_strlen(const char* s);
int	   ft_strcmp(const char* s1, const char* s2);
char*  ft_strcpy(char* dst, const char* src);
char*  ft_strdup(const char* s);

ssize_t ft_read(int fd, void* buf, size_t nbytes);
ssize_t ft_write(int fd, const void* buf, size_t nbytes);

#if BONUS

typedef struct s_list t_list;
struct s_list {
		void*	data;
		t_list* next;
};
typedef int	 (*cmp)(void*, void*);
typedef void (*free_func)(void*);

int	 ft_atoi_base(char* str, char* base);
void ft_list_push_front(t_list** begin_list, void* data);
int	 ft_list_size(t_list* begin_list);
void ft_list_sort(t_list** begin_list, cmp cmp);
void ft_list_remove_if(t_list** begin_list, void* data_ref, cmp cmp, free_func free_fct);

#endif

#endif /* LIBASM_H */
