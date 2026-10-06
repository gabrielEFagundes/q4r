use std::{
    path::PathBuf, 
    process::exit
};
use getopts::Matches;

use crate::{
    config::conf::{
        Backend, 
        Config
    }, 
    config::flags::QuacArgs
};

pub fn treat_optflag<'a>(quargs: &'a QuacArgs, conf: &'a mut Config, m: Matches){
    if m.opt_present("h"){ 
        treat_hflag(quargs);
    }

    if m.opt_present("v"){
        treat_vflag(conf);
    }

    conf.debug_mode = m.opt_present("d");
    
    conf.path = if !m.free.is_empty() && m.free.len() < 2{
        PathBuf::from(m.free[0].clone())
    } else {
        PathBuf::new()
    };

    if let Some(b) = &m.opt_str("backend") { conf.backend = Backend::map(b) }
}

#[inline]
fn treat_hflag(quargs: &QuacArgs){
    print!("{}", quargs.opts.usage("usage: quac <file.qo> [FLAGS]"));
    exit(0)
}

#[inline]
fn treat_vflag(conf: &mut Config){
    println!("{}", conf.version);
    exit(0)
}

#[inline]
pub fn validate_path(conf: &Config){
    if conf.path.is_empty(){ panic!("please, provide a valid path to your main file") }
}