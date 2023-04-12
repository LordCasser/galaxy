use crate::utils::{self, Rule};
use crossbeam::thread::scope;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Duration;
use std::thread;
use colored::Colorize;
#[derive(Clone)]
pub struct ScanResult {
    pub issue: String,
    pub issue_code: IssueCode ,
    pub discription: String,
}

type IssueCode = Vec<(String,String)>;
// pub struct IssueCode {
//     pub code_content: Vec<String>,
//     pub code_location: String,
// }

pub struct Scanner {
    rules: VecDeque<Rule>,
    code_path: Vec<PathBuf>,
    threads: usize,
    result: Vec<ScanResult>,
    mutex: Mutex<()>,
    cpp: bool,
}

impl Scanner {
    pub fn new(rules_path: Vec<Rule>, code_path: Vec<PathBuf>, threads: usize, cpp: bool) -> Self {
        let result: Vec<ScanResult> = Vec::new();
        let rules = rules_path.into();
        let mutex = Mutex::new(());
        let mut scanner = Self {
            rules,
            code_path,
            threads,
            result,
            mutex,
            cpp,
        };
        for _ in 0..threads {
            scanner.start_worker();
        }
        scanner
    }

    fn get_task(&mut self) -> Option<utils::Rule> {
        self.rules.pop_front()
    }

    fn start_worker(&mut self) {
        scope(|_| loop {
            match self.get_task() {
                None => break,
                Some(_rule) => {
                    let tmp = self.scan(self.code_path.clone(), _rule);
                    let _guard = self.mutex.lock().unwrap();
                    self.result.push(tmp);
                }
            }
        })
        .unwrap();
    }

    fn run_query(&self, s: &(tree_sitter::Tree, String), pattern: &str,source_path:PathBuf) -> IssueCode {
        let tree = weggli::parse(pattern, self.cpp);

        let mut c = tree.walk();
        c.goto_first_child();
        let qt = weggli::builder::build_query_tree(pattern, &mut c, self.cpp, None);

        let matches = qt.matches(s.0.root_node(), &s.1);
        let mut result: IssueCode = vec![];
        for m in matches {
            
            let line = &s.1[..m.start_offset()].matches('\n').count() + 1;
            let path = format!("{}:{}",source_path.display(),line).green() ;
            result.push((path.to_string(), m.display(&s.1, 10, 10)));
        }
        result
    }

    // pub fn scan(&self, paths: Vec<PathBuf>, rules: utils::Rule) -> ScanResult {
    fn scan(&self, paths: Vec<PathBuf>, rules: utils::Rule) -> ScanResult {
        let parse = |path| {
            let source = utils::read_file(path);
            (weggli::parse(&source, self.cpp), source)
        };
        let tmp: IssueCode = vec![];
        let mut result = ScanResult {
            issue: rules.issue,
            issue_code: tmp,
            discription: String::from_str("").unwrap(),
        };
        for path in paths.iter() {
            for rule in rules.patterns.iter() {
                let mut tmp_result = self.run_query(&parse(path), rule.as_str(),path.to_path_buf());
                result.issue_code.append(&mut tmp_result);
            }
        }
        result
    }

    pub fn wait_for_result(&mut self) -> Vec<ScanResult> {
        while !self.rules.is_empty() {
            thread::sleep(Duration::from_millis(100));
        }
        self.result.clone()
    }
}
