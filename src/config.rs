use std::env;

#[derive(Debug)]
pub struct Config {
    pub vikunja_url: String,
    pub vikunja_api_token: String,
    pub vikunja_project_id: i64,
    pub vikunja_todo_bucket_id: i64,
    pub vikunja_inprogress_bucket_id: i64,
    pub vikunja_done_bucket_id: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Self::load(|name| env::var(name).ok())
    }

    pub fn load(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let mut errors = Vec::new();

        let vikunja_url = read_string("VIKUNJA_URL", &lookup, &mut errors);
        let vikunja_api_token = read_string("VIKUNJA_API_TOKEN", &lookup, &mut errors);
        let vikunja_project_id = read_i64("VIKUNJA_PROJECT_ID", &lookup, &mut errors);
        let vikunja_todo_bucket_id = read_i64("VIKUNJA_TODO_BUCKET_ID", &lookup, &mut errors);
        let vikunja_inprogress_bucket_id =
            read_i64("VIKUNJA_INPROGRESS_BUCKET_ID", &lookup, &mut errors);
        let vikunja_done_bucket_id = read_i64("VIKUNJA_DONE_BUCKET_ID", &lookup, &mut errors);

        if !errors.is_empty() {
            return Err(format!("configuration errors:\n{}", errors.join("\n")));
        }

        Ok(Config {
            vikunja_url: vikunja_url.unwrap(),
            vikunja_api_token: vikunja_api_token.unwrap(),
            vikunja_project_id: vikunja_project_id.unwrap(),
            vikunja_todo_bucket_id: vikunja_todo_bucket_id.unwrap(),
            vikunja_inprogress_bucket_id: vikunja_inprogress_bucket_id.unwrap(),
            vikunja_done_bucket_id: vikunja_done_bucket_id.unwrap(),
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

    fn all_valid() -> impl Fn(&str) -> Option<String> {
        env_from(&[
            ("VIKUNJA_URL", "https://project.junco.dev"),
            ("VIKUNJA_API_TOKEN", "test-token"),
            ("VIKUNJA_PROJECT_ID", "1"),
            ("VIKUNJA_TODO_BUCKET_ID", "2"),
            ("VIKUNJA_INPROGRESS_BUCKET_ID", "3"),
            ("VIKUNJA_DONE_BUCKET_ID", "4"),
        ])
    }

    #[test]
    fn loads_valid_config() {
        let config = Config::load(all_valid()).unwrap();

        assert_eq!(config.vikunja_url, "https://project.junco.dev");
        assert_eq!(config.vikunja_api_token, "test-token");
        assert_eq!(config.vikunja_project_id, 1);
        assert_eq!(config.vikunja_todo_bucket_id, 2);
        assert_eq!(config.vikunja_inprogress_bucket_id, 3);
        assert_eq!(config.vikunja_done_bucket_id, 4);
    }

    #[test]
    fn reports_all_missing_vars() {
        let lookup = env_from(&[]);
        let err = Config::load(lookup).unwrap_err();

        assert!(err.contains("VIKUNJA_URL"), "should mention VIKUNJA_URL");
        assert!(
            err.contains("VIKUNJA_API_TOKEN"),
            "should mention VIKUNJA_API_TOKEN"
        );
        assert!(
            err.contains("VIKUNJA_PROJECT_ID"),
            "should mention VIKUNJA_PROJECT_ID"
        );
        assert!(
            err.contains("VIKUNJA_TODO_BUCKET_ID"),
            "should mention VIKUNJA_TODO_BUCKET_ID"
        );
        assert!(
            err.contains("VIKUNJA_INPROGRESS_BUCKET_ID"),
            "should mention VIKUNJA_INPROGRESS_BUCKET_ID"
        );
        assert!(
            err.contains("VIKUNJA_DONE_BUCKET_ID"),
            "should mention VIKUNJA_DONE_BUCKET_ID"
        );
    }

    #[test]
    fn reports_invalid_integer_vars() {
        let lookup = env_from(&[
            ("VIKUNJA_URL", "https://project.junco.dev"),
            ("VIKUNJA_API_TOKEN", "test-token"),
            ("VIKUNJA_PROJECT_ID", "not-a-number"),
            ("VIKUNJA_TODO_BUCKET_ID", "also-bad"),
            ("VIKUNJA_INPROGRESS_BUCKET_ID", "3"),
            ("VIKUNJA_DONE_BUCKET_ID", "4"),
        ]);

        let err = Config::load(lookup).unwrap_err();

        assert!(
            err.contains("VIKUNJA_PROJECT_ID"),
            "should mention VIKUNJA_PROJECT_ID"
        );
        assert!(
            err.contains("VIKUNJA_TODO_BUCKET_ID"),
            "should mention VIKUNJA_TODO_BUCKET_ID"
        );
        assert!(
            !err.contains("VIKUNJA_INPROGRESS_BUCKET_ID"),
            "should not mention valid vars"
        );
    }
}
