/// return the variable which inside input
/// # Remarks
/// str which format like {prefix}{var}{prefix} will be identified as variable
///!```
/// use simple_replace_templete_engine::get_variables;
/// let vars = get_variables("hello _t_name_t_","_t_");
/// assert_eq!(vars,vec!["name".to_owned()]);
///!```
pub fn get_variables(input: &str, prefix: &str) -> Vec<String> {
    use regex::Regex;
    use std::collections::HashSet;
    let regex = Regex::new(&format!(r#"(?m){}[^\s]*{}"#, prefix, prefix)).unwrap();
    let result = regex.find_iter(input);
    let mut vars = HashSet::new();
    for mat in result {
        let var = mat
            .as_str()
            .trim_start_matches(prefix)
            .trim_end_matches(prefix)
            .to_string();
        vars.insert(var);
    }
    return vars.into_iter().collect();
}

/// render templete with give map
///!```
/// use std::collections::HashMap;
/// use simple_replace_templete_engine::render;
///
/// let map = {
///     let mut map = HashMap::new();
///     map.insert("name".to_owned(), "me".to_owned());
///     map
/// };
/// let templete_str = "hello _t_name_t_".to_owned();
///
/// let ret = render(&templete_str, "_t_", &map).unwrap();
/// assert_eq!(ret,"hello me".to_owned());
///!```
pub fn render(
    input: &str,
    prefix: &str,
    map: &std::collections::HashMap<String, String>,
) -> Result<String, failure::Error> {
    use regex::Regex;
    let need_names = get_variables(input, prefix);
    for n in need_names.iter() {
        if map.get(n).is_none() {
            return Err(failure::format_err!("could not get variable {}", n));
        }
    }

    let mut input = input.to_string();
    for n in need_names.iter() {
        let regex_str = format!(r#"(?m){}{}{}"#, prefix, n, prefix);
        let regex = Regex::new(&regex_str).unwrap();
        let replace_str = map.get(n).unwrap();
        input = regex.replace_all(&input, replace_str.as_str()).to_string();
    }
    return Ok(input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_variable() {
        use crate::get_variables;
        let vars = get_variables("hello _t_name_t_", "_t_");
        assert_eq!(vars, vec!["name".to_owned()]);
    }
    #[test]
    fn test_replace() {
        use crate::render;
        use std::collections::HashMap;

        let map = {
            let mut map = HashMap::new();
            map.insert("name".to_owned(), "me".to_owned());
            map.insert("sec_name".to_owned(), "you".to_owned());
            map
        };
        let templete_str = "hello _t_name_t_ _t_sec_name_t_"_.to_owned();

        let ret = render(&templete_str, "_t_", &map).unwrap();
        assert_eq!(ret, "hello me you".to_owned());
    }
}
