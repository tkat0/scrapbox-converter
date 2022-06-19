import init, { hex_color_js } from 'scrapbox-parser-core';

const main = async () => {
    await init();
    console.log(hex_color_js('#2F14DF'));
};

main();
