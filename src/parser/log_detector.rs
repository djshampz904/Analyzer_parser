use nom::{
    bytes::complete::{
        tag,
        take_till,
        take_until
    },
    character::complete::{ char, space0, space1 },
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
    branch::alt,
    IResult,
};


#[derive(Debug, PartialEq, Clone)]
pub enum LogFormat {
    ApacheCommon,
    ApacheCombined,
    SysLog,
    ISOTimestamp,
    BracketedTimestamp,
    JSONLog,
    Unknown,
}

#[derive(Debug)]