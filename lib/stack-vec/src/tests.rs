use crate::StackVec;

#[test]
fn assignment_text_example() {
    let mut storage = [0u8; 1024];
    let mut vec = StackVec::new(&mut storage);

    for i in 0..10 {
        vec.push(i * i).expect("can push 1024 times");
    }

    for (i, v) in vec.iter().enumerate() {
        assert_eq!(*v, (i * i) as u8);
    }

    let last_element = vec.pop().expect("has elements");
    assert_eq!(last_element, 9 * 9);
}

#[test]
fn len_and_capacity_ok() {
    let mut storage = [0u8; 1024];
    let stack_vec = StackVec::new(&mut storage);

    assert_eq!(stack_vec.len(), 0);
    assert_eq!(stack_vec.capacity(), 1024);
    assert!(stack_vec.is_empty());
    assert!(!stack_vec.is_full());
}

#[test]
#[should_panic]
fn index_oob() {
    let mut storage = [0u8; 1024];
    let stack_vec = StackVec::new(&mut storage);
    let _ = stack_vec[0];
}

#[test]
#[should_panic]
fn index_oob_after_truncate() {
    let mut storage = [0u8; 1024];
    let mut stack_vec = StackVec::new(&mut storage);
    stack_vec.push(10).expect("len > 0");
    stack_vec.truncate(0);
    let _ = stack_vec[0];
}

#[test]
fn indexing() {
    let mut storage = [0u8; 1024];
    let mut stack_vec = StackVec::new(&mut storage);
    assert!(stack_vec.is_empty());

    stack_vec.push(10).expect("cap = 1024");
    assert_eq!(stack_vec[0], 10);
    assert_eq!(stack_vec.len(), 1);
    assert_eq!(stack_vec.capacity(), 1024);
    assert!(!stack_vec.is_empty());

    stack_vec.push(2).expect("cap = 1024");
    assert_eq!(stack_vec[0], 10);
    assert_eq!(stack_vec[1], 2);
    assert_eq!(stack_vec.len(), 2);
    assert_eq!(stack_vec.capacity(), 1024);

    stack_vec.truncate(0);
    assert!(stack_vec.is_empty());
    assert_eq!(stack_vec.len(), 0);
    assert_eq!(stack_vec.capacity(), 1024);

    for i in 0..100 {
        stack_vec.push(i).expect("cap = 1024");
    }

    assert_eq!(stack_vec.len(), 100);
    for i in 0..100 {
        assert_eq!(stack_vec[i], i as u8);
    }
}

#[test]
fn mut_indexing() {
    let mut storage = [0u8; 1024];
    let mut stack_vec = StackVec::with_len(&mut storage, 3);

    assert_eq!(stack_vec[0], 0);
    assert_eq!(stack_vec[1], 0);
    assert_eq!(stack_vec[2], 0);

    stack_vec[0] = 100;
    stack_vec[1] = 88;
    stack_vec[2] = 99;

    assert_eq!(stack_vec[0], 100);
    assert_eq!(stack_vec[1], 88);
    assert_eq!(stack_vec[2], 99);

    stack_vec[0] = 23;
    assert_eq!(stack_vec[0], 23);

    stack_vec[0] = stack_vec[1];
    assert_eq!(stack_vec[0], 88);
}

#[test]
fn pop() {
    let mut storage = [0usize; 1024];
    let mut stack_vec = StackVec::new(&mut storage);
    assert!(stack_vec.pop().is_none());

    stack_vec.push(123).expect("cap = 1024");
    assert_eq!(stack_vec.len(), 1);
    assert_eq!(stack_vec.pop(), Some(123));

    for i in 0..1024 {
        assert_eq!(stack_vec.len(), i);
        stack_vec.push(i).expect("cap = 1024");
        assert_eq!(stack_vec.len(), i + 1);
    }

    for i in 1023..=0 {
        assert_eq!(stack_vec.len(), i + 1);
        assert_eq!(stack_vec.pop(), Some(i));
        assert_eq!(stack_vec.len(), i);
    }
}

#[test]
fn push_just_far_enough() {
    let mut storage = [0usize; 2];
    let mut stack_vec = StackVec::new(&mut storage);
    stack_vec.push(1).expect("okay");
    stack_vec.push(2).expect("okay");
    assert!(stack_vec.is_full());
}

#[test]
#[should_panic]
fn push_too_far() {
    let mut storage = [0usize; 2];
    let mut stack_vec = StackVec::new(&mut storage);
    stack_vec.push(1).expect("okay");
    stack_vec.push(2).expect("okay");
    stack_vec.push(3).expect("not okay");
}

#[test]
fn iterator() {
    let mut storage = [0usize; 1024];
    let mut stack_vec = StackVec::new(&mut storage);
    assert!(stack_vec.iter().next().is_none());

    stack_vec.push(123).expect("cap = 1024");
    assert_eq!(stack_vec.len(), 1);

    for _ in 0..10 {
        let mut iter = stack_vec.iter();
        assert_eq!(iter.next(), Some(&123));
        assert_eq!(iter.next(), None);
    }

    stack_vec.truncate(0);
    assert!(stack_vec.iter().next().is_none());

    for i in 0..1024 {
        stack_vec.push(i * i).expect("cap = 1024");
    }

    for (i, val) in stack_vec.iter().enumerate() {
        assert_eq!(*val, i * i);
    }

    let mut i = 0;
    for val in &stack_vec {
        assert_eq!(*val, i * i);
        i += 1;
    }

    let mut i = 0;
    for val in stack_vec {
        assert_eq!(*val, i * i);
        i += 1;
    }
}

#[test]
fn as_slice() {
    let mut storage = [0usize; 5];
    let mut stack_vec = StackVec::new(&mut storage);
    assert_eq!(stack_vec.as_slice(), &[]);

    stack_vec.push(102).expect("cap = 5");
    assert_eq!(stack_vec.as_slice(), &[102]);
    assert_eq!(stack_vec.as_mut_slice(), &mut [102]);

    stack_vec.push(1).expect("cap = 5");
    assert_eq!(stack_vec.as_slice(), &[102, 1]);
    assert_eq!(stack_vec.as_mut_slice(), &mut [102, 1]);

    assert_eq!(stack_vec.pop(), Some(1));
    assert_eq!(stack_vec.as_slice(), &[102]);
    assert_eq!(stack_vec.as_mut_slice(), &mut [102]);
}

#[test]
fn errors() {
    let mut storage = [0usize; 1024];
    let mut vec = StackVec::new(&mut storage);
    for i in 0..1024 {
        assert_eq!(vec.push(i), Ok(()));
    }
    for i in 0..1024 {
        assert_eq!(vec.push(i), Err(()));
    }
    for i in 1023..=0 {
        assert_eq!(vec.pop(), Some(i));
    }
    for _ in 1023..=0 {
        assert_eq!(vec.pop(), None);
    }
}
