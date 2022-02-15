const r7z = require("./r7z/r7z.node")

console.log("r7z = ", r7z);

r7z.init();

let begin = Date.now();

let count = r7z.open("./2.zip");
for (let i = 0; i < count; ++i) {
    let name = r7z.fileName(i);
    let data = r7z.fileData(i);

    console.log(`${i}: name = ${name}, data.len = ${data.byteLength}`);
}
r7z.close();

let end = Date.now();

console.log(`time = ${end - begin} ms`);