use macros_wrapper::macros::lexer_token;

lexer_token! {
	token::Token {
		WhiteSpace ignore: r#"\s"#,
		Ident: r#"[a-zA-Z\_\$][a-zA-Z\_\$0-9]*"#,
		String: r#""((\")|[^"])*""#,
		Char: r#"'.'"#,
		RawString: |string| {
			string.next_eq('r')?;
			let mut next_char;
			let mut r_count = 0;
			while (next_char = string.next()?) == '#' {
				r_count += 1;
			}
			string.next_eq('"')?;
			let r_count = r_count;
			let mut counting = false;
			let count = -1;
			while count < r_count {
				next_char = string.next()?;
				if counting {
					if next_char == '#' {
						count += 1;
					} else {
						counting = false;
					}
				} else if next_char == '"' {
					counting = true;
					count = 0;
				}
			}
		},
	}
}

fn main() {
	
}
