use utils::prelude::*;

#[test]
pub fn update_example_env_toml() -> Result<()> {
  let paths = ProjectPaths::get()?;
  let example = paths.website()?.file("env.toml")?;
  let real = paths.website()?.file(".env.toml")?;

  let real: toml::Table =
    toml::from_str(&fs::read_to_string(real).wrap_err("Couldn't read real toml")?)
      .wrap_err("Couldn't parse real toml")?;
  let example = fs::read_to_string(example).wrap_err("Couldn't read example toml")?;
  let example: toml::Table = toml::from_str(&example)?;

  for key in real.keys() {
    if !example.contains_key(key.as_str()) {
      bail!(
        "Must add the key {} to the example toml as it appears in the real toml",
        key
      )
    }
  }

  info!("Checked both real and example env.toml files, all looks good :)");

  Ok(())
}
