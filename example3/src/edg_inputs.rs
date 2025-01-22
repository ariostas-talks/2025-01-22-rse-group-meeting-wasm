use polars_core::prelude::*;
use polars_io::prelude::*;
use std::io::Cursor;

// TODO: This should be done better

pub const FUELS_2030: &str = include_str!("../edg_inputs/2030/Fuels_data.csv");
// pub const FUELS_2035: &str = include_str!("../edg_inputs/2035/Fuels_data.csv");
// pub const FUELS_2040: &str = include_str!("../edg_inputs/2040/Fuels_data.csv");
// pub const FUELS_2045: &str = include_str!("../edg_inputs/2045/Fuels_data.csv");
// pub const FUELS_2050: &str = include_str!("../edg_inputs/2050/Fuels_data.csv");

// pub const LOAD_2030: &str = include_str!("../edg_inputs/2030/Load_data.csv");
// pub const LOAD_2035: &str = include_str!("../edg_inputs/2035/Load_data.csv");
// pub const LOAD_2040: &str = include_str!("../edg_inputs/2040/Load_data.csv");
// pub const LOAD_2045: &str = include_str!("../edg_inputs/2045/Load_data.csv");
// pub const LOAD_2050: &str = include_str!("../edg_inputs/2050/Load_data.csv");

// pub const RESOURCES_2030: &str = include_str!("../edg_inputs/2030/Resources_data.csv");
// pub const RESOURCES_2035: &str = include_str!("../edg_inputs/2035/Resources_data.csv");
// pub const RESOURCES_2040: &str = include_str!("../edg_inputs/2040/Resources_data.csv");
// pub const RESOURCES_2045: &str = include_str!("../edg_inputs/2045/Resources_data.csv");
// pub const RESOURCES_2050: &str = include_str!("../edg_inputs/2050/Resources_data.csv");

// pub const RESOURCES_VARIABILITY_2030: &str = include_str!("../edg_inputs/2030/Resources_variability.csv");
// pub const RESOURCES_VARIABILITY_2035: &str = include_str!("../edg_inputs/2035/Resources_variability.csv");
// pub const RESOURCES_VARIABILITY_2040: &str = include_str!("../edg_inputs/2040/Resources_variability.csv");
// pub const RESOURCES_VARIABILITY_2045: &str = include_str!("../edg_inputs/2045/Resources_variability.csv");
// pub const RESOURCES_VARIABILITY_2050: &str = include_str!("../edg_inputs/2050/Resources_variability.csv");

fn type_to_index(type_: &str) -> usize {
    match type_ {
        "Fuels" => 0,
        "Load" => 1,
        "Resources" => 2,
        "Resources Variability" => 3,
        _ => panic!("Invalid type"),
    }
}

fn year_to_index(year: u32) -> usize {
    match year {
        2030 => 0,
        2035 => 1,
        2040 => 2,
        2045 => 3,
        2050 => 4,
        _ => panic!("Invalid year"),
    }
}

pub fn get_data(type_: &str, year: u32) -> &str {

    let data_2030: Vec<&str> = vec![FUELS_2030]; //, LOAD_2030, RESOURCES_2030, RESOURCES_VARIABILITY_2030];
    // let data_2035: Vec<&str> = vec![FUELS_2035, LOAD_2035, RESOURCES_2035, RESOURCES_VARIABILITY_2035];
    // let data_2040: Vec<&str> = vec![FUELS_2040, LOAD_2040, RESOURCES_2040, RESOURCES_VARIABILITY_2040];
    // let data_2045: Vec<&str> = vec![FUELS_2045, LOAD_2045, RESOURCES_2045, RESOURCES_VARIABILITY_2045];
    // let data_2050: Vec<&str> = vec![FUELS_2050, LOAD_2050, RESOURCES_2050, RESOURCES_VARIABILITY_2050];

    let all_data: Vec<Vec<&str>>  = vec![data_2030]; //, data_2035, data_2040, data_2045, data_2050];

    return all_data[year_to_index(year)][type_to_index(type_)];
}

pub fn load_fuels_input(year: u32) -> DataFrame {
    let data = get_data("Fuels", year);
    let cursor = Cursor::new(data);
    let reader = CsvReader::new(cursor).finish().unwrap();
    reader
}