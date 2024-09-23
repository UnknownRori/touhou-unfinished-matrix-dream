use std::collections::HashMap;

use bgm::BGM;
use raylib::prelude::*;
use sfx::Sfx;

pub mod bgm;
pub mod sfx;

pub struct Assets {
    textures: HashMap<&'static str, Texture2D>,
    pub font: Font,
}

pub struct AudioAssets<'a> {
    // INFO : BGM
    pub bgm: Vec<BGM<'a>>,

    // INFO : SFX
    pub select_sfx: Sfx<'a>,
}

impl<'a> AudioAssets<'a> {
    pub fn new(audio: &'a RaylibAudio) -> Self {
        let mut bgm = Vec::with_capacity(13);
        bgm.push(BGM::new(
            "Dream more than Scarlet",
            "Zun - Remixed by UnknownRori",
            "Typical Touhou Project title screen",
            "./assets/bgm/dream-more-than-scarlet.ogg",
            audio,
        ));
        bgm.push(BGM::new(
            "Ground as Red as Cherry",
            "Zun - Remixed by UnknownRori",
            "Stage theme lorem ipsum (I don't have time to tell you this)",
            "./assets/bgm/ground-as-red-as-cherry.ogg",
            audio,
        ));
        bgm.push(BGM::new(
            "Desire Drive",
            "Zun - Remixed by UnknownRori",
            "Stage theme lorem ipsum (I don't have time to tell you this)",
            "./assets/bgm/desire-drive.ogg",
            audio,
        ));
        bgm.push(BGM::new(
            "Love Colored Master Spark",
            "Zun - Remixed by UnknownRori",
            "Marisa's theme",
            "./assets/bgm/master-spark.ogg",
            audio,
        ));

        bgm.push(BGM::new(
            "Bad Apple",
            "Zun - Remixed by UnknownRori",
            "It's iconic Touhou Moment",
            "./assets/bgm/bad-apple.ogg",
            audio,
        ));

        bgm.push(BGM::new(
            "Strawberry Crisis",
            "Zun - Remixed by UnknownRori",
            "Yumemi's We are doomed",
            "./assets/bgm/strawberry-crisis.ogg",
            audio,
        ));

        bgm.push(BGM::new(
            "Locked Girl ~ The Girl's Secrets Room",
            "Zun - Remixed by UnknownRori",
            "Patchy's theme",
            "./assets/bgm/locked.ogg",
            audio,
        ));

        bgm.push(BGM::new(
            "Eastern Judgement in Sixtieth Year ~ Fate of 60 Years",
            "Zun - Remixed by UnknownRori",
            "Eiki's Theme, you are getting punished",
            "./assets/bgm/eastern-judgement.ogg",
            audio,
        ));

        bgm.push(BGM::new(
            "Shoutoku Legend ~ True Administrator",
            "Zun - Remixed by UnknownRori",
            "Miko's theme",
            "./assets/bgm/true-administrator.ogg",
            audio,
        ));

        bgm.push(BGM::new(
            "Tonight Stars an Easygoing Egoist (Live ver)",
            "Zun - Remixed by UnknownRori",
            "Joon & Shion Yorigami's theme",
            "./assets/bgm/easygoing-egoist.ogg",
            audio,
        ));

        bgm.push(BGM::new(
            "Pure Furies ~ Whereabouts of the Heart",
            "Zun - Remixed by UnknownRori",
            "Junko's theme Not all song benefited solo",
            "./assets/bgm/pure-furies.ogg",
            audio,
        ));

        bgm.push(BGM::new(
            "U.N. Owen Was Her",
            "Zun - Remixed by UnknownRori",
            "Flandre Scarlet's theme, are you scared?",
            "./assets/bgm/un-owen.ogg",
            audio,
        ));

        bgm.push(BGM::new(
            "Crimson Bedievere",
            "Zun - Remixed by UnknownRori",
            "The end?",
            "./assets/bgm/crimson-bedievere.ogg",
            audio,
        ));

        let select_sfx = Sfx::new("./assets/sfx/select.ogg", audio);

        Self { select_sfx, bgm }
    }
}

impl Assets {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut textures = HashMap::new();
        // INFO : Texture
        let main_menu = rl
            .load_texture(thread, "./assets/ui/main-menu.png")
            .expect("[-] File not found!");
        textures.insert("main_menu", main_menu);
        let stg1 = rl
            .load_texture(thread, "./assets/backgrounds/stage1/bg.png")
            .expect("[-] File not found!");
        textures.insert("stg1", stg1);

        // INFO : Characters
        let dummy_char = rl
            .load_texture(thread, "./assets/characters/filler.png")
            .expect("[-] File not found!");
        textures.insert("dummy_char", dummy_char);
        let reimu_char = rl
            .load_texture(thread, "./assets/characters/reimu/reimu.png")
            .expect("[-] File not found!");
        textures.insert("reimu_char", reimu_char);
        let miko_char = rl
            .load_texture(thread, "./assets/characters/miko/miko.png")
            .expect("[-] File not found!");
        textures.insert("miko_char", miko_char);

        // INFO : Sprite
        let commons_sprite = rl
            .load_texture(thread, "./assets/sprites/commons.png")
            .expect("[-] File not found!");
        textures.insert("commons_sprite", commons_sprite);

        let reimu_sprite = rl
            .load_texture(thread, "./assets/sprites/reimu/reimu.png")
            .expect("[-] File not found!");
        textures.insert("reimu_sprite", reimu_sprite);

        let miko_sprite = rl
            .load_texture(thread, "./assets/sprites/miko/miko.png")
            .expect("[-] File not found!");
        textures.insert("miko_sprite", miko_sprite);
        let fairy_sprite = rl
            .load_texture(thread, "./assets/sprites/fairy/fairy.png")
            .expect("[-] File not found!");
        textures.insert("fairy_sprite", fairy_sprite);

        // INFO : Font
        let font = rl
            .load_font(thread, "./assets/fonts/pc-9800-bold.ttf")
            .expect("[-] File not found!");

        Self { textures, font }
    }

    pub fn get(&self, name: &str) -> &Texture2D {
        self.textures.get(name).unwrap()
    }
}
