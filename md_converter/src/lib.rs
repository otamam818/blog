//! This crate parses markdown into data structures which 
//! then gets parsed into html using handlebars
//!
//! This is in an attempt to load different html sections acynchronously,
//! increasing the page-load speeds for the main website.
//!
//! This crate also aims to give markdown files support for custom classes
//! and attributes

mod parser;
mod converter;
mod data_models;

pub use crate::converter::convert_text;

