// https://stackoverflow.com/questions/75785347/how-to-overload-a-rust-function-to-accept-closures-with-different-argument-signa/75789374#75789374
trait FunctionCall<O> {
    fn call_print(self, a: O, b: O) -> O;
}

trait FunctionComposition<O> {
    fn call_print(self, a: O, b: O) -> O;
}

impl<F, O> FunctionCall<O> for F
where
    F: FnOnce(O, O) -> O,
{
    fn call_print(self, a: O, b: O) -> O {
        self(a, b)
    }
}

impl<F, G, O> FunctionComposition<O> for F
where
    F: FnOnce(O) -> G,
    G: FnOnce(O) -> O,
{
    fn call_print(self, a: O, b: O) -> O {
        self(a)(b)
    }
}

macro_rules! printf {
    ($f:expr, $a:expr, $b:expr) => {
        println!("Result is {}", ($f).call_print($a, $b));
    };
}

fn function(a: i32, b: i32) -> i32 {
    a + b
}

fn func(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

fn main() {
    printf!(function, 22, 33);
    printf!(func, 33, 44);
}
