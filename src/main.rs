// just creates a function
macro_rules! create_function {
    ($name:ident) => {
        fn $name() {
            println!("You called macro to create function {:?}", stringify!(name));
        }
    };
}

// just prints out the expression and its evaluated result
macro_rules! create_expression {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

pub trait HelloThere {
    fn say_hello(&self);
}
#[derive(Clone)]
pub struct Test {}
impl HelloThere for Test {
    fn say_hello(&self) {
        println!("HEllo from Test.");
    }
}

#[derive(Clone)]
pub struct YetAnotherTest {}
impl HelloThere for YetAnotherTest {
    fn say_hello(&self) {
        println!("Hello from YetAnotherTest.");
    }
}

// generates a function with some visibility, identifier, argument type and return type
macro_rules! create_better_function {
    ($visibility:vis,$name:ident; $arg_type:ty,$ret_type:ty) => {
        $visibility fn $name(arg: $arg_type) -> $ret_type{
            arg.say_hello();
            return true;
        }
    };
}
macro_rules! create_better_function_templace {
    ($visibility:vis,$name:ident; $ret_type:ty) => {
        $visibility fn $name<T: HelloThere>(arg: T) -> $ret_type{
            arg.say_hello();
            return true;
        }
    };
}
// generates a template function

fn main() {
    create_function!(some_finction);
    some_finction();

    create_expression!({
        println!("IM EXPRESSIINGGGG");
        10 + 2
    });
    let a = Test {};
    let b = YetAnotherTest {};

    create_better_function!(pub, say_hello_meta; Test, bool);
    println!("{}", say_hello_meta(a.clone()));
    // println!("{}", say_hello_meta(b)); - Error here as say_hello_meta takes argument of type Test
    create_better_function_templace!(pub, say_hello_template; bool);
    say_hello_template(a);
    say_hello_template(b);
}
