mod utils;
mod parallel;
use std::path::{Path, PathBuf};
use colored::Colorize;
fn main() {
    let code_base = Path::new("/home/lordcasser/workspace/galaxy/sample");
    let code_path = utils::code_path_seek(code_base, false);
    let rule_base =  Path::new("/home/lordcasser/workspace/galaxy/rules");
    let rule_path = utils::rule_path_seek(rule_base);
    let mut scanner = parallel::Scanner::new(rule_path, code_path, 2, false);
    let results =  scanner.wait_for_result();
    for i in results.iter(){
        println!("{} {}","[+]issue:".blue(),i.issue.blue());
        for (path,code) in i.issue_code.iter(){
            println!("{}\n{}",path,code);
        }
    }
}
