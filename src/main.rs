use rspotify::{
    prelude::*,
    scopes,
    model::{PlayableItem, AdditionalType, Country, Market, CurrentlyPlayingContext},
    AuthCodeSpotify,
    Credentials,
    OAuth,
};

#[tokio::main]
async fn main() {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();
    let spotify = AuthCodeSpotify::new(creds, oauth);
    let url = spotify.get_authorize_url(false).unwrap();

    spotify.prompt_for_token(&url).await.unwrap();

    let market = Market::Country(Country::UnitedStates);
    let additional_types = [AdditionalType::Track];
    let current_playing = spotify
        .current_playing(Some(market), Some(&additional_types))
        .await
        .unwrap();

    if let Some(context) = current_playing {
        if let Some(PlayableItem::Track(full_track)) = context.item {
            let artist = &full_track.artists[0].name;
            let track = &full_track.name;

            println!("{artist}: {track}");
        }

    }

}
