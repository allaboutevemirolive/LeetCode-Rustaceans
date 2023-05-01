const { chromium } = require('playwright');
const fs = require('fs');


async function getLinksFree(page) {
    const matches = await page.$$eval('a[href*="/problems/"][class^="h-5 hover:text-blue-s"]', (links) =>
        links
            .filter((link) => {
                const row = link.closest('div[role="row"]');
                return (
                    row &&
                    row.querySelector('a[href*="/problems/"][class^="h-5 hover:text-blue-s"]') === link
                );
            })
            .map((link) => link.href)
    );
    return matches;
}


async function writeFile(matches, file_name) {
    fs.writeFile(`${file_name}.txt`, matches.join('\n'), (err) => {
        if (err) {
            throw err;
        }
        console.log(`File ${file_name} saved.`);
    });
}

(async () => {
    const browser = await chromium.launch();
    const page = await browser.newPage();

    const levelOnTxt = 'Medium';

    const levelOnLink = "MEDIUM";


    // Page number starts from 1
    for (let i = 2; i <= 2; i++) {

        const targetUrl = `https://leetcode.com/problemset/all/?difficulty=${levelOnLink}&page=${i}`;

        await page.goto(targetUrl);

        await page.waitForTimeout(5000);

        const matches = await getLinksFree(page);

        const file_name = `LeetCode-${levelOnTxt}-Page-` + i;

        await writeFile(matches, file_name);

    }
    await new Promise(() => { });
})();
