use crate::tomlread::{ColorFormat, FileTypeToml, FontFormat};
use std::collections::HashMap;

pub fn creat_default() -> HashMap<String, FileTypeToml> {
    let mut default_hash: HashMap<String, FileTypeToml> = HashMap::new();
    let dir = FileTypeToml::new(
        String::from("dir"),
        String::from(""),
        ColorFormat::White,
        None,
        FontFormat::Bold,
        vec![String::from("dir")],
    );
    default_hash.insert(dir.track[0].clone(), dir);
    let sym = FileTypeToml::new(
        String::from("sym"),
        String::from(""),
        ColorFormat::Red,
        None,
        FontFormat::Bold,
        vec![String::from("sym")],
    );
    default_hash.insert(sym.track[0].clone(), sym);
    let default = FileTypeToml::new(
        String::from("default"),
        String::from(""),
        ColorFormat::White,
        None,
        FontFormat::Regular,
        vec![String::from("default")],
    );
    default_hash.insert(default.track[0].clone(), default);
    let python = FileTypeToml::new(
        String::from("python"),
        String::from(""),
        ColorFormat::Yellow,
        None,
        FontFormat::Regular,
        vec![String::from("*.py"), String::from("*.pyc")],
    );
    default_hash.insert(python.track[0].clone(), python.clone());
    default_hash.insert(python.track[1].clone(), python);
    let git_folder = FileTypeToml::new(
        String::from("git folder"),
        String::from(""),
        ColorFormat::BrightRed,
        None,
        FontFormat::Bold,
        vec![String::from(".git"), String::from(".gitignore")],
    );
    default_hash.insert(git_folder.track[0].clone(), git_folder.clone());
    default_hash.insert(git_folder.track[1].clone(), git_folder);
    let rust = FileTypeToml::new(
        String::from("rust"),
        String::from(""),
        ColorFormat::BrightRed,
        None,
        FontFormat::Regular,
        vec![String::from("*.rs")],
    );
    default_hash.insert(rust.track[0].clone(), rust);
    let toml = FileTypeToml::new(
        String::from("toml"),
        String::from(""),
        ColorFormat::Blue,
        None,
        FontFormat::Regular,
        vec![String::from("*.toml")],
    );
    default_hash.insert(toml.track[0].clone(), toml);
    let c = FileTypeToml::new(
        String::from("c"),
        String::from(""),
        ColorFormat::Blue,
        None,
        FontFormat::Regular,
        vec![String::from("*.c"), String::from("*.h")],
    );
    default_hash.insert(c.track[0].clone(), c.clone());
    default_hash.insert(c.track[1].clone(), c);
    let docker = FileTypeToml::new(
        String::from("docker"),
        String::from(""),
        ColorFormat::BrightBlue,
        None,
        FontFormat::Regular,
        vec![String::from("Dockerfile")],
    );
    default_hash.insert(docker.track[0].clone(), docker);
    let go = FileTypeToml::new(
        String::from("go"),
        String::from("󰟓"),
        ColorFormat::BrightBlue,
        None,
        FontFormat::Regular,
        vec![String::from("*.go")],
    );
    default_hash.insert(go.track[0].clone(), go);
    let haskel = FileTypeToml::new(
        String::from("haskel"),
        String::from(""),
        ColorFormat::Magenta,
        None,
        FontFormat::Regular,
        vec![String::from("*.hs")],
    );
    default_hash.insert(haskel.track[0].clone(), haskel);
    let java = FileTypeToml::new(
        String::from("java"),
        String::from(""),
        ColorFormat::Red,
        None,
        FontFormat::Regular,
        vec![String::from("*.java")],
    );
    default_hash.insert(java.track[0].clone(), java);
    let julia = FileTypeToml::new(
        String::from("julia"),
        String::from(""),
        ColorFormat::Green,
        None,
        FontFormat::Regular,
        vec![String::from("*.jl")],
    );
    default_hash.insert(julia.track[0].clone(), julia);
    let kotlin = FileTypeToml::new(
        String::from("kotlin"),
        String::from(""),
        ColorFormat::Cyan,
        None,
        FontFormat::Regular,
        vec![String::from("*.kt"), String::from("*.kts")],
    );
    default_hash.insert(kotlin.track[0].clone(), kotlin.clone());
    default_hash.insert(kotlin.track[1].clone(), kotlin.clone());
    let lua = FileTypeToml::new(
        String::from("lua"),
        String::from(""),
        ColorFormat::Blue,
        None,
        FontFormat::Regular,
        vec![String::from("*.lua")],
    );
    default_hash.insert(lua.track[0].clone(), lua);
    let ocaml = FileTypeToml::new(
        String::from("ocaml"),
        String::from(""),
        ColorFormat::BrightRed,
        None,
        FontFormat::Regular,
        vec![String::from("*.opam")],
    );
    default_hash.insert(ocaml.track[0].clone(), ocaml);
    let perl = FileTypeToml::new(
        String::from("perl"),
        String::from(""),
        ColorFormat::BrightBlue,
        None,
        FontFormat::Regular,
        vec![String::from("*.pl")],
    );
    default_hash.insert(perl.track[0].clone(), perl);
    let php = FileTypeToml::new(
        String::from("php"),
        String::from(""),
        ColorFormat::Blue,
        None,
        FontFormat::Regular,
        vec![String::from("*.php")],
    );
    default_hash.insert(php.track[0].clone(), php);
    let ruby = FileTypeToml::new(
        String::from("ruby"),
        String::from(""),
        ColorFormat::Red,
        None,
        FontFormat::Regular,
        vec![String::from("*.rb")],
    );
    default_hash.insert(ruby.track[0].clone(), ruby);
    let r = FileTypeToml::new(
        String::from("r"),
        String::from(""),
        ColorFormat::Blue,
        None,
        FontFormat::Regular,
        vec![
            String::from("*.R"),
            String::from("*.Rd"),
            String::from("*.Rmd"),
            String::from("*.Rproj"),
            String::from("*.Rxs"),
        ],
    );
    default_hash.insert(r.track[0].clone(), r.clone());
    default_hash.insert(r.track[1].clone(), r.clone());
    default_hash.insert(r.track[2].clone(), r.clone());
    default_hash.insert(r.track[3].clone(), r.clone());
    default_hash.insert(r.track[4].clone(), r);
    let swift = FileTypeToml::new(
        String::from("swift"),
        String::from(""),
        ColorFormat::BrightRed,
        None,
        FontFormat::Regular,
        vec![String::from("*.swift")],
    );
    default_hash.insert(swift.track[0].clone(), swift);
    let zig = FileTypeToml::new(
        String::from("zig"),
        String::from(""),
        ColorFormat::Yellow,
        None,
        FontFormat::Regular,
        vec![String::from("*.zig")],
    );
    default_hash.insert(zig.track[0].clone(), zig);
    let javascript = FileTypeToml::new(
        String::from("javascript"),
        String::from(""),
        ColorFormat::Yellow,
        None,
        FontFormat::Regular,
        vec![String::from("*.js")],
    );
    default_hash.insert(javascript.track[0].clone(), javascript);
    let html = FileTypeToml::new(
        String::from("html"),
        String::from(""),
        ColorFormat::BrightCyan,
        None,
        FontFormat::Regular,
        vec![String::from("*.html")],
    );
    default_hash.insert(html.track[0].clone(), html);
    let css = FileTypeToml::new(
        String::from("css"),
        String::from(""),
        ColorFormat::BrightYellow,
        None,
        FontFormat::Regular,
        vec![String::from("*.css")],
    );
    default_hash.insert(css.track[0].clone(), css);
    let cpp = FileTypeToml::new(
        String::from("C++"),
        String::from(""),
        ColorFormat::Blue,
        None,
        FontFormat::Regular,
        vec![String::from("*.cpp")],
    );
    default_hash.insert(cpp.track[0].clone(), cpp);
    let cs = FileTypeToml::new(
        String::from("C#"),
        String::from("󰌛"),
        ColorFormat::BrightYellow,
        None,
        FontFormat::Regular,
        vec![String::from("*.cs")],
    );
    default_hash.insert(cs.track[0].clone(), cs);
    default_hash
}
