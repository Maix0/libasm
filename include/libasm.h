/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   libasm.h                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/18 22:37:39 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/20 18:15:22 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#pragma once
#include <stdint.h>

uint64_t ft_strlen(char *s);
char *ft_strcpy(char *dest, char *src);
void *ft_memcpy(void *dest, void *src, uint64_t n);
char *ft_strdup(char *str);
