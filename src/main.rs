use color_eyre::Result;

use stream_deck_plugin_template::Logger;
use stream_deck_plugin_template::Plugin;

#[tokio::main]
async fn main() -> Result<()> {
    let _logger = Logger::new("/Users/anti/streamdeck-plugin-template.log"); // do not drop!

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
