pub struct Config {
    pub standard_import_paths: &'static [&'static str],
}

pub static CONFIG: Config = Config {
    standard_import_paths: &["/usr/local/include", "/usr/include"],
};
