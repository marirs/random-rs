/*
"""
Utils file to common functions
"""
import re
import os
import string
import random

from typing import Optional
from itertools import chain


__pattern_re = re.compile(r'\^')

flatten = chain.from_iterable
get_path = lambda filename: os.path.abspath(os.path.join(os.path.dirname(__file__), filename))
*/

use rand::prelude::*;

//static __pattern_re = Regex::new(r"\^");

pub fn hex_pattern(pattern: Option<&str>, upper: Option<bool>) -> String {
    /*
    Generate a string with each circumflex ('^') in ``fmt``
        replaced with a random hexadecimal character
    :param pattern: Format of the return hexified string
    :param upper: if the hexified string to be in upper
    :return: formated hex string
            eg: hex_this('0x^^^') will return 0x3E5
                hex_this('^^:^^:^^:^^:^^:^^:') will return 3e:aa:1a:2f:9f:11
    */

    let pattern = pattern.unwrap_or("^^^^");
    let upper = upper.unwrap_or(false);
    let letters = "0123456789abcdef";
    let processed_letters = if upper {
        letters.to_ascii_uppercase()
    } else {
        letters.to_string()
    };

    let rg = regex::Regex::new(r"\^").unwrap();
    let result = rg.replace_all(&pattern, |_caps: &regex::Captures| {
        processed_letters
            .chars()
            .choose(&mut rand::thread_rng())
            .unwrap_or('\0')
            .to_string()
    });
    result.to_string()
}

/*
def lazy_read(file_obj, buff_size=1024):
    """Generator functions to Read the contents of the
    file in a lazy way
    :param file_obj: file object
    :param buff_size: default 1k
    :return: yield contents
    """
    while True:
        data = file_obj.read(buff_size)
        if not data:
            break
        yield data
*/
