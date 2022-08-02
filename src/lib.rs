#[no_mangle]
pub extern "system" fn Java_com_rokoblox_iron_NativeLibLoader_hello_1world() {
    println!("Hello from rust!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
