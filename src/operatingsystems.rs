/*
"""
operating system related generators
"""
from typing import Optional
import random
import uuid
from string import ascii_uppercase, digits
from itertools import groupby
from curious.core.utils import hex_pattern
from curious.core.constants import hostname_suffix, win_hostnames, nix_hostnames, brands
from curious.core.constants import server_application_codes, server_prefix, server_suffix, server_type

from curious.core.decorators import uppercase, lowercase
*/
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::constants::{
    //BRANDS, HOSTNAME_SUFFIX, NIX_HOSTNAMES, SERVER_APPLICATION_CODES, SERVER_PREFIX, SERVER_SUFFIX,
    //SERVER_TYPE, WIN_HOSTNAMES,
    BRANDS,
    NIX_HOSTNAMES,
    SERVER_APPLICATION_CODES,
    SERVER_PREFIX,
    SERVER_SUFFIX,
    WIN_HOSTNAMES,
};

pub fn logon_id(batch_size: Option<u32>) -> Vec<String> {
    /*
    Generates a hex logon id for windows os
    :param batch_size: how many to generate
    :return: string of hex logon id if batch size is 1 else a list of login id's
    */

    let batch_size = batch_size.unwrap_or(1);

    let mut logon_ids = Vec::<String>::new();
    for _ in 1..batch_size {
        logon_ids.push(crate::core::utils::hex_pattern(Some("0x^^^^^^^"), None));
    }

    //return ''.join(logon_ids) if batch_size == 1 else logon_ids
    logon_ids
}

pub fn new_uuid(batch_size: Option<u32>) -> Vec<String> {
    /*
    Generate a UUID/GUID
    :param batch_size: how many to generate
    :return: str of generated UUID or list of uuid's ig batch size is more than 1
    */

    let batch_size = batch_size.unwrap_or(1);

    let mut uuids = Vec::<String>::new();

    for _ in 1..batch_size {
        uuids.push(Uuid::new_v4().to_string());
    }

    //return ''.join(uuids) if batch_size == 1 else uuids
    uuids
}

//@uppercase
pub fn localhost_name(
    windows: Option<bool>,
    linux: Option<bool>,
    name: Option<String>,
    suffix_local: Option<bool>,
    batch_size: Option<u32>,
) -> Vec<String> {
    /*
    Generate a random localhost Name: Same as hostname, but this will not take a prefix, instead just
    generate a random name and return
    :param windows: Generates windows machine name
    :param linux: generates linux machine names
    :param name: if given this name will be used as suffix instead of a random generated id
    :param suffix_local: eg: hostname.local
    :param batch_size: how many to generate
    :return: returns a hostname as str if batch size is 1, else returns a list
    */

    let windows = windows.unwrap_or(false);
    let linux = linux.unwrap_or(false);
    let name: String = match name {
        Some(x) => x,
        None => "".to_owned(),
    };

    let suffix_local = suffix_local.unwrap_or(false);
    let batch_size = batch_size.unwrap_or(1);

    let mut hostname = Vec::<String>::new();

    let mut trng = rand::thread_rng();

    for _ in 1..batch_size {
        let n = trng.gen_range(8..12);

        let prefix: String;

        let suffix = if !name.is_empty() {
            name.clone()
        } else {
            let s: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(n)
                .map(char::from)
                .collect();
            s.to_ascii_uppercase()
        };

        if windows {
            let mut winhbrands = Vec::new();
            for i in WIN_HOSTNAMES.iter() {
                winhbrands.push(i);
            }
            for i in BRANDS.iter() {
                winhbrands.push(i);
            }

            let x = winhbrands.choose(&mut rand::thread_rng());
            prefix = x.unwrap().to_string();
        } else if linux {
            let mut nixnames = Vec::new();
            for i in NIX_HOSTNAMES.iter() {
                nixnames.push(i);
            }

            let x = nixnames.choose(&mut rand::thread_rng());

            prefix = x.unwrap().to_string();
        } else {
            let mut mixnames = Vec::new();
            for i in NIX_HOSTNAMES.iter() {
                mixnames.push(i);
            }
            for i in WIN_HOSTNAMES.iter() {
                mixnames.push(i);
            }

            let x = mixnames.choose(&mut rand::thread_rng());

            prefix = x.unwrap().to_string();
        }

        let hname = format!(
            "{}-{}",
            prefix,
            if suffix_local { suffix } else { "".to_string() }
        );

        hostname.push(hname);
        //hostname.push(r"{prefix}-{suffix}{'.local' if suffix_local else ''}")
    }

    hostname

    //return ''.join(hostname) if batch_size == 1 else hostname
}

