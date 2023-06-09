// Just list a bunch of things from the current crate - what about listing the
// functions that are defined in the toplevel?

mod visitor;

use std::{thread, time::Duration};

use ra_ap_base_db::SourceDatabaseExt;
use ra_ap_hir::{Crate, Semantics};
use ra_ap_ide::{AnalysisHost, RootDatabase};
use ra_ap_ide_db::symbol_index::SymbolsDatabase;
use ra_ap_paths::AbsPathBuf;
use ra_ap_project_model::{PackageData, ProjectManifest, ProjectWorkspace, RustcSource};
use ra_ap_rust_analyzer::cli::load_cargo;

use crate::visitor::SymbolVisitor;

fn start_rust_analyzer() -> (AnalysisHost, Vec<PackageData>) {
    // TODO: we want to have a `--manifest-path` or something?
    let path = std::env::current_dir().unwrap().join("Cargo.toml");
    let path = AbsPathBuf::assert(path);

    let manifest = ProjectManifest::from_manifest_file(path).unwrap();

    let config = ra_ap_project_model::CargoConfig {
        sysroot: Some(RustcSource::Discover),
        ..Default::default()
    };

    let no_progress = |_| ();

    let workspace = ProjectWorkspace::load(manifest, &config, &no_progress).unwrap();

    let roots = match workspace {
        ProjectWorkspace::Cargo { ref cargo, .. } => cargo
            .packages()
            .map(|package| cargo[package].clone())
            .filter(|package| package.is_member)
            .collect(),
        _ => todo!("Unimplemented crate type"),
    };

    let load_cargo_config = load_cargo::LoadCargoConfig {
        load_out_dirs_from_check: true,
        with_proc_macro: true,
        prefill_caches: false,
    };

    let (host, _, _) =
        load_cargo::load_workspace(workspace, &Default::default(), &load_cargo_config).unwrap();

    (host, roots)
}

fn main() {
    let (host, _roots) = start_rust_analyzer();
    let db = host.raw_database();

    let semantics = Semantics::new(db);

    let stuff = collect_function_calls(db, semantics);

    dbg!(stuff);

    println!("Things went smoothly - stopping rust-analyzer!");
}

fn collect_function_calls(
    db: &RootDatabase,
    semantics: Semantics<RootDatabase>,
) -> Vec<ra_ap_hir::PathResolution> {
    let stuff = db
        .local_roots()
        .iter()
        .flat_map(|root| {
            let root = db.source_root(*root);
            let tmp = SymbolVisitor::visit_crate(&root, &semantics);

            thread::sleep(Duration::from_secs(10));
            tmp
        })
        .collect::<Vec<_>>();
    stuff
}
