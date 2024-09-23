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
        let textures = HashMap::new();
        let font = rl
            .load_font(thread, "./assets/fonts/pc-9800-bold.ttf")
            .expect("[-] File not found!");
        let mut assets = Self { textures, font };

        assets.load_textures(rl, thread, "./assets/ui/main-menu.png", "main_menu");
        assets.load_textures(rl, thread, "./assets/ui/stage-view.png", "stage_view");
        assets.load_textures(rl, thread, "./assets/backgrounds/stage1/bg.png", "stg1");

        assets.load_textures(rl, thread, "./assets/characters/filler.png", "dummy_char");
        assets.load_textures(
            rl,
            thread,
            "./assets/characters/reimu/reimu.png",
            "reimu_char",
        );
        assets.load_textures(rl, thread, "./assets/characters/miko/miko.png", "miko_char");

        assets.load_textures(rl, thread, "./assets/sprites/commons.png", "commons_sprite");
        assets.load_textures(
            rl,
            thread,
            "./assets/sprites/reimu/reimu.png",
            "reimu_sprite",
        );
        assets.load_textures(rl, thread, "./assets/sprites/miko/miko.png", "miko_sprite");
        assets.load_textures(
            rl,
            thread,
            "./assets/sprites/fairy/fairy.png",
            "fairy_sprite",
        );

        assets
    }

    pub fn get(&self, name: &str) -> &Texture2D {
        self.textures.get(name).unwrap()
    }

    pub fn load_textures(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        name: &'static str,
    ) {
        let tex = rl.load_texture(thread, path).expect("[-] File not found!");
        tex.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_POINT);
        self.textures.insert(name, tex);
    }
}
