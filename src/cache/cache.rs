pub(crate) struct Cache {
    dir: std::string::String,
    ext: std::string::String,
}

impl Cache {
    pub(crate) fn new(
        name: &std::primitive::str,
        ext: &std::primitive::str,
    ) -> std::result::Result<Self, std::io::Error> {
        let dir: std::string::String = format!("cache/{}", name);
        std::fs::create_dir_all(dir.clone())?;
        Ok(Self {
            dir,
            ext: ext.to_string(),
        })
    }

    fn filename(&self, key: &std::primitive::str) -> std::string::String {
        format!(
            "{:x}.{}",
            <sha2::Sha256 as sha2::Digest>::digest(key),
            self.ext
        )
    }

    fn path(&self, key: &std::primitive::str) -> std::string::String {
        format!("{}/{}", self.dir, self.filename(key))
    }

    pub(crate) fn write(
        &self,
        key: &std::primitive::str,
        contents: &std::primitive::str,
    ) -> std::result::Result<(), std::io::Error> {
        std::fs::write(self.path(key), contents)
    }

    pub(crate) fn exists(&self, key: &std::primitive::str) -> std::primitive::bool {
        std::path::Path::new(&self.path(key)).exists()
    }

    pub(crate) fn read(
        &self,
        key: &std::primitive::str,
    ) -> std::result::Result<std::string::String, std::io::Error> {
        std::fs::read_to_string(self.path(key))
    }
}
