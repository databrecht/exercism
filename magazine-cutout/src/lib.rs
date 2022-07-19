use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = magazine.iter().fold(HashMap::new(), |mut words, &str| {
        *words.entry(str).or_insert(0) += 1;
        words
    });
    note.into_iter()
        .map(|&note_word| {
            let entry = map.entry(note_word).or_default();
            *entry -= 1;
            *entry >= 0
        })
        .all(|res| res)
}
