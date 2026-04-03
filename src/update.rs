use self_update::cargo_crate_version;

pub fn update() {
    let target = self_update::get_target();
    let status = self_update::backends::github::Update::configure()
        .repo_owner("bebop-dex")
        .repo_name("bebop-cli")
        .bin_name("bebop")
        .bin_path_in_archive(&format!("bebop-{}/bebop", target))
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()
        .and_then(|u| u.update());

    match status {
        Ok(s) => println!("updated to v{}!", s.version()),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
