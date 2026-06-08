import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const rootDirectory = path.resolve(scriptDirectory, "..");
const packageJsonPath = path.join(rootDirectory, "package.json");
const cargoTomlPath = path.join(rootDirectory, "Cargo.toml");

const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
const version = packageJson.version;

const cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
const pattern = /^version = "[^"]+"/m;
if (!pattern.test(cargoToml)) {
  throw new Error("Could not find version field in Cargo.toml");
}
const updated = cargoToml.replace(pattern, `version = "${version}"`);
fs.writeFileSync(cargoTomlPath, updated);

console.log(`Synced uss-language-server version to ${version}`);
