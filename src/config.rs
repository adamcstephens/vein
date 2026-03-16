use std::env;

#[derive(Debug)]
pub struct ConnectionConfig {
    pub vikunja_url: String,
    pub vikunja_api_token: String,
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub project_id: i64,
    pub view_id: i64,
    pub todo_bucket_id: i64,
    pub inprogress_bucket_id: i64,
    pub done_bucket_id: i64,
}

impl ConnectionConfig {
    pub fn from_env() -> Result<Self, String> {
        Self::load(|name| env::var(name).ok())
    }

    pub fn load(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let mut errors = Vec::new();

        let vikunja_url = read_string("VIKUNJA_URL", &lookup, &mut errors);
        let vikunja_api_token = read_string("VIKUNJA_API_TOKEN", &lookup, &mut errors);

        if !errors.is_empty() {
            return Err(format!("configuration errors:\n{}", errors.join("\n")));
        }

        Ok(ConnectionConfig {
            vikunja_url: vikunja_url.unwrap(),
            vikunja_api_token: vikunja_api_token.unwrap(),
        })
    }
}

impl ProjectConfig {
    pub fn from_env() -> Result<Self, String> {
        Self::load(|name| env::var(name).ok())
    }

    pub fn load(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let mut errors = Vec::new();

        let project_id = read_i64("VIKUNJA_PROJECT_ID", &lookup, &mut errors);
        let view_id = read_i64("VIKUNJA_VIEW_ID", &lookup, &mut errors);
        let todo_bucket_id = read_i64("VIKUNJA_TODO_BUCKET_ID", &lookup, &mut errors);
        let inprogress_bucket_id = read_i64("VIKUNJA_INPROGRESS_BUCKET_ID", &lookup, &mut errors);
        let done_bucket_id = read_i64("VIKUNJA_DONE_BUCKET_ID", &lookup, &mut errors);

        if !errors.is_empty() {
            return Err(format!("configuration errors:\n{}", errors.join("\n")));
        }

        Ok(ProjectConfig {
            project_id: project_id.unwrap(),
            view_id: view_id.unwrap(),
            todo_bucket_id: todo_bucket_id.unwrap(),
            inprogress_bucket_id: inprogress_bucket_id.unwrap(),
            done_bucket_id: done_bucket_id.unwrap(),
        })
    }
}

fn read_string(
    name: &str,
    lookup: &impl Fn(&str) -> Option<String>,
    errors: &mut Vec<String>,
) -> Option<String> {
    match lookup(name) {
        Some(val) => Some(val),
        None => {
            errors.push(format!("  {name}: missing"));
            None
        }
    }
}

fn read_i64(
    name: &str,
    lookup: &impl Fn(&str) -> Option<String>,
    errors: &mut Vec<String>,
) -> Option<i64> {
    match lookup(name) {
        Some(val) => match val.parse::<i64>() {
            Ok(n) => Some(n),
            Err(_) => {
                errors.push(format!("  {name}: invalid integer \"{val}\""));
                None
            }
        },
        None => {
            errors.push(format!("  {name}: missing"));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn env_from(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |name| map.get(name).cloned()
    }

    #[test]
    fn loads_valid_connection_config() {
        let lookup = env_from(&[
            ("VIKUNJA_URL", "http://localhost:59123"),
            ("VIKUNJA_API_TOKEN", "test-token"),
        ]);

        let config = ConnectionConfig::load(lookup).unwrap();

        assert_eq!(config.vikunja_url, "http://localhost:59123");
        assert_eq!(config.vikunja_api_token, "test-token");
    }

    #[test]
    fn reports_missing_connection_vars() {
        let lookup = env_from(&[]);
        let err = ConnectionConfig::load(lookup).unwrap_err();

        assert!(err.contains("VIKUNJA_URL"));
        assert!(err.contains("VIKUNJA_API_TOKEN"));
    }

    #[test]
    fn loads_valid_project_config() {
        let lookup = env_from(&[
            ("VIKUNJA_PROJECT_ID", "1"),
            ("VIKUNJA_VIEW_ID", "10"),
            ("VIKUNJA_TODO_BUCKET_ID", "2"),
            ("VIKUNJA_INPROGRESS_BUCKET_ID", "3"),
            ("VIKUNJA_DONE_BUCKET_ID", "4"),
        ]);

        let config = ProjectConfig::load(lookup).unwrap();

        assert_eq!(config.project_id, 1);
        assert_eq!(config.view_id, 10);
        assert_eq!(config.todo_bucket_id, 2);
        assert_eq!(config.inprogress_bucket_id, 3);
        assert_eq!(config.done_bucket_id, 4);
    }

    #[test]
    fn reports_all_missing_project_vars() {
        let lookup = env_from(&[]);
        let err = ProjectConfig::load(lookup).unwrap_err();

        assert!(err.contains("VIKUNJA_PROJECT_ID"));
        assert!(err.contains("VIKUNJA_VIEW_ID"));
        assert!(err.contains("VIKUNJA_TODO_BUCKET_ID"));
        assert!(err.contains("VIKUNJA_INPROGRESS_BUCKET_ID"));
        assert!(err.contains("VIKUNJA_DONE_BUCKET_ID"));
    }

    #[test]
    fn reports_invalid_project_integer_vars() {
        let lookup = env_from(&[
            ("VIKUNJA_PROJECT_ID", "not-a-number"),
            ("VIKUNJA_VIEW_ID", "10"),
            ("VIKUNJA_TODO_BUCKET_ID", "also-bad"),
            ("VIKUNJA_INPROGRESS_BUCKET_ID", "3"),
            ("VIKUNJA_DONE_BUCKET_ID", "4"),
        ]);

        let err = ProjectConfig::load(lookup).unwrap_err();

        assert!(err.contains("VIKUNJA_PROJECT_ID"));
        assert!(err.contains("VIKUNJA_TODO_BUCKET_ID"));
        assert!(!err.contains("VIKUNJA_INPROGRESS_BUCKET_ID"));
    }
}
