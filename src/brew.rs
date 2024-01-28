mod json;
use crate::system::{
    self,
    Output,
    StreamsToString
};

pub struct Brew {
    pub stdout: String,
    output: Output,
}

impl Brew {
    fn new(output: Output) -> Self {
        let stdout = output.stdout_string();
        Self { stdout, output }
    }
    
    pub fn cmd(args: &[&str]) -> Self {
        Self::new(
            system::execute("brew", args).unwrap()
        )
    }
    
    pub fn cmd_with_items(cmd: &str, items: &[&str], args: &str) -> Self {
        Self::cmd(
            &[&[cmd], items, &[args]].concat()
        )
    }
    
    /// Sorted list of all outdated formulae and casks
    pub fn outdated() -> Self {
        Self::new(
            system::pipe(
                &[
                    ("bash",  &["-c", "cat <(brew outdated -v --formulae) <(brew outdated -v --casks)"]),
                    ("sort",  &[]),
                ]
            )
            .unwrap()
        )
    }
    
    /// List of all installed formulae
    pub fn list_with_desc(args: &[&str], item_type: &str) -> Self {
        let desc_cmd = format!(r#"brew desc "${{0}}" "${{@}}" --eval-all {}"#, item_type);
        
        Self::new(
            system::pipe(
                &[
                    ("brew",  args),
                    ("tr", 	  &["\n", " "]),
                    ("xargs", &["bash", "-c", &desc_cmd]),
                ]
            )
            .unwrap()
        )
    }
    
    /// JSON Parser yielding name, description, homepage
    pub fn name_desc_homepage_array(items: &[&str]) -> [Vec<String>;3] {
        json::name_desc_homepage(items)
    }
    
    // Split brew's space/colon separated output into two columns
    pub fn cols(&self) -> (Vec<&str>, Vec<&str>) {
        self.stdout
            .lines()
            .map(|l| l.split_once(&[' ', ':']).unwrap())
            .unzip()
    }
    
    pub fn array(&self) -> [Vec<&str>;2] {
        let (l, r) = self.cols();
        [l,r]
    }
    
    pub fn stderr(&self) -> String {
        self.output.stderr_string()
    }
}
