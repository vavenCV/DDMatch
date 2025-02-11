
#[macro_export]
macro_rules! get_field_names {
    ($struct_name:ident) => {
        {
            let struct_fields = get_fields!($struct_name);
            let field_names = struct_fields
                .iter()
                .map(|field| stringify!(#field))
                .collect::<Vec<_>>()
                .join(", ");

            field_names
        }
    };
}
#[macro_export]
macro_rules! get_fields {
    ($struct_name:ident) => {
        {
            let struct_type = std::any::type_name::<$struct_name>();
            if struct_type.starts_with("core::option::Option<") {
                let variant_type = struct_type.trim_start_matches("core::option::Option<").trim_end_matches(">");
                if variant_type == std::any::type_name::<u64>() {
                    // return vec![stringify!($struct_name)];
                }
            }

            let struct_ref: &$struct_name = &Default::default();
            let fields_iter = struct_ref.iter();
            fields_iter.cloned().collect::<Vec<_>>()
        }
    };
}
#[macro_export]
macro_rules! get_value_placeholders {
    ($struct_name:ident) => {
        {
            let struct_fields = get_fields!($struct_name);
            let value_placeholders = struct_fields
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(", ");

            value_placeholders
        }
    };
}
#[macro_export]
macro_rules! get_binds {
    ($struct_name:ident) => {
        {
            let struct_fields = get_fields!($struct_name);
            let bind_methods: Vec<String> = struct_fields
                .iter()
                .map(|field| format!(".bind(&self.{})", stringify!(#field)))
                .collect();

            bind_methods.join("")
        }
    };

    ($struct_name:ident, $parameter:expr) => {
        {
            let struct_fields = get_fields!($struct_name);
            let bind_methods: Vec<String> = struct_fields
                .iter()
                .enumerate()
                .map(|(index, field)| format!(".bind(&{}.{})", $parameter, stringify!(#field)))
                .collect();

            bind_methods.join("")
        }
    };
}
#[macro_export]
macro_rules! generate_binds {
    ($struct_name:ident) => {
        {
            let struct_fields = get_fields!($struct_name);
            let bind_calls: Vec<String> = struct_fields
                .iter()
                .map(|field| format!(".bind(&self.{})", stringify!(#field)))
                .collect();

            bind_calls.join("\n")
        }
    };

    ($struct_name:ident, $parameter:expr) => {
        {
            let struct_fields = get_fields!($struct_name);
            let bind_calls: Vec<String> = struct_fields
                .iter()
                .enumerate()
                .map(|(index, field)| format!(".bind(&{}.{})", $parameter, stringify!(#field)))
                .collect();

            bind_calls.join("\n")
        }
    };
}

#[macro_export]
macro_rules! generate_insert_query {
    ($struct_name:ident, $table_name:expr) => {
        {
            let query_template = format!(
                "let result = sqlx::query(
                    \"INSERT INTO {} ({}) VALUES ({})\",
                )
                .{}{}",
                $table_name,
                get_field_names!($struct_name),
                get_value_placeholders!($struct_name),
                get_binds!($struct_name),
                generate_binds!($struct_name)
            );

            query_template
        }
    };

    ($struct_name:ident, $table_name:expr, $parameter:expr) => {
        {
            let query_template = format!(
                "let result = sqlx::query(
                    \"INSERT INTO {} ({}) VALUES ({})\",
                )
                .{}{};",
                $table_name,
                get_field_names!($struct_name),
                get_value_placeholders!($struct_name),
                get_binds!($struct_name, $parameter),
                generate_binds!($struct_name, $parameter),
            );

            query_template
        }
    };
}
