/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/18 22:46:50 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/20 17:25:29 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libasm.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *debug_string(const char *s, size_t n) {
  char *out = malloc(n * 4);
  assert(out != NULL);
  char *h = out;
  for (size_t i = 0; i < n; i++) {
    if (s[i] == '\0') {
      *(h++) = '\\';
      *(h++) = '0';
    } else if (s[i] == '\\') {
      *(h++) = '\\';
      *(h++) = '\\';
    } else if (s[i] == '\n') {
      *(h++) = '\\';
      *(h++) = 'n';
    } else if (s[i] > '~' || s[i] < ' ') {
      *(h++) = '\\';
      *(h++) = 'x';
      *(h++) = "0123456789ABCDEF"[(s[i] & 0xF0) >> 4];
      *(h++) = "0123456789ABCDEF"[(s[i] & 0x0F)];
    } else
      *(h++) = s[i];
  }
  *h = 0;
  return out;
}

#define DO_TEST(s)                                                             \
  do {                                                                         \
    char *str = strdup(s);                                                     \
    char mbuffer[1024];                                                        \
    char lbuffer[1024];                                                        \
    memset(mbuffer, '@', sizeof(mbuffer));                                     \
    memset(lbuffer, '@', sizeof(lbuffer));                                     \
    uint64_t l = ft_strlen(str);                                                  \
    ft_strcpy(mbuffer, str);                                            \
    strcpy(lbuffer, str);                                               \
    if (memcmp(mbuffer, lbuffer, 1024) == 0)                                   \
      printf("[\x1b[32mSUCCESS\x1b[0m]\n");                                    \
    else {                                                                     \
      char *ms = debug_string(mbuffer, sizeof(mbuffer));                       \
      char *ls = debug_string(lbuffer, sizeof(lbuffer));                       \
      printf("[\x1b[31mFAILURE\x1b[0m]\nMINE: '%s'\nLIBC:'%s'\n", ms, ls);     \
      free(ms);                                                                \
      free(ls);                                                                \
    };                                                                         \
    free(str);                                                                 \
  } while (0);

int main(void) { DO_TEST("some short text"); }
