use cargo::core::manifest::EitherManifest;
use cargo::core::SourceId;
use cargo::util::config::Config;
use cargo::util::errors::CargoResult;
use cargo::util::toml::read_manifest;

fn main() -> CargoResult<()> {
    let abs_path = &to_absolute::to_absolute_from_current_dir(".")?;
    let source_id = SourceId::for_directory(abs_path)?;
    let abs_cargo_toml = &to_absolute::to_absolute_from_current_dir("Cargo.toml")?;
    let config = Config::default()?;

    let (manifest, _) = read_manifest(abs_cargo_toml, source_id, &config)?;

    match manifest {
        EitherManifest::Virtual(m) => {
            println!("note: this is virtual manifest.");
            println!("{:#?}", m);
        }
        EitherManifest::Real(m) => {
            println!("{:#?}", m);
        }
    };

    Ok(())
}
