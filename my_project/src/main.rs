fn most_frequent_word(text: &str) -> (String, usize) {
    
    
    let split_word: Vec<&str> = text.split(' ').collect();
    

    let mut result = (split_word[0], 0);

    for i in 0..split_word.len(){
        let v: Vec<&str> = text.matches(split_word[i]).collect();
        if v.len() > result.1{
            result.0 = split_word[i];
            result.1 = v.len();
        }
        
    }


    ((result.0).to_string(), result.1)
    //(max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}