export const defaultData = `\
Scraobox To Markdown Converter

#scrapbox #obsidian #rust #webassembly

This is a hobby project to import [Scrapbox https://scrapbox.io] to [Obsidian https://obsidian.md/].
It aims to be able to convert Scrapbox and Markdown to each other,

Scrapbox syntax parser is written in Rust by using [nom https://github.com/Geal/nom].
And then, It is compiled into WebAssembly and run in the browser.

[*** Supported Syntax]

[** List]
\tnormal
\t\tnormal
\t1. decimal
\t2. decimal
\t3. decimal

[** Heading]
\t\`[* bold]\` is converted to Heading

[** Emphasis]
\t[* bold]
\t[/ italic]
\t[- strikethrough]
\t[*-/ mix]
\t\`println("Hello World!");\`

[** Tag]
\t#rust #scrapbox #obsidian #webassembly

[** Link]
\t[internal-link]
\t[Scrapbox https://scrapbox.io]
\t[https://scrapbox.io Scrapbox]
\t[https://scrapbox.io]
`;
