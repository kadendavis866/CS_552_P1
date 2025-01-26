#[cfg(test)]
mod test {
    use crate::List;

    fn int_cmp(a: &i32, b: &i32) -> i32 {
        if *a == *b {
            0
        } else if *a < *b {
            -1
        } else {
            1
        }
    }

    fn int_destroy(_: &i32) {}

    fn string_cmp(a: &String, b: &String) -> i32 {
        if *a == *b {
            0
        } else {
            1
        }
    }

    fn string_destroy(_: &String) {}

    fn populate_list() -> List<i32> {
        let mut lst = List::new(int_cmp, int_destroy);
        for i in 0..5 {
            lst.add(i);
        }
        lst
    }

    #[test]
    fn test_create_destroy() {
        let lst = List::new(int_cmp, int_destroy);
        assert_ne!(lst.head, std::ptr::null_mut());
        assert_eq!(lst.len, 0);
        unsafe {
            assert_eq!((*lst.head).value, None);
            assert_ne!((*lst.head).next, std::ptr::null_mut());
            assert_ne!((*lst.head).prev, std::ptr::null_mut());
            assert_eq!((*lst.head).next, (*lst.head).prev);
        }
    }

    #[test]
    fn test_add1() {
        let mut lst = List::new(int_cmp, int_destroy);
        lst.add(1);
        assert_eq!(lst.len, 1);
        unsafe {
            assert_eq!((*lst.head).next, (*lst.head).prev);
            assert_ne!(lst.head, (*lst.head).next);
            assert_ne!(lst.head, (*lst.head).prev);
            assert_eq!((*lst.head).value, None);
            assert_eq!((*(*lst.head).next).value, Some(1));
            assert_eq!((*(*lst.head).prev).value, Some(1));
        }
    }

    #[test]
    fn test_add2() {
        let mut lst = List::new(int_cmp, int_destroy);
        lst.add(1);
        assert_eq!(lst.len, 1);
        lst.add(2);
        assert_eq!(lst.len, 2);
        unsafe {
            assert_ne!((*lst.head).next, (*lst.head).prev);
            assert_ne!((*lst.head).next, lst.head);
            assert_ne!((*lst.head).prev, lst.head);
            assert_eq!((*lst.head).value, None);
            assert_eq!((*(*lst.head).next).value, Some(2));
            assert_eq!((*(*lst.head).prev).value, Some(1));
        }
    }

    #[test]
    fn test_remove_index0() {
        let mut lst = populate_list();
        let rval = lst.remove_index(0);
        assert_eq!(lst.len, 4);
        assert_eq!(rval, Some(4));
        unsafe {
            let mut curr = (*lst.head).next;
            //List should be 3->2->1->0
            for i in (0..=3).rev() {
                assert_eq!((*curr).value, Some(i));
                curr = (*curr).next;
            }
            curr = (*lst.head).prev;
            for i in 0..=3 {
                assert_eq!((*curr).value, Some(i));
                curr = (*curr).prev;
            }
        }
    }

    #[test]
    fn test_remove_index3() {
        let mut lst = populate_list();
        let rval = lst.remove_index(3);
        assert_eq!(lst.len, 4);
        assert_eq!(rval, Some(1));
        unsafe {
            let mut curr = (*lst.head).next;
            //List should be 4->3->2->0
            for i in (2..=4).rev() {
                assert_eq!((*curr).value, Some(i));
                curr = (*curr).next;
            }
            //Check the last one
            assert_eq!((*curr).value, Some(0));
            //Set the curr back one node so we can check prev links
            curr = (*curr).prev;
            for i in 2..=4 {
                assert_eq!((*curr).value, Some(i));
                curr = (*curr).prev;
            }
        }
    }

    #[test]
    fn test_remove_index4() {
        let mut lst = populate_list();
        let rval = lst.remove_index(4);
        assert_eq!(lst.len, 4);
        assert_eq!(rval, Some(0));
        unsafe {
            let mut curr = (*lst.head).next;
            //List should be 4->3->2->1
            for i in (1..=4).rev() {
                assert_eq!((*curr).value, Some(i));
                curr = (*curr).next;
            }
            curr = (*lst.head).prev;
            for i in 1..=4 {
                assert_eq!((*curr).value, Some(i));
                curr = (*curr).prev;
            }
        }
    }

    #[test]
    fn test_invalid_index() {
        let mut lst = populate_list();
        let rval = lst.remove_index(666);
        assert_eq!(lst.len, 5);
        assert_eq!(rval, None);
        unsafe {
            let mut curr = (*lst.head).next;
            //List should be 4->3->2->1->0
            for i in (0..=4).rev() {
                assert_eq!((*curr).value, Some(i));
                curr = (*curr).next;
            }
            curr = (*lst.head).prev;
            for i in 0..=4 {
                assert_eq!((*curr).value, Some(i));
                curr = (*curr).prev;
            }
        }
    }

    #[test]
    fn test_remove_all() {
        let mut lst = populate_list();
        //List should be 4->3->2->1->0
        for i in (0..=4).rev() {
            let rval = lst.remove_index(0);
            assert_eq!(rval, Some(i));
        }
        //Make sure we back to default
        unsafe {
            assert_ne!((*lst.head).next, std::ptr::null_mut());
            assert_ne!((*lst.head).prev, std::ptr::null_mut());
            assert_eq!((*lst.head).next, (*lst.head).prev);
            assert_eq!((*lst.head).value, None);
        }
        assert_eq!(lst.len, 0);
    }

    #[test]
    fn test_index_of0() {
        let lst = populate_list();
        //List should be 4->3->2->1->0
        unsafe {
            let data = (*(*lst.head).next).value.as_ref().unwrap();
            assert_eq!(lst.index_of(data), Some(0));
        }
    }

    #[test]
    fn test_index_of3() {
        assert_eq!(populate_list().index_of(&1), Some(3));
    }

    #[test]
    fn test_not_in_list() {
        assert_eq!(populate_list().index_of(&22), None);
    }

    #[test]
    fn test_index_of_empty() {
        let lst = List::new(int_cmp, int_destroy);
        assert_eq!(lst.index_of(&1), None);
    }

    #[test]
    fn test_add_string() {
        let mut lst: List<String> = List::new(string_cmp, string_destroy);
        lst.add("hello".to_string());
        lst.add("world".to_string());
        unsafe {
            assert_eq!((*(*lst.head).next).value, Some("world".to_string()));
            assert_eq!((*(*lst.head).prev).value, Some("hello".to_string()));
        }
    }

    #[test]
    fn test_remove_string() {
        let mut lst: List<String> = List::new(string_cmp, string_destroy);
        lst.add("hello".to_string());
        lst.add("world".to_string());
        let rval = lst.remove_index(1);
        assert_eq!(rval, Some("hello".to_string()));
        unsafe {
            assert_eq!((*(*lst.head).next).value, Some("world".to_string()));
            assert_eq!((*(*lst.head).prev).value, Some("world".to_string()));
        }
    }

    #[test]
    fn test_index_of_string() {
        let mut lst: List<String> = List::new(string_cmp, string_destroy);
        lst.add("hello".to_string());
        lst.add("world".to_string());
        let mut data = "hello".to_string();
        assert_eq!(lst.index_of(&data), Some(1));
        data = "world".to_string();
        assert_eq!(lst.index_of(&data), Some(0));
        data = "not in list".to_string();
        assert_eq!(lst.index_of(&data), None);
    }
}
