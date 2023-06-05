// Just list a bunch of things from the current crate - what about listing the
// functions that are defined in the toplevel?

use ra_ap_hir::Crate;
use ra_ap_ide::AnalysisHost;
use ra_ap_paths::AbsPathBuf;
use ra_ap_project_model::{ProjectManifest, ProjectWorkspace, RustcSource};
use ra_ap_rust_analyzer::cli::load_cargo;

fn start_rust_analyzer() -> AnalysisHost {
    // TODO: we want to have a `--manifest-path` or something?
    let path = std::env::current_dir().unwrap().join("Cargo.toml");
    let path = AbsPathBuf::assert(path);

    let manifest = ProjectManifest::from_manifest_file(path).unwrap();

    let config = ra_ap_project_model::CargoConfig {
        sysroot: Some(RustcSource::Discover),
        ..Default::default()
    };

    let no_progress = |message| println!("ra reported message {message}");

    let workspace = ProjectWorkspace::load(manifest, &config, &no_progress).unwrap();

    let load_cargo_config = load_cargo::LoadCargoConfig {
        load_out_dirs_from_check: true,
        with_proc_macro: true,
        prefill_caches: false,
    };

    let (host, _, _) =
        load_cargo::load_workspace(workspace, &Default::default(), &load_cargo_config).unwrap();

    host
}

fn main() {
    let host = start_rust_analyzer();
    let db = host.raw_database();

    for krate in Crate::all(db) {
        let display_name = krate.display_name(db).unwrap();
        let display_name = display_name.canonical_name();

        println!("Found crate {display_name}");
    }

    println!("Things went smoothly - stopping rust-analyzer!");
}
