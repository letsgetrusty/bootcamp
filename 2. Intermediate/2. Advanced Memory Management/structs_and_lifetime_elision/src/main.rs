struct Tweet<'a> {
    content: &'a str,
}

// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime
//    is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is
//    &self or &mut self, the lifetime of self is assigned to all output
//    lifetime parameters. 

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

fn main() {
    let mut tweet = Tweet {
        content: "example",
    };
    let old_content = tweet.replace_content("replace_example");
    println!("{old_content}");
    println!("{}", tweet.content);
}

fn take_and_return_content<'a>(content: &'a str, content2: &'a str) -> &'a str {
    content
}
