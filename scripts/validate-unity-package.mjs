import fs from 'node:fs';

const packagePath = 'packages/com.gamebayoumy.zed-unity/package.json';
const pkg = JSON.parse(fs.readFileSync(packagePath, 'utf8'));

const required = ['name', 'version', 'displayName', 'unity', 'repository'];
for (const key of required) {
  if (!pkg[key]) {
    throw new Error(`${packagePath} missing required field: ${key}`);
  }
}

if (pkg.name !== 'com.gamebayoumy.zed-unity') {
  throw new Error(`Unexpected Unity package name: ${pkg.name}`);
}

if (!pkg.dependencies || pkg.dependencies['com.unity.ide.visualstudio'] !== '2.0.27') {
  throw new Error('Unity package must depend on com.unity.ide.visualstudio@2.0.27');
}

console.log(`${pkg.name}@${pkg.version} manifest ok`);
