// const { arrayBuffer } = require("stream/consumers")
const captcha = require("./pkg")
const fs = require("fs")

const img = captcha.generate_image("93919")
console.log(img);
fs.writeFileSync("c.png", Buffer.from(img.buffer))