#[derive(Debug, Clone)]
pub struct Music {
    title: String,
    like: u32,
    unlike: u32,
    now_playing: bool,
}

impl Music {
    pub fn new(title: String, like: u32, unlike: u32) -> Self {
        Self { title, like, unlike, now_playing: false }
    }
}

pub struct Playlist {
    list: Vec<Music>,
}

impl Playlist {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    // add
    pub fn add(&mut self, music: Music) {
        self.list.push(music);
        //aynısı varsa tekrar eklenmeyecek kontrol eklenecek buraya
    }
    // remove
    pub fn remove(&mut self, index: usize) {
        if index < self.list.len() {
            self.list.remove(index);
        } else {
            panic!("index is not available")
        }
    }
    // play
    pub fn play(&mut self, index: usize) {
        if index < self.list.len() {
            self.list[index].now_playing = true;
        } else {
            panic!("index is not available")
        }
    }
    // durdur
    pub fn stop(&mut self, index: usize) {
        if index < self.list.len() {
            self.list[index].now_playing = false;
        } else {
            panic!("index is not available")
        }
    }
    // like
    pub fn like_music(&mut self, index: usize) {
        if index < self.list.len() {
            self.list[index].like += 1;
        } else {
            panic!("index is not available")
        }
    }
    // unlike
    pub fn unlike_music(&mut self, index: usize) {
        if index < self.list.len() {
            self.list[index].like -= 1;
        } else {
            panic!("index is not available")
        }
    }
    // print list
    pub fn print_playlist(&self) {
        for (index, music) in self.list.iter().enumerate() {
            println!(
                "{}.Playing:[{}], Title:{}, Like:{}, Unlike:{}",
                index + 1,
                if music.now_playing {
                    "x"
                } else {
                    ""
                },
                music.title,
                music.like,
                music.unlike,
              
            );
        }
        println!("---------------------------");
    }
    // sonraki şarkı
    pub fn next_music(&mut self) {
        let mut new_list = self.list.clone();
        let mut ind = 0;
        // let mut playing_music = self.list.iter().find(|music| music.now_playing == true);
        for (index, item) in new_list.iter_mut().enumerate() {
            if item.now_playing == true {
                item.now_playing = false;
                ind = index + 1;
            } else {
            }
        }
        new_list[ind].now_playing = true;
        self.list = new_list;

    }
    // önceki şarkı
    pub fn prev_music(&mut self) {
        
        let mut new_list = self.list.clone();
        let mut ind = 0;
        // let mut playing_music = self.list.iter().find(|music| music.now_playing == true);
        for (index, item) in new_list.iter_mut().enumerate() {
            if item.now_playing == true {
                item.now_playing = false;
                ind = index - 1;
            } else {
            }
        }
        new_list[ind].now_playing = true;
        self.list = new_list;
    }
}
