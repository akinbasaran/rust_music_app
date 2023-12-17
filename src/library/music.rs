pub struct Music<T> {
    title: T,
    like: bool,
    unlike: bool,
}

impl<T> Music<T> {
    pub fn new(title: T, like: bool, unlike: bool) -> Self {
        Self { title, like, unlike }
    }
}

pub struct Playlist<T> {
    list: Vec<Music<T>>,
    now_playing: Option<Music<T>>,
}

impl<T> Playlist<T> {
    pub fn new() -> Self {
        Self { list: Vec::new(), now_playing: None }
    }

    // add aynı eklenemeyecek filter kullanabılırsın
    pub fn add() {
        let a = [1, 2, 3,4];

        assert_eq!(
            a.iter().find(|&&x| x == 2),
            Some(&2)
        );

        assert_eq!(
            a.iter().find(|&&x| x == 5),
            None
        );
    }
    // remove

    // play olacak now playıngten

    // sonraki şarkı

    // önceki şarkı

    // durdur

    // like boole

    // unlike boole

    // print list
}
