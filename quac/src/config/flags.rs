use getopts::Options;

use crate::config::{
    conf::Config, 
    helpers
};

pub struct QuacArgs{
    pub(crate) args: Vec<String>,
    pub(crate) opts: Options
}

impl QuacArgs{
    pub fn new(args: Vec<String>) -> Self{
        Self { args, opts: Options::new() }
    }

    pub fn init(mut self) -> Self{
        self.opts.optflag("v", "version", "shows the current version of your build of q4r/quac");
        self.opts.optflag("h", "help", "shows the usage of quac");
        self.opts.optflag("d", "debug", "whether debug mode is active or not, debug mode will add rust's backtrace on top of q4r's default error messages");
        self.opts.optopt("", "backend", "change the backend of the compiler", "<qbe/c>");
        self
    }

    pub fn extract_opts(&self, mut conf: Config) -> Config{
        let matches = match self.opts.parse(&self.args[1..]){
            Ok(m) => m,
            Err(why) => panic!("error when parsing flags: {}", why)
        };

        helpers::treat_optflag(self, &mut conf, matches);
        conf
    }
}