import fs from "node:fs";

const packageJson = JSON.parse(fs.readFileSync("package.json", "utf8"));
const version = packageJson.version;

function replaceVersion(path, pattern, replacement) {
  const original = fs.readFileSync(path, "utf8");
  if (!pattern.test(original)) {
    throw new Error(`Could not find version field in ${path}`);
  }
  const updated = original.replace(pattern, replacement(version));
  fs.writeFileSync(path, updated);
}

function updateJsonVersion(path) {
  const json = JSON.parse(fs.readFileSync(path, "utf8"));
  json.version = version;
  fs.writeFileSync(path, `${JSON.stringify(json, null, 2)}\n`);
}

replaceVersion(
  "Cargo.toml",
  /^version = "[^"]+"/m,
  (value) => `version = "${value}"`
);
replaceVersion(
  "extension.toml",
  /^version = "[^"]+"/m,
  (value) => `version = "${value}"`
);
updateJsonVersion("packages/com.gamebayoumy.zed-unity/package.json");
updateJsonVersion("crates/uss-language-server/package.json");

console.log(`Synced Zed Unity monorepo version to ${version}`);
