/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   build.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/22 14:40:41 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/22 14:48:33 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn main() {
    println!("cargo::rerun-if-changed=../src");
    println!("cargo::rustc-link-lib=asm");
    println!("cargo::rustc-link-search=native=../");
}
