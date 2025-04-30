pub static TOKENLIST: &str = "(){}[],.:;*+-/@#$\"\' =^~|%<>\n";

pub fn contains(elem: char, list: &str) -> bool {
    for i in list.chars() {
        if i.to_string() == elem.to_string() {
            return true;
        }
    }
    false
}

pub fn tokenize(code: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut sptokens: Vec<char> = Vec::new();

    for tk in code.chars() {
        if contains(tk, TOKENLIST) {
            if !sptokens.is_empty() {
                let tmp: String = sptokens.iter().collect();
                tokens.push(tmp);
                sptokens = Vec::new();
            }
            tokens.push(tk.to_string());
        } else {
            sptokens.push(tk);
        }
    }
    if !sptokens.is_empty() {
        let tmp: String = sptokens.iter().collect();
        tokens.push(tmp);
        sptokens = Vec::new();
    }
    tokens
}
