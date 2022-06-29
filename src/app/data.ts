export const defaultData = `\
Scraobox To Markdown Converter

#scrapbox #obsidian #rust #webassembly

This is a hobby project to import [Scrapbox https://scrapbox.io] to [Obsidian https://obsidian.md/].
It aims to be able to convert Scrapbox and Markdown to each other.

Scrapbox syntax parser is written in Rust by using [nom https://github.com/Geal/nom].
And then, It is compiled into WebAssembly and run in the browser without sending data externally.

[*** Supported Syntax]

[** List]
\tnormal
\t\tnormal
\t1. decimal
\t2. decimal
\t3. decimal

[** Heading]
\t\`[* bold]\` is converted to Heading
\tYou can configure the level mapping between Scrapbox and Markdown

[** Emphasis]
\t[* bold]
\t[[bold]]
\t[/ italic]
\t[- strikethrough]
\t[*-/ mix]

[** Tag]
\t#rust #scrapbox #obsidian #webassembly

[** Link]
\t[internal link]
\t[Scrapbox https://scrapbox.io]
\t[https://scrapbox.io Scrapbox]
\t[https://scrapbox.io]
\t[/help-jp/Scrapbox]

[** Image]
\t[https://www.rust-lang.org/static/images/rust-logo-blk.svg]
\t[https://www.rust-lang.org/ https://www.rust-lang.org/static/images/rust-logo-blk.svg]

[** Block quote and Commandline]
\t\`git\`
\t$ git log
\t% git log

[** Code block]
code:hello.rs
 fn main() {
     println!("Hello, world!");
 }

[** Table]
table:table
 1\t2\t3
 A\tB\tC
 D\tE\tF

table:table
 1\t2\t3
`;
