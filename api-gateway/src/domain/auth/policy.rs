use http::Method;
use serde::Deserialize;

/// One row from `[[route]]` in config.toml. Compiled into `RouteRule`.
#[derive(Debug, Clone, Deserialize)]
pub struct RouteConfig {
    pub pattern: String,
    pub methods: Vec<String>,
    pub upstream: String,
    #[serde(default)]
    pub public: bool,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
}

/// Compiled route rule used at request time.
#[derive(Debug, Clone)]
pub struct RouteRule {
    pub segments: Vec<PatternSeg>,
    pub methods: Vec<Method>,
    pub upstream: String,
    pub policy: AccessPolicy,
}

#[derive(Debug, Clone)]
pub enum PatternSeg {
    Literal(String),
    Param,    // :id
    CatchAll, // *rest (must be last)
}

#[derive(Debug, Clone)]
pub struct AccessPolicy {
    pub public: bool,
    pub required_roles: Vec<String>,  // any-of
    pub required_scopes: Vec<String>, // all-of
}

impl RouteRule {
    pub fn compile(cfg: RouteConfig) -> anyhow::Result<Self> {
        let segments = parse_pattern(&cfg.pattern)?;
        let mut methods = Vec::with_capacity(cfg.methods.len());
        for m in &cfg.methods {
            methods.push(
                m.parse::<Method>()
                    .map_err(|e| anyhow::anyhow!("invalid method '{m}': {e}"))?,
            );
        }
        Ok(Self {
            segments,
            methods,
            upstream: cfg.upstream,
            policy: AccessPolicy {
                public: cfg.public,
                required_roles: cfg.roles,
                required_scopes: cfg.scopes,
            },
        })
    }

    pub fn matches(&self, method: &Method, path: &str) -> bool {
        if !self.methods.contains(method) {
            return false;
        }
        let path_segs: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let mut pi = 0usize;
        for (i, seg) in self.segments.iter().enumerate() {
            match seg {
                PatternSeg::CatchAll => return true,
                PatternSeg::Param => {
                    if pi >= path_segs.len() || path_segs[pi].is_empty() {
                        return false;
                    }
                    pi += 1;
                }
                PatternSeg::Literal(lit) => {
                    if pi >= path_segs.len() || path_segs[pi] != lit {
                        return false;
                    }
                    pi += 1;
                }
            }
            // last segment & path has more → not a match unless catch-all
            if i + 1 == self.segments.len() && pi < path_segs.len() {
                return false;
            }
        }
        pi == path_segs.len()
    }
}

fn parse_pattern(pat: &str) -> anyhow::Result<Vec<PatternSeg>> {
    let trimmed = pat.trim_start_matches('/');
    let mut out = Vec::new();
    let parts: Vec<&str> = trimmed.split('/').collect();
    for (i, p) in parts.iter().enumerate() {
        if let Some(rest) = p.strip_prefix('*') {
            if i + 1 != parts.len() {
                anyhow::bail!("catch-all '*{rest}' must be the last segment in '{pat}'");
            }
            out.push(PatternSeg::CatchAll);
        } else if p.starts_with(':') {
            out.push(PatternSeg::Param);
        } else {
            out.push(PatternSeg::Literal((*p).to_owned()));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(pattern: &str, methods: &[&str]) -> RouteRule {
        RouteRule::compile(RouteConfig {
            pattern: pattern.to_owned(),
            methods: methods.iter().map(|s| (*s).to_owned()).collect(),
            upstream: "u".into(),
            public: false,
            roles: vec![],
            scopes: vec![],
        })
        .unwrap()
    }

    #[test]
    fn literal_match() {
        let r = rule("/api/channels", &["GET"]);
        assert!(r.matches(&Method::GET, "/api/channels"));
        assert!(!r.matches(&Method::POST, "/api/channels"));
        assert!(!r.matches(&Method::GET, "/api/channels/abc"));
    }

    #[test]
    fn param_match() {
        let r = rule("/api/channels/:id", &["GET"]);
        assert!(r.matches(&Method::GET, "/api/channels/123"));
        assert!(!r.matches(&Method::GET, "/api/channels/"));
        assert!(!r.matches(&Method::GET, "/api/channels"));
    }

    #[test]
    fn nested_param() {
        let r = rule("/api/reservations/:id/cancel", &["POST"]);
        assert!(r.matches(&Method::POST, "/api/reservations/abc/cancel"));
        assert!(!r.matches(&Method::POST, "/api/reservations/abc/foo"));
    }
}
