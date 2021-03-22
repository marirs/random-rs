//use rand;

/*
"""
Networking related generators
"""
import random

from typing import Optional, NamedTuple
from collections import namedtuple
from ipaddress import ip_network, IPv4Network, IPv4Address, IPv6Network, IPv6Address
from curious.core.utils import hex_pattern
from curious.core.constants import dummy_corps_locations
from curious.core.constants import tlds
*/

//use std::collections::HashMap;
use crate::core::constants::{DUMMY_CORPS_LOCATIONS, TLDS};
use ipnetwork::IpNetwork;
use rand::prelude::*;

fn __exclude_ip_networks(
    networks: Vec<IpNetwork>,
    networks_to_exclude: Vec<IpNetwork>,
) -> Vec<IpNetwork> {
    /*
    Private function - DO NOT USE OUTSIDE OF THIS FILE:
    Exclude the list of networks from another list of networks
    and return a flat list of new networks.
    :param networks: List of IPv4 networks to exclude from
    :param networks_to_exclude: List of IPv4 networks to exclude
    :returns: Flat list of IPv4 networks
    */
    let mut final_networks = Vec::<IpNetwork>::with_capacity(networks.capacity());

    for network in &networks {
        let mut add_to_final = true;
        for exc_network in &networks_to_exclude {
            if network == exc_network {
                add_to_final = false;
                break;
            }
        }
        if add_to_final {
            final_networks.push(*network);
        }
    }

    final_networks
}

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
fn _ip_from_subnet(subnet_str: String, version: u8) -> Option<String> {
    /*
    Private function to generate IP Address from a given subnet
    :param subnet_str: Subnet String
    :return: IP Address (v4 or v6)
    */

    if subnet_str.is_empty() {
        return None;
    }

    if version == 4 {
        let subnet: IpNetwork = IpNetwork::V4(subnet_str.parse().unwrap());
        let prefix_max = subnet.broadcast();
        let prefix_min = subnet.network();

        let prefix_max_u32: u32;
        let prefix_min_u32: u32;

        if let IpAddr::V4(x) = prefix_max {
            prefix_max_u32 = u32::from(x);
        } else {
            return None;
        };

        if let IpAddr::V4(x) = prefix_min {
            prefix_min_u32 = u32::from(x);
        } else {
            return None;
        };

        let mut trng = rand::thread_rng();
        let bits: u32 = trng.gen_range(0..(prefix_max_u32 - prefix_min_u32));

        let calculated_address = prefix_min_u32 + bits;

        let addr = Ipv4Addr::from(calculated_address);
        return Some(addr.to_string());
    } else if version == 6 {
        let subnet: IpNetwork = IpNetwork::V6(subnet_str.parse().unwrap());

        let prefix_max = subnet.broadcast();
        let prefix_min = subnet.network();

        let prefix_max_u128: u128;
        let prefix_min_u128: u128;

        if let IpAddr::V6(x) = prefix_max {
            prefix_max_u128 = u128::from(x);
        } else {
            return None;
        };

        if let IpAddr::V6(x) = prefix_min {
            prefix_min_u128 = u128::from(x);
        } else {
            return None;
        };

        let mut trng = rand::thread_rng();
        let bits: u128 = trng.gen_range(0..(prefix_max_u128 - prefix_min_u128));

        let calculated_address = prefix_min_u128 + bits;

        let addr = Ipv6Addr::from(calculated_address);
        return Some(addr.to_string());
    }

    None
}

