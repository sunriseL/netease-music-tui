use super::super::app::App;
use termion::event::Key;
use super::common_events;
use std::collections::HashSet;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            let next_index = common_events::on_down_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
            );
            app.track_table.selected_index = next_index;
        }
        k if common_events::up_event(k) => {
            let next_index = common_events::on_up_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
            );
            app.track_table.selected_index = next_index;
        }
        Key::Char('\n') => {
            #[allow(non_snake_case)]
            let TrackTable = &app.track_table;
            let track_playing = TrackTable.tracks.get(TrackTable.selected_index.to_owned()).unwrap().to_owned();
            // println!("{:#?}", track);
            // let url = &app.get_song_url();
            match &mut app.my_playlist {
                Some(list) => {
                    // if exist, append tracktable to my playlist
                    list.tracks = TrackTable.tracks.to_owned();
                }
                None => {
                    // if none, add this tracktable to my playlist
                    app.my_playlist = Some(TrackTable.to_owned());
                }
            }
            app.start_playback(track_playing.id.unwrap().to_string());
            app.current_playing = Some(track_playing);
        }
        _ => {}
    }
}
