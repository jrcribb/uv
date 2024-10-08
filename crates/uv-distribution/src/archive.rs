use distribution_types::Hashed;
use pypi_types::HashDigest;
use uv_cache::{ArchiveId, Cache};

/// An archive (unzipped wheel) that exists in the local cache.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Archive {
    /// The unique ID of the entry in the wheel's archive bucket.
    pub id: ArchiveId,
    /// The computed hashes of the archive.
    pub hashes: Vec<HashDigest>,
}

impl Archive {
    /// Create a new [`Archive`] with the given ID and hashes.
    pub(crate) fn new(id: ArchiveId, hashes: Vec<HashDigest>) -> Self {
        Self { id, hashes }
    }

    /// Returns `true` if the archive exists in the cache.
    pub(crate) fn exists(&self, cache: &Cache) -> bool {
        cache.archive(&self.id).exists()
    }
}

impl Hashed for Archive {
    fn hashes(&self) -> &[HashDigest] {
        &self.hashes
    }
}
