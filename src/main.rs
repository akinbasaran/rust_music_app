mod library;

use library::music::{ Music, Playlist };
fn main() {
    let mut favori_muziklerim = Playlist::new();
    let music1 = Music::new("lovely".to_string(), 0, 0);
    let music2 = Music::new("yarim yarim".to_string(), 0, 0);
    let music3 = Music::new("canısı".to_string(), 0, 0);
    let music4 = Music::new("konyalım".to_string(), 0, 0);

    favori_muziklerim.add(music1);
    favori_muziklerim.add(music2);
    favori_muziklerim.add(music3);
    favori_muziklerim.add(music4); 
    favori_muziklerim.print_playlist();
    favori_muziklerim.like_music(3);
    favori_muziklerim.like_music(2);
    favori_muziklerim.like_music(2);
    favori_muziklerim.like_music(1);
    favori_muziklerim.like_music(1);
    favori_muziklerim.print_playlist();
    favori_muziklerim.unlike_music(3);
    favori_muziklerim.unlike_music(2);
    favori_muziklerim.unlike_music(1);
    favori_muziklerim.print_playlist();
    favori_muziklerim.play(1);
    favori_muziklerim.print_playlist();
    favori_muziklerim.next_music();
    favori_muziklerim.print_playlist();
    favori_muziklerim.prev_music();
    favori_muziklerim.prev_music();
    favori_muziklerim.print_playlist();
    favori_muziklerim.remove(3);
    favori_muziklerim.print_playlist();


    
}
