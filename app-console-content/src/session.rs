use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum Tenant {
    Pointsav,
    Woodfine,
}

impl Tenant {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pointsav" => Some(Tenant::Pointsav),
            "woodfine" => Some(Tenant::Woodfine),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Tenant::Pointsav => "pointsav",
            Tenant::Woodfine => "woodfine",
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub tenant: Tenant,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Idle,
    Proofread,
    Running,
    Results,
    Draft,
}

pub struct ProofSession {
    pub user: User,
    pub mode: Mode,
    pub buffer: String,
    pub started: Instant,
}

impl ProofSession {
    pub fn new(user: User) -> Self {
        Self {
            user,
            mode: Mode::Idle,
            buffer: String::new(),
            started: Instant::now(),
        }
    }
}
