import * as fs from "fs";
import * as path from "path";

const targetDir = process.argv[2];
const target = process.argv[3];
const originalName = targetDir.split("/").pop() || "binary";

console.log("📦 Rename Binary Script");
console.log(`Target Directory: ${targetDir}`);
console.log(`Target Platform: ${target}`);
console.log(`Original Name: ${originalName}`);

const isWindows = target.includes("windows");
const originalExt = isWindows ? ".exe" : "";
const targetExt = isWindows ? ".exe" : "";

// Fixed path to match the actual output structure
const originalPath = path.join(
  targetDir,
  target,
  "release",
  `${originalName}${originalExt}`,
);
const newDir = path.join(targetDir);
const newPath = path.join(newDir, `${originalName}-${target}${targetExt}`);

console.log("\nPaths:");
console.log(`Looking for original binary at: ${originalPath}`);
console.log(`Will rename to: ${newPath}`);

// Ensure the target directory exists
if (!fs.existsSync(newDir)) {
  console.log(`\nCreating directory: ${newDir}`);
  fs.mkdirSync(newDir, { recursive: true });
}

if (fs.existsSync(originalPath)) {
  console.log("\n✅ Found original binary");
  fs.renameSync(originalPath, newPath);
  console.log(`✅ Successfully renamed to: ${newPath}`);
} else {
  console.error(`\n❌ Error: Original binary not found at: ${originalPath}`);
  console.error("Directory contents:");
  try {
    const parentDir = path.dirname(originalPath);
    if (fs.existsSync(parentDir)) {
      console.error(fs.readdirSync(parentDir));
    } else {
      console.error(`Parent directory ${parentDir} does not exist`);
    }
  } catch (err) {
    console.error("Could not list directory contents:", err);
  }
  process.exit(1);
}
