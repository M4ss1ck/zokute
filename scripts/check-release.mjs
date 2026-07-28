import { execSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = join(__dirname, "..");

function checks() {
  const results = [];
  let pass = true;

  function ok(msg) { results.push(`  \u2713 ${msg}`); }
  function fail(msg) { results.push(`  \u2717 ${msg}`); pass = false; }
  function header(h) { results.push(`\n${h}`); }

  // Configuration files to check
  const files = {
    "tauri.conf.json": join(ROOT, "src-tauri", "tauri.conf.json"),
    "Cargo.toml": join(ROOT, "src-tauri", "Cargo.toml"),
    "package.json": join(ROOT, "package.json"),
  };

  header("=== Version consistency ===");
  const versions = {};
  for (const [name, path] of Object.entries(files)) {
    if (!existsSync(path)) { fail(`${name} not found`); continue; }
    const content = readFileSync(path, "utf8");
    let v;
    if (name === "Cargo.toml") {
      const m = content.match(/^version\s*=\s*"([^"]+)"/m);
      v = m ? m[1] : null;
    } else {
      const json = JSON.parse(content);
      v = json.version;
    }
    if (v) {
      versions[name] = v;
      ok(`${name}: ${v}`);
    } else {
      fail(`${name}: no version found`);
    }
  }
  const unique = new Set(Object.values(versions));
  if (unique.size > 1) {
    fail(`Version mismatch: ${JSON.stringify(versions)}`);
  } else if (unique.size === 1) {
    ok(`All versions agree: ${[...unique][0]}`);
  }

  header("=== Identifier ===");
  const tauriConfig = JSON.parse(readFileSync(files["tauri.conf.json"], "utf8"));
  if (tauriConfig.identifier === "io.github.m4ss1ck.zokute") {
    ok("Identifier is correct");
  } else {
    fail(`Identifier is ${tauriConfig.identifier}`);
  }

  header("=== Build ===");
  try {
    const out = execSync("pnpm build 2>&1", { cwd: ROOT, encoding: "utf8" });
    ok("Frontend build succeeds");
  } catch (e) {
    fail(`Frontend build failed: ${e.message}`);
  }
  try {
    const out = execSync("cargo build --manifest-path src-tauri/Cargo.toml 2>&1", { encoding: "utf8" });
    ok("Rust build succeeds");
  } catch (e) {
    fail(`Rust build failed: ${e.message}`);
  }

  header("=== Tests ===");
  try {
    execSync("pnpm test:run 2>&1", { cwd: ROOT, encoding: "utf8" });
    ok("Frontend tests pass");
  } catch (e) {
    fail(`Frontend tests failed`);
  }
  try {
    execSync("cargo test --manifest-path src-tauri/Cargo.toml 2>&1", { encoding: "utf8" });
    ok("Rust tests pass");
  } catch (e) {
    fail(`Rust tests failed`);
  }

  header("=== Package ===");
  const debDir = join(ROOT, "src-tauri", "target", "release", "bundle", "deb");
  if (existsSync(debDir)) {
    const debs = execSync(`ls "${debDir}"/*.deb 2>/dev/null || true`, { encoding: "utf8" }).trim();
    if (debs) {
      ok(`Debian package found: ${debs}`);
      for (const deb of debs.split("\n")) {
        try {
          const info = execSync(`dpkg-deb --info "${deb}"`, { encoding: "utf8" });
          ok(`${deb}: metadata valid`);
        } catch (e) {
          fail(`${deb}: dpkg-deb failed: ${e.message}`);
        }
      }
    } else {
      fail("No .deb found");
    }
  } else {
    fail("bundle/deb directory not found");
  }

  header("=== Docs examples ===");
  const examplesDir = join(ROOT, "docs", "examples");
  if (existsSync(examplesDir)) {
    const examples = execSync(`ls "${examplesDir}"/*.toml 2>/dev/null || true`, { encoding: "utf8" }).trim();
    if (examples) {
      ok(`Found ${examples.split("\n").length} doc examples`);
    } else {
      fail("No doc examples found");
    }
  } else {
    fail("docs/examples directory not found");
  }

  header("=== No telemetry ===");
  const grep = execSync("rg -l 'telemetry\\|analytics\\|tracker\\|amplitude\\|segment\\|sentry' --include='*.rs' --include='*.ts' --include='*.tsx' --include='*.toml' --include='*.json' src/ src-tauri/ 2>/dev/null || true", { encoding: "utf8" }).trim();
  if (grep) {
    fail(`Potential telemetry found in:\n${grep}`);
  } else {
    ok("No telemetry references");
  }

  header("=== Privacy scan ===");
  const secrets = execSync("rg -l 'api_key\\|api_secret\\|token\\|password\\|secret' --include='*.rs' --include='*.ts' --include='*.tsx' src/ src-tauri/src/ 2>/dev/null || true", { encoding: "utf8" }).trim();
  if (secrets) {
    fail(`Potential secrets found in:\n${secrets}`);
  } else {
    ok("No secrets in source");
  }

  results.push("");
  results.push(pass ? "\u2713 All checks passed" : "\u2717 Some checks failed");

  console.log(results.join("\n"));
  process.exit(pass ? 0 : 1);
}

checks();
