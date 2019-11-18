use std::fs::write;
use std::io;

fn main() -> io::Result<()> {
    write("index.html", HTML)?;
    write("main.js", JS)?;
    Ok(())
}

const HTML: &[u8] =
br#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>WebAssembly template</title>
    <script type="module" src="main.js"></script>
</head>
<body>
</body>
</html>
"#;

const JS: &[u8] =
br#"import init from "pkg/progect.js";

window.addEventListener("load", async () => {
    await init();
    // add your code here
});
"#;
