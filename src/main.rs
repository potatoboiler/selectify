use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-library-read")).unwrap();

    let mut spotify = AuthCodeSpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).await.unwrap();

    let stream = spotify.current_user_saved_tracks(None);
    println!("\nItems (concurrent):");
    stream
        .try_for_each_concurrent(10, |item| async move {
            println!("* {}", item.track.name);
            Ok(())
        })
        .await
        .unwrap();

    // https://github.com/ramsayleung/rspotify/blob/master/examples/pagination_async.rs
}
