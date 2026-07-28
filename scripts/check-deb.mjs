import { execSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

var __dirname = dirname(fileURLToPath(import.meta.url));
var ROOT = join(__dirname, "..");

function findDebs() {
  var dir = join(ROOT, "src-tauri", "target", "release", "bundle", "deb");
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter(function(f) { return f.endsWith(".deb"); }).map(function(f) { return join(dir, f); });
}

function checks() {
  var results = [];
  var pass = true;

  function ok(msg) { results.push("  OK " + msg); }
  function fail(msg) { results.push("  FAIL " + msg); pass = false; }

  results.push("=== Package existence ===");
  var debFiles = findDebs();
  if (debFiles.length === 0) {
    fail("No .deb found in bundle/deb/ - run pnpm build:deb first");
  } else {
    ok("Found " + debFiles.length + " .deb file(s)");
  }

  for (var i = 0; i < debFiles.length; i++) {
    var p = debFiles[i];
    var name = p.split("/").pop();
    results.push("");
    results.push("=== " + name + " ===");
    try {
      var contents = execSync("dpkg-deb --contents " + JSON.stringify(p), { encoding: "utf8" });
      if (contents.includes("usr/bin/") || contents.includes("usr/lib/")) {
        ok("contains binary");
      } else {
        fail("missing binary");
      }
      if (contents.includes(".desktop")) {
        ok("contains .desktop entry");
      } else {
        fail("missing .desktop entry");
      }
    } catch (e) {
      fail("dpkg-deb failed: " + e.message);
    }

    try {
      var info = execSync("dpkg-deb --info " + JSON.stringify(p), { encoding: "utf8" });
      if (info.includes("Package:")) {
        ok("has Package field");
      } else {
        fail("missing Package field");
      }
      if (info.includes("Version:")) ok("has version");
      else fail("missing version");
    } catch (e) {
      fail("dpkg-deb info failed: " + e.message);
    }
  }

  results.push("");
  results.push("=== Identifier check ===");
  var config = JSON.parse(readFileSync(join(ROOT, "src-tauri", "tauri.conf.json"), "utf8"));
  if (config.identifier === "io.github.m4ss1ck.zokute") {
    ok("tauri.conf.json identifier is correct");
  } else {
    fail("identifier is " + config.identifier + ", expected io.github.m4ss1ck.zokute");
  }

  results.push("");
  results.push(pass ? "PASS All checks passed" : "FAIL Some checks failed");
  console.log(results.join("\n"));
  process.exit(pass ? 0 : 1);
}

checks();
