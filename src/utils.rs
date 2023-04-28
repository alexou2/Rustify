pub fn type_of<T>(_: &T) {
    let type_of_var = format!("{}", std::any::type_name::<T>());
    println!("{}", type_of_var)
}