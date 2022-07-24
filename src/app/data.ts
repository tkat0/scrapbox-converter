export const defaultScrapboxData = `\
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

[** Math]

\t[$ \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a} ]
\t[$\\left( \\sum_{k=1}^n a_k b_k \\right)^2 \\leq \\left( \\sum_{k=1}^n a_k^2 \\right) \\left( \\sum_{k=1}^n b_k^2 \\right)]

`;

export const defaultMarkdownData = `\
Scraobox To Markdown Converter

#scrapbox #obsidian #rust #webassembly

This is a hobby project to import [Scrapbox](https://scrapbox.io) to [Obsidian](https://obsidian.md/).
It aims to be able to convert Scrapbox and Markdown to each other.

Scrapbox syntax parser is written in Rust by using [nom](https://github.com/Geal/nom).
And then, It is compiled into WebAssembly and run in the browser without sending data externally.

# Supported Syntax

## List
* normal
  * normal
1. decimal
1. decimal
1. decimal

## Heading
* \`[* bold]\` is converted to Heading
* You can configure the level mapping between Scrapbox and Markdown

## Emphasis
* **bold**
* **bold**
* *italic*
* ~~strikethrough~~
* ~~***mix***~~

## Tag
* #rust #scrapbox #obsidian #webassembly

## Link
* [[internal link]]
* [Scrapbox](https://scrapbox.io)
* [Scrapbox](https://scrapbox.io)
* https://scrapbox.io
* https://scrapbox.io/help-jp/Scrapbox

## Image
* ![](https://www.rust-lang.org/static/images/rust-logo-blk.svg)
* ![](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

## Block quote and Commandline
* \`git\`
* \`$ git log\`
* \`% git log\`

## Code block
\`\`\`hello.rs
fn main() {
    println!("Hello, world!");
}
\`\`\`

## Table
| 1 | 2 | 3 |
| --- | --- | --- |
| A | B | C |
| D | E | F |

| 1 | 2 | 3 |
| --- | --- | --- |

## Math

* $$ \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a} $$
* $$\\left( \\sum_{k=1}^n a_k b_k \\right)^2 \\leq \\left( \\sum_{k=1}^n a_k^2 \\right) \\left( \\sum_{k=1}^n b_k^2 \\right)$$

`;
