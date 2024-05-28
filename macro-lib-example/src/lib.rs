// Define the macro
#[macro_export]
macro_rules! generate_getters_setters {
    // The macro takes a struct name and its fields
    ($struct_name:ident { $($field_name:ident: $field_type:ty),* }) => {


        // Define a struct. This expands to:
        //
        //     pub struct S {
        //         a: a_type,
        //         b: b_type,
        //         c: c_type,
        //     }
        pub struct $struct_name {
            $(
                $field_name: $field_type,
            )*
        }

        impl $struct_name {
            // Generate a hello method for the struct
            pub fn hello(&self) {
                println!("Hello, I'm {}!", self.name);
            }
        }
    }
}
