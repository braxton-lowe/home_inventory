use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort: Option<String>,
    pub sort_by: Option<String>,
    pub search: Option<String>,
    pub active: Option<String>,
}

impl ListParams {
    pub fn limit_or(&self, default: i64) -> i64 {
        self.limit.unwrap_or(default).min(100).max(1)
    }

    pub fn offset_or(&self, default: i64) -> i64 {
        self.offset.unwrap_or(default).max(0)
    }

    pub fn search_filter(&self) -> Option<String> {
        self.search.as_deref()
            .filter(|s| !s.is_empty())
            .map(|s| format!("%{}%", s))
    }

    /// Returns the sort column, validated against an allowlist.
    /// Falls back to the default if not specified or not in the allowlist.
    pub fn sort_column<'a>(&self, allowed: &[&'a str], default: &'a str) -> &'a str {
        if let Some(ref col) = self.sort_by {
            allowed.iter().find(|&&a| a.eq_ignore_ascii_case(col)).copied().unwrap_or(default)
        } else {
            default
        }
    }

    /// Returns the active filter: Some(true), Some(false), or None (all).
    /// Defaults to Some(true) when not specified.
    pub fn active_filter(&self) -> Option<bool> {
        match self.active.as_deref() {
            Some(s) if s.eq_ignore_ascii_case("false") => Some(false),
            Some(s) if s.eq_ignore_ascii_case("all") => None,
            _ => Some(true), // default: show only active
        }
    }

    /// Returns "ASC" or "DESC" based on the sort param.
    /// Falls back to the provided default if not specified.
    pub fn sort_direction_or<'a>(&self, default: &'a str) -> &'a str {
        match self.sort.as_deref() {
            Some(s) if s.eq_ignore_ascii_case("asc") => "ASC",
            Some(s) if s.eq_ignore_ascii_case("desc") => "DESC",
            _ => default,
        }
    }
}
