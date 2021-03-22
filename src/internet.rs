/*
"""
Internet related generations
"""
from typing import Optional
from curious.core.constants import all_tlds
import random
import os
*/

use rand::seq::SliceRandom;
use rand::Rng;

use crate::core::constants::ALL_TLDS;

pub fn dga_domain(
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>,
    length: Option<u32>,
    tld: Option<&str>,
) -> String {
    /*
    """Generates a domain name using DGA Algorithm
    https://en.wikipedia.org/wiki/Domain_generation_algorithm
    :param year: int
    :param month: int
    :param day: int
    :param length: int
    :param tld: str
    :return: str of a DGA domain
    """
    */

    let mut trng = rand::thread_rng();

    let mut domain = "".to_owned();
    let mut selected_year: u32 = match year {
        Some(x) => x,
        None => trng.gen_range(1..9999),
    };
    let mut selected_month: u32 = match month {
        Some(x) => x,
        None => trng.gen_range(1..12),
    };
    let mut selected_day = match day {
        Some(x) => x,
        None => trng.gen_range(1..30),
    };
    let selected_tld = match tld {
        Some(x) => x,
        None => ALL_TLDS
            .choose(&mut rand::thread_rng())
            .unwrap_or(&ALL_TLDS[0]),
    };
    let selected_length: u32 = match length {
        Some(x) => x,

        None => trng.gen_range(10..25),
    };

    for _ in 0..selected_length {
        selected_year =
            ((selected_year ^ 8 * selected_year) >> 11) ^ ((selected_year & 0xFFFFFFF0) << 17);
        selected_month =
            ((selected_month ^ 4 * selected_month) >> 25) ^ 16 * (selected_month & 0xFFFFFFF8);
        selected_day =
            ((selected_day ^ (selected_day << 13)) >> 19) ^ ((selected_day & 0xFFFFFFFE) << 12);

        let xchar =
            std::char::from_u32(((selected_year ^ selected_month ^ selected_day) % 25) + 97)
                .unwrap_or('\0');
        domain.push(xchar);
    }
    domain.push('.');
    domain += selected_tld;
    domain
}
