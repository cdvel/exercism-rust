pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        _ => {
            format!("{before} bottles of beer on the wall, {before} bottles of beer.\nTake one down and pass it around, {after} bottle{plural} of beer on the wall.\n", 
                    before = n,
                    after = n-1,
                    plural = if n == 2 {""} else {"s"})
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    let mut verse_number: u32 = start;
    while verse_number >= end {
        if !song.is_empty() {
            song.push_str("\n");
        }
        song.push_str(&*verse(verse_number));
        if verse_number == 0 {
            break;
        };
        verse_number -= 1;
    }
    song.to_string()
}
