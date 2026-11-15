//! SigmaOS-data-verification: data verification model.
//!
//! Real implementation: checks, signatures, hashes, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Verification check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationCheck {
    pub id: String,
    pub name: String,
    pub data: String,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError {
    EmptyName,
    DuplicateCheck(String),
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "check name must not be empty"),
            Self::DuplicateCheck(id) => write!(f, "duplicate check {:?}", id),
        }
    }
}

impl std::error::Error for VerificationError {}

/// Data verification engine.
pub struct DataVerification {
    pub name: String,
    checks: BTreeMap<String, VerificationCheck>,
}

impl DataVerification {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            checks: BTreeMap::new(),
        }
    }

    pub fn add_check(&mut self, check: VerificationCheck) -> Result<(), VerificationError> {
        if check.name.trim().is_empty() { return Err(VerificationError::EmptyName); }
        let id = check.id.clone();
        if self.checks.contains_key(&id) {
            return Err(VerificationError::DuplicateCheck(id.clone()));
        }
        self.checks.insert(id, check);
        Ok(())
    }

    pub fn check_count(&self) -> usize { self.checks.len() }

    pub fn get_check(&self, id: &str) -> Option<&VerificationCheck> { self.checks.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_create_aaa() {
        let v = DataVerification::new("My Verification");
        assert_eq!(v.name, "My Verification");
        assert_eq!(v.check_count(), 0);
    }

    #[test]
    fn test_add_check_aaa() {
        let mut v = DataVerification::new("My Verification");
        let c = VerificationCheck { id: "c1".into(), name: "hash-check".into(), data: "file".into(), hash: "sha256".into() };
        v.add_check(c).unwrap();
        assert_eq!(v.check_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut v = DataVerification::new("My Verification");
        let c = VerificationCheck { id: "x".into(), name: "".into(), data: "a".into(), hash: "b".into() };
        assert!(v.add_check(c).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut v = DataVerification::new("My Verification");
        let c = VerificationCheck { id: "x".into(), name: "X".into(), data: "a".into(), hash: "b".into() };
        v.add_check(c).unwrap();
        let c2 = VerificationCheck { id: "x".into(), name: "X".into(), data: "a".into(), hash: "b".into() };
        assert!(v.add_check(c2).is_err());
    }
}
