use std::path::PathBuf;

use quac_core::{
    errors::error::QError, 
    linefeeder::SourceMap
};

pub mod diagnoser;
pub mod renderer;
pub mod macros;

pub fn diagnose(errors: Vec<QError>, source: Vec<u8>, sourcemap: SourceMap, path: PathBuf){
    diagnoser::diagnose(errors, source, sourcemap, path);
}