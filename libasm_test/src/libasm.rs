/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   libasm.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/22 14:42:46 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/26 15:01:35 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// Here i am making an UGLY assumption:
// Option<Box<T>> == *mut T
// while in practice this should be true, there is nothing preventing the compiler from inserting
// some 124344278943 bytes before, making this false.
//
// Box isn't #[repr(transparent)] nor #[repr(C)], meaning that I can't make any assumption onto its
// layout.
//
// in practice, current Box is an (Unique<T>, Allocator) where Allocator = Global = ();
// since Global is a unit struct, it doesn't exists in memory, meaning that Box = (Unique<T>)
// This means that if T is a thin pointer, Box = (*mut T)
// If T is a fat pointer, then I can go fuck myself, but this should be caught by the const asserts
//
// Yes I know I'm a bad person for saying that Option<Box<T>> == *mut T
// Sue me :)
#[repr(C)]
#[derive(Debug)]
pub struct List<T> {
    next: Option<Box<List<T>>>,
    data: Option<Box<T>>,
}

impl<T> List<T> {
    const _ASSERT_BOX_EQ_PTR: () = {
        assert!(size_of::<Option<Box<T>>>() == size_of::<*mut T>());
    };

    pub fn new(val: T) -> Box<Self> {
        Box::new(Self {
            next: None,
            data: Some(Box::new(val)),
        })
    }
    pub fn size(&self) -> usize {
        let mut i = 0;
        let mut cur = Some(self);
        while let Some(c) = cur {
            cur = c.next().map(|n| &**n);
            i += 1;
        }
        i
    }

    pub fn insert_after(&mut self, val: T) {
        let old_next = self.next.take();
        self.next = Some(Box::new(List {
            next: old_next,
            data: Some(Box::new(val)),
        }));
    }

    pub fn insert_before(self: &mut Box<Self>, new_begin: T) {
        let old_head = std::mem::replace(
            self,
            Box::new(List {
                next: None,
                data: Some(Box::new(new_begin)),
            }),
        );
        self.next = Some(old_head);
    }

    pub fn next(&self) -> Option<&Box<Self>> {
        self.next.as_ref()
    }

    pub fn data(&self) -> Option<&T> {
        self.data.as_deref()
    }
    pub fn next_mut(&mut self) -> Option<&mut Box<Self>> {
        self.next.as_mut()
    }

    pub fn data_mut(&mut self) -> Option<&mut T> {
        self.data.as_deref_mut()
    }
}

unsafe extern "C" {
    /* Mandatory */
    pub fn ft_strlen(s: *const u8) -> u64;
    pub fn ft_strcpy(dest: *mut u8, src: *const u8) -> *mut u8;
    pub fn ft_strdup(src: *const u8) -> *mut u8;
    pub fn ft_strcmp(lhs: *const u8, rhs: *const u8) -> i32;

    pub fn ft_read(fd: i32, data: *const (), count: u64) -> i64;
    pub fn ft_write(fd: i32, data: *mut (), count: u64) -> i64;

    /* Implemetation detail */
    pub fn ft_memcpy(dest: *mut u8, src: *const u8, n: u64) -> *mut u8;
    pub fn ft_memcmp(lhs: *const u8, rhs: *const u8, n: u64) -> i32;

}

// Bonus part
#[allow(improper_ctypes)]
unsafe extern "C" {
    // helper functions
    pub fn ft_list_new(data: *mut ()) -> *mut List<()>;

    // actual functions
    pub fn ft_list_size(lst: *const List<()>) -> i32;
    pub fn ft_list_push_front(lst: *mut *mut List<()>, data: *mut ());
}
