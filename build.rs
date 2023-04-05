fn main() {
    if std::env::var_os("DOCS_RS").is_some() {
        println!("cargo:rustc-env=SQLX_OFFLINE=true");
    }

    println!("cargo:rerun-if-changed=migrations");
}
