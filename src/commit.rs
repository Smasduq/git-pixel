use std::path::Path;

use git2::{Error, Oid, Repository, Signature, Time};

use crate::layout::Placement;

fn resolve_identity(repo: &Repository) -> Result<(String, String), Error> {
    let config = repo.config()?;
    let name = config
        .get_string("user.name")
        .map_err(|_| Error::from_str("could not resolve user.name from git config"))?;
    let email = config
        .get_string("user.email")
        .map_err(|_| Error::from_str("could not resolve user.email from git config"))?;
    if name.is_empty() || email.is_empty() {
        return Err(Error::from_str(
            "user.name and user.email must both be set in git config",
        ));
    }
    Ok((name, email))
}

pub struct CommitOutcome {
    pub count: usize,
    pub before_head: String,
}

pub fn generate_commits(
    repo_path: &Path,
    placements: &[Placement],
) -> Result<CommitOutcome, Error> {
    let repo = Repository::open(repo_path).map_err(|e| {
        Error::from_str(&format!(
            "cannot open git repository at {}: {e}",
            repo_path.display()
        ))
    })?;

    let (name, email) = resolve_identity(&repo)?;

    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    let before_head = head_commit.id().to_string();
    let tree = head_commit.tree()?;
    let mut parent_oid: Oid = head_commit.id();

    let mut count = 0usize;
    for placement in placements {
        for _ in 0..placement.intensity {
            let timestamp = placement
                .date
                .and_hms_opt(12, 0, 0)
                .expect("noon is a valid time")
                .and_utc()
                .timestamp();
            let time = Time::new(timestamp, 0);

            let author = Signature::new(&name, &email, &time)?;
            let committer = Signature::new(&name, &email, &time)?;

            let parent = repo.find_commit(parent_oid)?;
            let message = format!("gitpixel: {}", placement.date);

            let commit_id = repo.commit(
                Some("HEAD"),
                &author,
                &committer,
                &message,
                &tree,
                &[&parent],
            )?;

            parent_oid = commit_id;
            count += 1;
        }
    }

    Ok(CommitOutcome {
        count,
        before_head,
    })
}

/// Check whether the repo's HEAD currently points at `expected_oid`.
pub fn head_is(repo: &Repository, expected_oid: &str) -> Result<bool, Error> {
    let head_oid = repo.head()?.peel_to_commit()?.id().to_string();
    Ok(head_oid == expected_oid)
}

/// Hard-reset the repo's current branch back to `target_oid`.
pub fn reset_to_oid(repo_path: &Path, target_oid: &str) -> Result<(), Error> {
    let repo = Repository::open(repo_path).map_err(|e| {
        Error::from_str(&format!(
            "cannot open git repository at {}: {e}",
            repo_path.display()
        ))
    })?;
    let oid = Oid::from_str(target_oid)
        .map_err(|_| Error::from_str(&format!("invalid target oid: {target_oid}")))?;
    let commit = repo
        .find_commit(oid)
        .map_err(|_| Error::from_str(&format!("commit {target_oid} not found")))?;
    repo.reset(commit.as_object(), git2::ResetType::Hard, None)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use chrono::NaiveDate;
    use git2::Commit;
    use tempfile::TempDir;

    fn setup_repo() -> (TempDir, Repository) {
        let dir = TempDir::new().unwrap();
        let repo = Repository::init(dir.path()).unwrap();

        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();
        drop(config);

        // Create an initial commit so HEAD has a parent to walk from.
        let sig = Signature::now("Test User", "test@example.com").unwrap();
        {
            let tree_id = {
                let mut index = repo.index().unwrap();
                index.write_tree().unwrap()
            };
            let tree = repo.find_tree(tree_id).unwrap();
            repo.commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
                .unwrap();
        }

        (dir, repo)
    }

    #[test]
    fn creates_correct_number_of_commits_on_dates() {
        let (dir, repo) = setup_repo();
        let placements = vec![
            Placement {
                date: NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(),
                intensity: 3,
            },
            Placement {
                date: NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(),
                intensity: 1,
            },
            Placement {
                date: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
                intensity: 2,
            },
        ];

        let outcome = generate_commits(dir.path(), &placements).unwrap();
        assert_eq!(outcome.count, 6);

        // Walk history (excluding the initial commit) and count per-date.
        let mut counts: HashMap<NaiveDate, usize> = HashMap::new();
        let mut walk = repo.revwalk().unwrap();
        walk.push_head().unwrap();
        walk.set_sorting(git2::Sort::TOPOLOGICAL).unwrap();
        for oid in walk {
            let commit: Commit = repo.find_commit(oid.unwrap()).unwrap();
            if commit.message().map(|m| m.starts_with("gitpixel:")).unwrap_or(false) {
                let date = chrono::DateTime::from_timestamp(commit.time().seconds(), 0)
                    .unwrap()
                    .date_naive();
                *counts.entry(date).or_insert(0) += 1;
            }
        }

        assert_eq!(*counts.get(&NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()).unwrap(), 4);
        assert_eq!(*counts.get(&NaiveDate::from_ymd_opt(2026, 2, 1).unwrap()).unwrap(), 2);
    }

    #[test]
    fn reset_to_oid_reverts_generated_commits() {
        let (dir, repo) = setup_repo();
        let before = repo.head().unwrap().peel_to_commit().unwrap().id().to_string();

        let placements = vec![Placement {
            date: NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(),
            intensity: 2,
        }];
        let outcome = generate_commits(dir.path(), &placements).unwrap();
        assert_eq!(outcome.before_head, before);

        // HEAD should now be at the after state.
        let after = repo.head().unwrap().peel_to_commit().unwrap().id().to_string();
        assert!(head_is(&repo, &after).unwrap());

        // Revert back to the pre-run head.
        reset_to_oid(dir.path(), &before).unwrap();
        let head_now = repo.head().unwrap().peel_to_commit().unwrap().id().to_string();
        assert_eq!(head_now, before);
        assert!(head_is(&repo, &before).unwrap());
    }

    #[test]
    fn errors_when_identity_missing() {
        let dir = TempDir::new().unwrap();
        let repo = Repository::init(dir.path()).unwrap();
        drop(repo);

        // No user.name/email configured.
        let placements = vec![Placement {
            date: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            intensity: 1,
        }];
        assert!(generate_commits(dir.path(), &placements).is_err());
    }

    #[test]
    fn errors_on_open_invalid_repo_path() {
        let dir = TempDir::new().unwrap();
        let placements = vec![Placement {
            date: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            intensity: 1,
        }];
        assert!(generate_commits(dir.path(), &placements).is_err());
    }
}
