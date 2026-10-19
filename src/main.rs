fn main() {
    let mut text = String::from("Пик Балмера");
    reverse_words_at_odd_positions(&mut text);
    println!("{text}");
}

fn len_char(byte: u8) -> usize {
    if byte & 0x80 == 0 {
        1
    } else if byte & 0xE0 == 0xC0 {
        2
    } else if byte & 0xF0 == 0xE0 {
        3
    } else {
        4
    }
}

fn reverse_words_at_odd_positions(text: &mut String) {
    let bytes = unsafe { text.as_mut_vec() };
    let mut i = 0;
    let mut word_number = 0;

    while i < bytes.len() {
        let len = len_char(bytes[i]);

        let c = match std::str::from_utf8(&bytes[i..i + len]) {
            Ok(s) => match s.chars().next() {
                Some(c) => c,
                None => return,
            },
            Err(_) => return,
        };

        if c.is_whitespace() {
            i += len;
            continue;
        }

        let start = i;

        while i < bytes.len() {
            let len = len_char(bytes[i]);

            let c = match std::str::from_utf8(&bytes[i..i + len]) {
                Ok(s) => match s.chars().next() {
                    Some(c) => c,
                    None => return,
                },
                Err(_) => return,
            };

            if c.is_whitespace() {
                break;
            }

            i += len;
        }

        if word_number % 2 == 0 {
            let word = &mut bytes[start..i];
            word.reverse();

            let mut j = word.len();

            while j > 0 {
                let len = len_char(word[j - 1]);
                word[j - len..j].reverse();
                j -= len;
            }
        }

        word_number += 1;
    }
}

#[cfg(test)]
#[path = "../tests/unit/monologue.rs"]
mod monologue_tests;
