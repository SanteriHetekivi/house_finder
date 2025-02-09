/// File based cache.
pub(crate) struct Cache {
    pub(self) dir: std::string::String,
    pub(self) ext: std::string::String,
}

impl Cache {
    /// Create a new cache object
    ///
    /// # Arguments
    /// * `name` - The name of the cache directory.
    /// * `ext` - The extension of the cache files.
    pub(crate) fn new(
        name: &std::primitive::str,
        ext: &std::primitive::str,
    ) -> std::result::Result<Self, std::io::Error> {
        let mut exe_dir: std::path::PathBuf = std::env::current_exe()?;
        exe_dir.pop(); // Remove the executable name to get the directory
        let dir: std::string::String = exe_dir
            .join(format!("cache/{}", name))
            .to_str()
            .unwrap()
            .to_string();
        std::fs::create_dir_all(dir.clone())?;
        Ok(Self {
            dir,
            ext: ext.to_string(),
        })
    }

    /// Generate filename for given key.
    ///
    /// # Arguments
    /// * `key` - The key to generate the filename for.
    pub(self) fn filename(&self, key: &std::primitive::str) -> std::string::String {
        format!(
            "{:x}.{}",
            <sha2::Sha256 as sha2::Digest>::digest(key),
            self.ext
        )
    }

    /// Generate path for given key.
    ///
    /// # Arguments
    /// * `key` - The key to generate the path for.
    pub(self) fn path(&self, key: &std::primitive::str) -> std::string::String {
        format!("{}/{}", self.dir, self.filename(key))
    }

    /// Write contents to cache file.
    ///
    /// # Arguments
    /// * `key` - The key to use to generate filename.
    /// * `contents` - The contents to write to the cache file.
    pub(crate) fn write(
        &self,
        key: &std::primitive::str,
        contents: &std::primitive::str,
    ) -> std::result::Result<(), std::io::Error> {
        std::fs::write(self.path(key), contents)
    }

    /// Check if cache file exists.
    ///
    /// # Arguments
    /// * `key` - The key to use to generate filename.
    pub(crate) fn exists(&self, key: &std::primitive::str) -> std::primitive::bool {
        std::path::Path::new(&self.path(key)).exists()
    }

    /// Read contents from cache file.
    ///
    /// # Arguments
    /// * `key` - The key to use to generate filename.
    pub(crate) fn read(
        &self,
        key: &std::primitive::str,
    ) -> std::result::Result<std::string::String, std::io::Error> {
        std::fs::read_to_string(self.path(key))
    }
}
