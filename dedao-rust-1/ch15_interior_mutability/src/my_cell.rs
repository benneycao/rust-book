struct CellV1<T> {
    value: T,
}

impl<T> CellV1<T> {
    fn new(v: T) -> Self
    where
        T: Copy,
    {
        CellV1 { value: v }
    }

    fn set(&self, v: T)
    where
        T: Copy,
    {
        unsafe {
            let p = &(self.value) as *const T as *mut T;
            *p = v;
        }
    }
    fn get(&self) -> T
    where
        T: Copy,
    {
        self.value
    }
}
#[test]
#[allow(unused)]
fn test_cell() {
    let c = CellV1::new(10);
    let p = &c;
    p.set(2);
    eprintln!("p = {:?}", p.get());
}
