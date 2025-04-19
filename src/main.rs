use clap::{arg, command};

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .arg(arg!(<search> "search element\nenter [.] for lib start page").requires_if(".", "lib"))
        .arg(arg!([lib] "library to search through").default_value("std"))
        .get_matches();
    let search = matches.get_one::<String>("search").expect("required");
    let lib = matches.get_one::<String>("lib").expect("default");
    open_docs(lib, search)?;
    Ok(())
}

fn open_docs(lib: &str, search: &str) -> anyhow::Result<()> {
    let url = match (lib, search) {
        ("std", search) => format!("https://doc.rust-lang.org/std/?search={search}"),
        (lib, ".") => format!("https://docs.rs/{lib}/latest/{lib}/"),
        (lib, search) => format!("https://docs.rs/{lib}/latest/{lib}/?search={search}"),
    };
    webbrowser::open(&url)?;
    Ok(())
}
