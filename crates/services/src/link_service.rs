use common::Link;
use database::link_repo::LinkRepo;

pub struct LinkService {
    repo: LinkRepo,
}

#[derive(Debug)]
pub enum LinkServiceError {
    RepoError,
}

impl LinkService {
    pub fn new(repo: LinkRepo) -> Self {
        LinkService { repo }
    }

    pub fn create_link(&self, target_url: &str) -> Result<Link, LinkServiceError> {
        self.repo
            .create_link(target_url)
            .map_err(|_| LinkServiceError::RepoError)
    }

    pub fn get_link(&self, code: &str) -> Result<Option<Link>, LinkServiceError> {
        self.repo
            .get_link(code)
            .map_err(|_| LinkServiceError::RepoError)
    }
}
