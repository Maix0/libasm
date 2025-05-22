/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   libasm.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/22 14:42:46 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/22 14:48:57 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

unsafe extern "C" {
    /* Mandatory */
    pub fn ft_strlen(s: *const u8) -> u64;
    pub fn ft_strcpy(dest: *mut u8, src: *const u8) -> *mut u8;
    pub fn ft_strdup(src: *const u8) -> *mut u8;
    pub fn ft_strcmp(lhs: *const u8, rhs: *const u8) -> i32;

    /* Implemetation detail */
    pub fn ft_memcpy(dest: *mut u8, src: *const u8, n: u64) -> *mut u8;
    pub fn ft_memcmp(lhs: *const u8, rhs: *const u8, n: u64) -> i32;
}
