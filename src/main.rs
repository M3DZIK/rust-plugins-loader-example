use plugins_loader::loader;

fn main() -> anyhow::Result<()> {
    let plugins = loader()?;

    for plugin in &plugins.plugins {
        plugin.execute()
    }

    Ok(())
}
