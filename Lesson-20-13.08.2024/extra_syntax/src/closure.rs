#[test]
fn example() {
    let arr = [1, 2, 3, 4, 5];

    fn square(x: i32) -> i32 {
        x * x
    }

    fn is_even(x: &i32) -> bool {
        x % 2 == 0
    }

    let limit = 10;

    let res = arr
        .into_iter()
        .map(square)
        .filter(is_even)
        .filter(|x| *x < limit)
        .collect::<Vec<_>>();
    assert_eq!(&res, &[4]);
}

// struct Limit {
//     limit: i32,
// }

// impl FnOnce<(&i32,)> for Limit {
//     type Output = bool;

//     extern "rust-call" fn call_once(self, args: (&i32,)) -> Self::Output {
//         self.call(args)
//     }
// }

// impl FnMut<(&i32,)> for Limit {
//     extern "rust-call" fn call_mut(&mut self, args: (&i32,)) -> Self::Output {
//         self.call(args)
//     }
// }

// impl Fn<(&i32,)> for Limit {
//     extern "rust-call" fn call(&self, args: (&i32,)) -> Self::Output {
//         args.0 < &self.limit
//     }
// }
