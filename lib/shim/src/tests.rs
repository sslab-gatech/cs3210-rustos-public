mod assert {
    #[test]
    fn test_const_assert() {
        struct S1 (u8);
        const_assert_size!(S1, 1);
        S1(1);
        
        struct S2 (u16, u16);
        const_assert_size!(S2, 2+2);
        S2(2, 2);
    }
}