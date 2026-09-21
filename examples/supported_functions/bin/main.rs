use supported_functions::*;

fn main() {
    loop {
        let result = get_values();
        let formatted_result: String = result
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("[{formatted_result}]");
        std::thread::sleep(core::time::Duration::from_millis(2000));
    }
}

fn get_values() -> [i32; 31] {
    [
        standalone_get(),
        Struct.get(),
        Struct.mut_get(),
        Struct.consume_get(),
        Box::new(Struct).box_get(),
        Struct::static_get(),
        Struct.trait_get(),
        Struct.trait_mut_get(),
        Struct.trait_consume_get(),
        Box::new(Struct).trait_box_get(),
        Struct::trait_static_get(),
        Enum::Variant.get(),
        Enum::Variant.mut_get(),
        Enum::Variant.consume_get(),
        Box::new(Enum::Variant).box_get(),
        Enum::static_get(),
        Enum::Variant.trait_get(),
        Enum::Variant.trait_mut_get(),
        Enum::Variant.trait_consume_get(),
        Box::new(Enum::Variant).trait_box_get(),
        Enum::trait_static_get(),
        Union::new().get(),
        Union::new().mut_get(),
        Union::new().consume_get(),
        Box::new(Union::new()).box_get(),
        Union::static_get(),
        Union::new().trait_get(),
        Union::new().trait_mut_get(),
        Union::new().trait_consume_get(),
        Box::new(Union::new()).trait_box_get(),
        Union::trait_static_get(),
    ]
}
