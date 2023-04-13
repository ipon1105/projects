struct Rect{
    w: i32,
    h: i32,
}

impl Rect {
    fn can_hold(&self, obj: Rect) -> bool {
        self.w > obj.w && self.h > obj.h
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn work_done(){
        let str1 = Rect{w: 3, h: 5};
        let str2 = Rect{w: 2, h: 4};
        
        assert!(
            str1.can_hold(str2),
            "str1 can not hold str2",
        );
    }

    #[test] 
    fn work_panic(){
        // panic!("NOOOOOO");
    }

    #[test]
    #[should_panic]
    fn work_should_panic(){
        panic!("Yes");
    }

    #[test]
    fn work_equal() {
        let x = 3;
        let y = 3;

        // assert(x == y);
        assert_eq!(x, y);
    }

    #[test]
    fn work_not_equal() {
        let x = 3;
        let y = 4;

        // assert(x != y);
        assert_ne!(x, y);
    }

    #[test]
    fn work_result() -> Result<(), String>{
        // Ok(())
        Err("O неееет".to_string())
    }
}
