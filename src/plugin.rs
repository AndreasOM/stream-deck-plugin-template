use stream_deck_plugin::ClientResponse;
use color_eyre::Result;
use stream_deck_plugin::Client;

#[derive(Debug)]
pub struct Plugin {
	client: Client,
}

impl Plugin {

	pub async fn new_from_args() -> Result<Self> {
		let client = Client::new_from_args().await?;
		Ok( Self {
			client,

		} )
	}

	async fn handle_update(&mut self) -> Result<()> {
		//tracing::info!("Handling update");
		// handle polling of external status here
		Ok(())
	}
	async fn handle_response(&mut self, response: ClientResponse) -> Result<()> {
		//tracing::info!("Handling {response:?}");
		match response {
			ClientResponse::WillAppear{ .. } => {

			},
			ClientResponse::KeyUp { context, payload, .. } => {
				let settings = payload.settings;
				tracing::info!("KeyUp [{context}] -> Settings {settings:?}");
				let state = 1-payload.state;
				self.client.send_set_state( context, state ).await?;
			},
			o => {
				// Note: It might be ok to just ignore some responses, depending on your use case
				tracing::warn!("Unhandled response: {o:?}");
			}
		}
		Ok(())
	}
	pub async fn run( &mut self ) -> Result<()> {
	    loop {
	        let timeout = tokio::time::sleep(tokio::time::Duration::from_millis(1000));
	        tokio::select! {
	            _ = timeout => {
	                let _todo = self.handle_update( ).await;
	            }
	            Some( response ) = self.client.recv() => {
	                let _todo = self.handle_response( response ).await;
	            }
	        }
	    }

//		Ok(())
	}
}
