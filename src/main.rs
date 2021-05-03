enum Type {
    Int(i64),
    Float(f64),
    Boolean(bool)
}

fn print_type(t: Type) {
    match t {
        Type::Int(i) => println!("type is integer {}", i),
        Type::Float(_f) => println!("type is float {}", _f),
        Type::Boolean(_b) => println!("type is bool"),
    }
}

fn main() {
    print_type(Type::Int(4));
    print_type(Type::Boolean(false));
    print_type(Type::Float(4.0));
}
