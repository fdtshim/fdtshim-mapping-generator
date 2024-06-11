/* SPDX-License-Identifier: GPL-3.0-only */

#[warn(clippy::pedantic)]
mod dtb_data;
use crate::dtb_data::*;
use glob::glob;
use itertools::Itertools;
use log::error;
use log::info;
use std::env;
use std::path::Path;
use std::process::exit;

fn dump_dts(prefix: &Path) {
    println!("/dts-v1/;");
    println!();
    println!("/ {{");
    println!("\tfdtshim,schema-version = {:?};", "0.1");
    println!("\tfdtshim,generator = {:?};", "fdtshim-mapping-generator");
    println!("\tcompatible = {:?};", "fdtshim,mapping");
    println!("}};");
    println!();

    let mut data: Vec<DtbData> = Vec::new();
    info!("--- Reading dtb data...");
    for entry in glob(prefix.join("**/*.dtb").to_str().unwrap()).unwrap() {
        match entry {
            Ok(path) => {
                let dtb = DtbData::new(path.clone(), prefix);
                info!("    Reading {:?}", path.strip_prefix(prefix).unwrap());
                data.push(dtb);
            }
            Err(e) => {
                error!("{:?}", e);
                exit(3);
            }
        }
    }
    info!("--- Sorting and sanitizing...");

    // Data needs to be sorted by desired grouping for chunk_by.
    // We group on the main compatible, so sort by main compatible.
    data.sort_by(|a, b| a.compatible().partial_cmp(b.compatible()).unwrap());

    // Data for which only one dtb has the declared compatible
    let mut valid: Vec<&DtbData> = Vec::new();
    // Data for which many dtbs have the declared compatible
    let mut invalid: Vec<Vec<&DtbData>> = Vec::new();

    info!("    Figuring out duplicated compatibles...");
    for (_key, chunk) in &data.iter().chunk_by(|el| el.compatible().clone()) {
        let group: Vec<&DtbData> = chunk.collect();
        if group.len() == 1 {
            let only = group.first().unwrap();
            valid.push(only);
        } else {
            invalid.push(group);
        }
    }

    info!("    Writing out warnings...");
    if invalid.is_empty() {
        println!("/* No warnings during generation */");
    } else {
        println!("/*");
        println!(" * WARNING: These dtb files share the main compatible names.");
        println!(" *          No action has been taken for them.");

        for dtbs in invalid {
            println!(" *");
            let compatible = dtbs.first().unwrap().compatible();
            println!(" * - {compatible}");
            for dtb in dtbs {
                println!(" *     - {}", dtb.path);
            }
        }
        println!(" */");
    }
    println!();

    info!("    Sorting output...");
    // Sort back by file path.
    valid.sort();

    info!("    Writing data...");
    println!("/ {{");
    println!("\tmapping {{");
    for dtb in valid {
        println!("\t\t/* {}: {:?} */", dtb.model, dtb.compatibles);
        println!("\t\t{} {{", dtb.node_name());
        println!("\t\t\tdtb = {:?};", dtb.path);
        println!("\t\t\tmodel = {:?};", dtb.model);
        println!("\t\t\tcompatible = {};", dtb.compatibles_source());
        println!("\t\t}};");
    }
    println!("\t}};");
    println!("}};");
}

fn print_usage() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.first().unwrap();
    println!("Usage: {cmd} <path to dtbs output>");
}

fn main() {
    stderrlog::new()
        .quiet(false)
        .verbosity(log::Level::Info)
        .module(module_path!())
        .init()
        .unwrap();

    // TODO: move to a more complete args parsing crate once this gets more involved.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        if args.len() < 2 {
            error!("No argument provided.");
        } else {
            error!("Too many arguments provided.");
        }
        print_usage();
        exit(1);
    }
    let arg = args.last().unwrap();

    if arg == "-h" || arg == "--help" || arg == "/?" {
        print_usage();
        exit(0);
    }

    let path = Path::new(arg);

    if !path.is_dir() {
        error!("The given path is not valid or not a directory.");
        exit(2);
    }

    dump_dts(path);
}