fn __ipaddr(
    version: Option<u32>,
    address_class: Option<char>,
    is_private_ip: Option<bool>,
    from_subnet: Option<String>,
) -> String {
    let version = version.unwrap_or(4);
    let mut address_class = address_class.unwrap_or('r');
    let is_private_ip = is_private_ip.unwrap_or(false);
    //let from_subnet = from_subnet.unwrap_or("".to_string());
    /*
    Generate random IP address
    :param version: 4 for IPv4 or 6 for IPv6
    :param address_class: a, b, c as address class networks - Any other alphabet, randomise the class
    :param is_private_ip: True to generate private IP range, else generate Public IP
    :param from_subnet: Get Random IP addresses from the given subnet -
                        Sample IPv4 Subnet: 10.0.0.0/8
                        Sample IPv6 Subnet: 2402:9400:1000:11::/64
    :return: random IPv4 or IPv6 address as string
    */

    // define the return value
    let mut ip: Option<String> = None;

    if from_subnet != None {
        let from_subnet = from_subnet.unwrap_or_else(|| "".to_string());
        ip = _ip_from_subnet(from_subnet, 4);
        if ip != None {
            if let Some(x) = ip {
                return x;
            }
        }
    }

    let mut network_classes;
    let mut private_networks;
    let mut excluded_networks;

    let mut trng = rand::thread_rng();

    //# If not from_subnet and/or
    //# subnet was provided wrong (fail-over)
    if version == 4 {
        // IPv4
        network_classes = std::collections::HashMap::<char, IpNetwork>::new();
        network_classes.insert('a', "0.0.0.0/1".parse().unwrap());
        network_classes.insert('b', "128.0.0.0/2".parse().unwrap());
        network_classes.insert('c', "192.0.0.0/3".parse().unwrap());

        //# Three common private networks from class A, B and CIDR
        //# to generate private addresses from.
        private_networks = Vec::<IpNetwork>::new();
        private_networks.push("10.0.0.0/8".parse().unwrap());
        private_networks.push("172.16.0.0/12".parse().unwrap());
        private_networks.push("192.168.0.0/16".parse().unwrap());

        // List of networks from which IP addresses will never be generated,
        // includes other private IANA and reserved networks from
        // https://www.iana.org/assignments/iana-ipv4-special-registry/iana-ipv4-special-registry.xhtml
        excluded_networks = Vec::<IpNetwork>::new();
        excluded_networks.push("0.0.0.0/8".parse().unwrap());
        excluded_networks.push("100.64.0.0/10".parse().unwrap());
        excluded_networks.push("127.0.0.0/8".parse().unwrap()); // loopback network
        excluded_networks.push("169.254.0.0/16".parse().unwrap()); // link-local network
        excluded_networks.push("192.0.0.0/24".parse().unwrap());
        excluded_networks.push("192.0.2.0/24".parse().unwrap());
        excluded_networks.push("192.31.196.0/24".parse().unwrap());
        excluded_networks.push("192.52.193.0/24".parse().unwrap());
        excluded_networks.push("192.88.99.0/24".parse().unwrap()); // 6to4 anycast relay
        excluded_networks.push("192.175.48.0/24".parse().unwrap());
        excluded_networks.push("198.18.0.0/15".parse().unwrap());
        excluded_networks.push("198.51.100.0/24".parse().unwrap());
        excluded_networks.push("203.0.113.0/24".parse().unwrap());
        excluded_networks.push("224.0.0.0/4".parse().unwrap()); // multicast network
        excluded_networks.push("240.0.0.0/4".parse().unwrap());
        excluded_networks.push("255.255.255.255/32".parse().unwrap());

        //if address_class not in network_classes.keys(){
        if true {
            address_class = *(['a', 'b', 'c'].choose(&mut trng).unwrap_or(&'c'));
        }

        let supernet = network_classes[&address_class];

        let public_networks = network_classes[&address_class];
        let mut sclasses = private_networks.to_vec();
        sclasses.append(&mut excluded_networks.to_vec());

        let pnetworks = vec![public_networks];

        let public_networks = __exclude_ip_networks(pnetworks, sclasses);

        //private_networks = [subnet for subnet in private_networks if subnet.overlaps(supernet)];
        private_networks = __exclude_ip_networks(private_networks, excluded_networks);

        //let all_networks = [network_classes[&address_class]];
        //let anetworks = __exclude_ip_networks(all_networks.to_vec(), excluded_networks);

        if is_private_ip {
            //# this is an internal private IP
            //ip = _ip_from_subnet(str(random.choice(private_networks)));
            ip = _ip_from_subnet(private_networks.choose(&mut trng).unwrap().to_string(), 4);
        } else if !is_private_ip {
            //# this will be a public IP
            ip = _ip_from_subnet(public_networks.choose(&mut trng).unwrap().to_string(), 4);
        }
    } else if version == 6 {
        //# IPv6
        private_networks = Vec::<IpNetwork>::new();
        private_networks.push("fc00::/7".parse().unwrap()); //# Unique Local Addresses (ULAs - RFC 4193)
        private_networks.push("fc00::/8".parse().unwrap()); //# Unique Local Addresses (ULAs - RFC 4193)
        private_networks.push("fd00::/8".parse().unwrap()); //# Unique Local Addresses (ULAs - RFC 4193)

        let mut public_networks = Vec::<IpNetwork>::new();
        public_networks.push("2001::/16".parse().unwrap());
        public_networks.push("2001::/32".parse().unwrap());
        public_networks.push("2001::/48".parse().unwrap());
        public_networks.push("2001::/56".parse().unwrap());
        public_networks.push("2001::/64".parse().unwrap());

        let mut excluded_networks = Vec::<IpNetwork>::new();
        excluded_networks.push("::/128".parse().unwrap()); //# unspecified address
        excluded_networks.push("::1/128".parse().unwrap()); //# loopback address
        excluded_networks.push("fe80::/10".parse().unwrap()); //# link-local network
        excluded_networks.push("ff00::/8".parse().unwrap()); //# multicast address (RFC 4038)
        excluded_networks.push("2001:db8::/32".parse().unwrap()); //# reserved for use in documentation
        excluded_networks.push("2002::/16".parse().unwrap()); //# 6to4 public router anycast (RFC 3068)
        excluded_networks.push("2000::/3".parse().unwrap()); //# global unicast

        private_networks = __exclude_ip_networks(private_networks, excluded_networks);

        if is_private_ip {
            //# this is an internal private IP
            ip = _ip_from_subnet(private_networks.choose(&mut trng).unwrap().to_string(), 6);
        } else if !is_private_ip {
            //# this will be a public IP
            ip = _ip_from_subnet(public_networks.choose(&mut trng).unwrap().to_string(), 6);
        }
    }

    ip.unwrap()
}

