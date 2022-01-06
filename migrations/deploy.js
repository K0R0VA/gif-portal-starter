const appName = "gif_portal";
const fs = require("fs");
const idl = require(`./target/idl/${appName}.json`);
fs.writeFileSync("./app/src/idl.json", JSON.stringify(idl));
const account = anchor.web3.Keypair.generate();
fs.writeFileSync("./app/keypair.json", JSON.stringify(account));
