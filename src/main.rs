extern crate curl;

use std::env;
use curl::easy::Easy;
use std::process;


pub fn get_gitignore(orig_name: String) -> String {
    let names = vec![
        "Actionscript",
        "Ada",
        "Agda",
        "Android",
        "AppceleratorTitanium",
        "AppEngine",
        "ArchLinuxPackages",
        "Autotools",
        "C++",
        "C",
        "CakePHP",
        "CFWheels",
        "ChefCookbook",
        "Clojure",
        "CMake",
        "CodeIgniter",
        "CommonLisp",
        "Composer",
        "Concrete5",
        "Coq",
        "CraftCMS",
        "CUDA",
        "D",
        "Dart",
        "Delphi",
        "DM",
        "Drupal",
        "Eagle",
        "Elisp",
        "Elixir",
        "Elm",
        "EPiServer",
        "Erlang",
        "ExpressionEngine",
        "ExtJs",
        "Fancy",
        "Finale",
        "ForceDotCom",
        "Fortran",
        "FuelPHP",
        "Gcov",
        "GitBook",
        "Anjuta",
        "Ansible",
        "Archives",
        "Bazaar",
        "BricxCC",
        "Calabash",
        "Cloud9",
        "CodeKit",
        "CVS",
        "DartEditor",
        "Dreamweaver",
        "Dropbox",
        "Eclipse",
        "EiffelStudio",
        "Emacs",
        "Ensime",
        "Espresso",
        "FlexBuilder",
        "GPG",
        "JDeveloper",
        "JEnv",
        "JetBrains",
        "Kate",
        "KDevelop4",
        "Lazarus",
        "LibreOffice",
        "Linux",
        "LyX",
        "macOS",
        "Matlab",
        "Mercurial",
        "MicrosoftOffice",
        "ModelSim",
        "Momentics",
        "MonoDevelop",
        "NetBeans",
        "Ninja",
        "NotepadPP",
        "Otto",
        "Redcar",
        "Redis",
        "SBT",
        "SlickEdit",
        "Stata",
        "SublimeText",
        "SVN",
        "SynopsysVCS",
        "Tags",
        "TextMate",
        "TortoiseGit",
        "Vagrant",
        "Vim",
        "VirtualEnv",
        "VisualStudioCode",
        "WebMethods",
        "Windows",
        "Xcode",
        "XilinxISE",
        "Go",
        "Gradle",
        "Grails",
        "GWT",
        "Haskell",
        "Idris",
        "IGORPro",
        "Java",
        "Jboss",
        "Jekyll",
        "Joomla",
        "Julia",
        "KiCad",
        "Kohana",
        "Kotlin",
        "LabVIEW",
        "Laravel",
        "Leiningen",
        "LemonStand",
        "Lilypond",
        "Lithium",
        "Lua",
        "Magento",
        "Maven",
        "Mercury",
        "MetaProgrammingSystem",
        "Nanoc",
        "Nim",
        "Node",
        "Objective-C",
        "OCaml",
        "Opa",
        "OpenCart",
        "OracleForms",
        "Packer",
        "Perl",
        "Phalcon",
        "PlayFramework",
        "Plone",
        "Prestashop",
        "Processing",
        "PureScript",
        "Python",
        "Qooxdoo",
        "Qt",
        "R",
        "Rails",
        "RhodesRhomobile",
        "ROS",
        "Ruby",
        "Rust",
        "Sass",
        "Scala",
        "Scheme",
        "SCons",
        "Scrivener",
        "Sdcc",
        "SeamGen",
        "SketchUp",
        "Smalltalk",
        "Stella",
        "SugarCRM",
        "Swift",
        "Symfony",
        "SymphonyCMS",
        "Terraform",
        "TeX",
        "Textpattern",
        "TurboGears2",
        "Typo3",
        "Umbraco",
        "Unity",
        "UnrealEngine",
        "VisualStudio",
        "VVVV",
        "Waf",
        "WordPress",
        "Xojo",
        "Yeoman",
        "Yii",
        "ZendFramework",
        "Zephir",
    ];
    // check if passed arg matches existing name
    // use that name to get gitignore file it does, otherwise proceed
    // with the provided gitignore filename
    let mut name = orig_name.to_string();
    for n in names.iter() {
        if n.to_string().to_lowercase() == orig_name.to_string().to_lowercase() {
            name = n.to_string();
            break;
        } 
    }
    let base_url = "https://raw.githubusercontent.com/github/gitignore/master/";
    
    let mut url = format!("{}{}.gitignore", base_url, name);
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.perform().unwrap();
    if easy.response_code().unwrap() != 200 {
        // check in Global
        url = format!("{}Global/{}.gitignore", base_url, name);
        easy.url(&url).unwrap();
    }

    let mut html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            html = String::from_utf8(Vec::from(data)).unwrap();       
            Ok(data.len())
        }).unwrap(); 

        transfer.perform().unwrap();
    }
    return html
}


#[cfg(test)]
const NOT_FOUND: &'static str = "404: Not Found\n";


#[test]
fn get_gitignore_returns_404_on_gobblygoop() {
    let resp = get_gitignore("gobblygoop".to_string());
    assert_eq!(resp, NOT_FOUND);
    let resp = get_gitignore("thisdefinitelyshouldnotexist".to_string());
    assert_eq!(resp, NOT_FOUND);
}


#[test]
fn get_gitignore_finds_macos() {
    let resp = get_gitignore("macos".to_string());
    assert_ne!(resp, NOT_FOUND);
    let resp = get_gitignore("macOS".to_string());
    assert_ne!(resp, NOT_FOUND);
    let resp = get_gitignore("macOs".to_string());
    assert_ne!(resp, NOT_FOUND);
}


#[test]
fn get_gitignore_finds_rust() {
    let resp = get_gitignore("rust".to_string());
    assert_ne!(resp, NOT_FOUND);
    let resp = get_gitignore("Rust".to_string());
    assert_ne!(resp, NOT_FOUND);
}


#[test]
fn get_gitignore_find_mixed_case_examples() {
    let resp = get_gitignore("AppEngine".to_string());
    assert_ne!(resp, NOT_FOUND);
    let resp = get_gitignore("appEngine".to_string());
    assert_ne!(resp, NOT_FOUND);
    let resp = get_gitignore("appengine".to_string());
    assert_ne!(resp, NOT_FOUND);
    let resp = get_gitignore("EpISeRvEr".to_string());
    assert_ne!(resp, NOT_FOUND);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage:\n\tignore [python|rust|go|any .gitignore on github.com/github/gitignore] > .gitignore");
        process::exit(1);
    }
    println!("\n{}", get_gitignore(args[1].to_string()));   
}
