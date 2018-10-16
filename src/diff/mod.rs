use crate::mend_github::MendGithub;
use std::io::Read;
use unidiff::PatchSet;

#[derive(Debug)]
pub struct Diff {
    pub url: String,
    pub patchset: PatchSet,
}

impl Diff {
    pub fn from_pr(
        client: &mut MendGithub,
        user: &str,
        repo: &str,
        pr_id: u64,
    ) -> Result<Diff, String> {
        let diff_url = client.get_diff_url(user, repo, pr_id);
        let mut resp = reqwest::get(&diff_url).unwrap();
        assert!(resp.status().is_success());

        let mut content = String::new();
        resp.read_to_string(&mut content).unwrap();
        let patchset = Diff::patchset_from_str(&content);
        Ok(Diff {
            url: diff_url,
            patchset,
        })
    }

    pub fn patchset_from_str(content: &str) -> PatchSet {
        let mut patchset = PatchSet::new();
        patchset.parse(content).expect("Error parsing diff");
        patchset
    }
}
