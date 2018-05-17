const MAX_COMMIT_LEN:usize = 500;

pub fn assert_commit_len(commit: String) -> bool {
	if commit.len() > MAX_COMMIT_LEN {
		panic!("Commit too long!")
	}
	return true;
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_assert_commit_len() {
    	let commit = "X".repeat(500);
        assert!(assert_commit_len(commit));
    }

    #[test]
    #[should_panic]
    fn test_assert_commit_len_panic() {
    	let commit = "X".repeat(501);
    	assert_commit_len(commit);
    }
}
