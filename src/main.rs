use std::{env, process::exit};

use librespot_core::{
    authentication::Credentials,
    config::SessionConfig,
    session::Session,
    spotify_id::{SpotifyId, SpotifyAudioType},
};
use librespot_playback::{
    audio_backend,
    config::{AudioFormat, PlayerConfig},
    mixer::NoOpVolume,
    player::Player,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let username = args[1].clone();
    let password = args[2].clone();

    let mut track = SpotifyId::from_base62(&args[3]).unwrap();
    track.audio_type = SpotifyAudioType::Track;

    //for (backend_name, _) in audio_backend::BACKENDS {
    //    println!("{backend_name}");
    //}
    //exit(1);
    let (backend_name, backend) = &audio_backend::BACKENDS[0];

    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();

    println!("Connecting....");
    if let Ok((session, _)) = Session::connect(
        session_config,
        Credentials::with_password(username, password),
        None,
        false
    ).await {
        let (mut player, _) = Player::new(
            player_config,
            session,
            Box::new(NoOpVolume),
            move || {
                println!("{backend_name}");
                backend(None, audio_format)
            }
        );

        player.load(track, true, 0);
        //player.play();

        println!("Playing...");

        player.await_end_of_track().await;

        println!("Done");

    }
}


