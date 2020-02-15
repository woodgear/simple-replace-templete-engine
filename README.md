# feature
Give the variables you want to have when rendering

# how to use
```rust
    
    fn test_get_variable() {
        use crate::get_variables;
        let vars = get_variables("hello _t_name_t_", "_t_");
        assert_eq!(vars, vec!["name".to_owned()]);
    }

    fn test_replace() {
        use crate::render;
        use std::collections::HashMap;

        let map = {
            let mut map = HashMap::new();
            map.insert("name".to_owned(), "me".to_owned());
            map
        };
        let templete_str = "hello _t_name_t_".to_owned();

        let ret = render(&templete_str, "_t_", &map).unwrap();
        assert_eq!(ret, "hello me".to_owned());
    }

```