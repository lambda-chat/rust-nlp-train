use std::{path::PathBuf, sync::Arc};
use sudachi::{
    analysis::{stateful_tokenizer::StatefulTokenizer, Mode},
    config::Config,
    dic::dictionary::JapaneseDictionary,
};

fn main() {
    let config_file = Some(PathBuf::from("../sudachi/resources/sudachi.json"));
    let dictionary_path = Some(PathBuf::from("../sudachi/resources/system.dic"));
    let config =
        Config::new(config_file, None, dictionary_path).expect("Failed to read config file(s)");
    let dict =
        Arc::new(JapaneseDictionary::from_cfg(&config).expect("Failed to make a dictionary"));
    let morphs = {
        let mut tokenizer = StatefulTokenizer::create(dict.clone(), false, Mode::A);
        tokenizer.reset().push_str("外国人参政権".into());
        tokenizer.do_tokenize().ok().unwrap();
        tokenizer.into_morpheme_list().ok().unwrap()
    };

    for idx in 0..morphs.len() {
        let morph = morphs.get(idx);
        println!("{}\t{:?}", morph.surface(), morph.part_of_speech());
    }
}