//@lowercase
pub fn hostname(prefix_str: &str, suffix_str: Option<&str>, mac_suffix: Option<bool>) -> String {
    /*
    Generate a hostname based on a given prefix str
    :param prefix_str: A string to prefix for a qualified hostname
                    Eg: username or fname, etc...
    :param suffix_str: A string to suffix
                    Eg: -PC, -DESKTOP, -HOME, etc...
    :param mac_suffix: If mac_suffix be used or not
    :return: returns a valid hostname
    */

    let mut suffix_str = suffix_str.unwrap_or("");
    let mac_suffix = mac_suffix.unwrap_or(true);

    let mut trng = rand::thread_rng();
    let x = trng.gen_range(1..10);

    let hostnum: String = format!("{:02}", x);

    println!("{}", hostnum);

    let digits = "0123456789";

    //let mut prefix_str_name = prefix_str.translate(str.maketrans('', '', digits));
    let mut hashmap: HashMap<char, Option<char>> = HashMap::new();
    // for c in prefix_str.chars() {
    //     hashmap.insert(c, c);
    // }
    for c in digits.chars() {
        hashmap.insert(c, None);
    }

    let mut prefix_str_name = String::with_capacity(prefix_str.len());
    for c in prefix_str.chars() {
        match hashmap.get(&c) {
            Some(None) => {
                // do nothing in case of digits mapped to None
            }
            _ => prefix_str_name.push(c),
        }
    }

    //prefix_str = [''.join(g) for k, g in groupby(prefix_str, ' ._'.__contains__) if not k]

    //prefix_str = f'{prefix_str[0]}{hostnum if bool(random.getrandbits(1)) else ""}'
    let prefix_str = format!(
        "{}{}",
        prefix_str_name,
        if rand::random::<bool>() {
            hostnum
        } else {
            "".to_string()
        }
    );

    const HOSTNAME_SUFFIX: &[&str] = &["-PC", "-MAC", "-OSX", "-LINUX", "-CHROMEBOOK"];

    let mut host_suffix = HOSTNAME_SUFFIX.to_vec();
    if !mac_suffix {
        host_suffix.retain(|x| *x != "-OSX");
        host_suffix.retain(|x| *x != "-MAC");
    }
    suffix_str = if suffix_str.is_empty() {
        host_suffix.choose(&mut trng).unwrap_or(&"")
    } else {
        suffix_str
    };

    //dbg!(host_suffix);
    //dbg!(suffix_str);

    let host = format!("{}{}", prefix_str, suffix_str);
    //dbg!(host);
    host
}

//@lowercase
pub enum OsType {
    Windows,
    Linux,
    Others,
}

pub fn servername(windows: Option<bool>, linux: Option<bool>) -> String {
    /*
    Generate a random server name
    :param windows: Windows Server Names
    :param linux: Linux Server Names
    :return: returns a generated server name
    */

    let windows = windows.unwrap_or(false);
    let linux = linux.unwrap_or(false);

    let mut trng = rand::thread_rng();

    if windows {
        // srvname = f'W{random.choice([2012, 2014, 2016, 2019])}R2' \
        //           f'{str(random.randint(1, 10)).rjust(3, "0")}-SRV'
        format!(
            "W{}R2{:03}-SRV",
            [2012, 2014, 2016, 2019].choose(&mut trng).unwrap_or(&2019),
            trng.gen_range(1..10)
        )
    } else if linux {
        // srvname = f'{random.choice(nix_hostnames)}{str(random.randint(1, 10)).rjust(3, "0")}' \
        //           f'{random.choice(server_application_codes)}-SRV'
        format!(
            "{}{:03}{}-SRV",
            NIX_HOSTNAMES.choose(&mut trng).unwrap_or(&NIX_HOSTNAMES[0]),
            trng.gen_range(1..10),
            SERVER_APPLICATION_CODES
                .choose(&mut trng)
                .unwrap_or(&SERVER_APPLICATION_CODES[0])
        )
    } else {
        // {srvnum = str(random.randint(1, 5)).rjust(2, '0')
        //         srvname = f'{random.choice(server_prefix)}{random.choice(server_application_codes)}' \
        //                   f'{random.choice(server_type)}' \
        //                   f'{srvnum if bool(random.getrandbits(1)) else ""}{random.choice(server_suffix)}'}

        let srvnum = format!("{:02}", trng.gen_range(1..5));
        format!(
            "{}{}{}",
            SERVER_PREFIX.choose(&mut trng).unwrap_or(&SERVER_PREFIX[0]),
            SERVER_APPLICATION_CODES
                .choose(&mut trng)
                .unwrap_or(&SERVER_APPLICATION_CODES[0]),
            if rand::random::<bool>() {
                srvnum
            } else {
                (SERVER_SUFFIX.choose(&mut trng).unwrap_or(&SERVER_SUFFIX[0])).to_string()
            }
        )
    }
}