pub fn public_ip(
    version: Option<u32>,
    address_class: Option<char>,
    from_subnet: Option<String>,
) -> String {
    let version = version.unwrap_or(4);
    let address_class = address_class.unwrap_or('r');
    //let from_subnet = from_subnet.unwrap_or(None);

    // Returns a Public IP Address
    __ipaddr(Some(version), Some(address_class), None, from_subnet)
}

pub fn private_ip(
    version: Option<u32>,
    address_class: Option<char>,
    from_subnet: Option<String>,
) -> String {
    //Returns a Private IP Address
    //let version = version.unwrap_or(4);
    //let address = address_class.unwrap_or('r');
    //let from_subnet = from_subnet.unwrap_or(None);

    __ipaddr(version, address_class, Some(true), from_subnet)
}

pub fn port_number(
    is_system: Option<bool>,
    is_user: Option<bool>,
    is_dynamic: Option<bool>,
) -> u32 {
    /*
    Generates a random network port number
    https://tools.ietf.org/html/rfc6335
    :param is_system: System or well-known ports
    :param is_user: User or registered ports
    :param is_dynamic: Dynamic / private / ephemeral ports
    :rtype: int
    */

    let is_system = is_system.unwrap_or(false);
    let is_user = is_user.unwrap_or(false);
    let is_dynamic = is_dynamic.unwrap_or(false);

    let mut trng = rand::thread_rng();

    if is_system {
        return trng.gen_range(0..1023);
    } else if is_user {
        return trng.gen_range(1024..49151);
    } else if is_dynamic {
        return trng.gen_range(49152..65535);
    }

    trng.gen_range(0..65535)
}

pub fn mac_address(upper_case: Option<bool>, oui: Option<String>) -> String {
    /*
    Generates a random MAC Address
    */
    //let upper_case = upper_case.unwrap_or(false);
    let oui = oui.unwrap_or_else(|| "".to_string());

    if oui.is_empty() {
        //return hex_pattern(pattern='^^:^^:^^:^^:^^:^^', upper=upper_case)
        return crate::core::utils::hex_pattern(Some("^^:^^:^^:^^:^^:^^"), upper_case);
    }

    crate::core::utils::hex_pattern(Some("^^:^^:^^:^^:^^:^^"), upper_case)
    //let mac_adr = crate::core::utils::hex_pattern(Some("^^:^^:^^:^^:^^:^^"), upper_case);
    //mac_adr[:len(str(oui))] = list(str(oui))
    //return "".join(mac_adr)
    //return "".to_string();
    // /mac_adr
}

pub struct FQDN {
    pub domain: String,
    pub sub_domain: String,
    pub tld: String,
    pub fqdn: String,
}

pub fn fqdn(company_name: &str) -> Option<FQDN> {
    /*
    Create a FQDN with the given company name
    :param company_name: company name as string
    :return: namedtuple of fqdn, domain, subdomain, tld
    */
    if company_name.is_empty() {
        return None;
    }

    let mut trng = rand::thread_rng();
    //let FQDN = namedtuple('fqdn', ['domain', 'sub_domain', 'tld', 'fqdn'])
    let _fqdn = format!(
        "{}.{}.{}",
        DUMMY_CORPS_LOCATIONS
            .choose(&mut trng)
            .unwrap_or(&DUMMY_CORPS_LOCATIONS[0]),
        company_name,
        TLDS.choose(&mut trng).unwrap_or(&TLDS[0])
    );
    //f'{random.choice(dummy_corps_locations)}.' \
    //        f'{company_name}.' \
    //        f'{random.choice(tlds)}'

    let splitfqdn = _fqdn.split('.').collect::<Vec<&str>>();

    let tld = splitfqdn[2];
    let domain = splitfqdn[1];
    let subdomain = splitfqdn[0];

    Some(FQDN {
        domain: domain.to_string(),
        sub_domain: subdomain.to_string(),
        tld: tld.to_string(),
        fqdn: _fqdn.to_string(),
    })
}
