/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/18 22:46:50 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/20 19:32:39 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libasm.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void test_strcpy() {

  char *s = "Some test that isn't just a.";
  char *out = malloc(ft_strlen(s) + 1);

  ft_strcpy(out, s);
  assert(strcmp(s, out) == 0);
  free(out);
}

int main(void) {
  char *s1 = "0123456789ABCDE";
  char *s2 = ft_strdup(s1);
  assert(strcmp(s1, s2) == 0);
  free(s2);
}
