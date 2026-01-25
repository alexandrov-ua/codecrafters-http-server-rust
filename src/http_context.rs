pub struct HttpContext {
    path_params: std::collections::HashMap<String, String>,   
}

impl HttpContext {
    pub fn new() -> Self {
        HttpContext {
            path_params: std::collections::HashMap::new(),
        }
    }

    pub fn new_with_params(params: std::collections::HashMap<String, String>) -> Self {
        HttpContext {
            path_params: params,
        }
    }

    pub fn get_path_param(&self, key: &str) -> Option<&String> {
        self.path_params.get(key)
    }
}