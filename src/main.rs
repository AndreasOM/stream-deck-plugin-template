use color_eyre::Result;
use directories::BaseDirs;

use stream_deck_plugin_template::Logger;
use stream_deck_plugin_template::Plugin;

#[tokio::main]
async fn main() -> Result<()> {
    // this is not strictly needed, but `tail -f` on this makes development much easier
    let base_dirs = BaseDirs::new().expect("BaseDirs::new should work");
    let mut log_file = base_dirs.home_dir().to_path_buf();
    log_file.push("streamdeck-plugin-template.log");
    let _logger = Logger::new( &log_file ); // do not drop!

    let mut plugin = Plugin::new_from_args().await.map_err(|e|{
        tracing::error!("Error initializing plugin: {e:?}");
        e
    })?;

    match plugin.run().await {
        Ok(_) => {},
        Err( e ) => {
            tracing::error!("Error running plugin: {e:?}")
        }
    }

    Ok(())
}
