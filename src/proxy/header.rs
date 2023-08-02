// Bloom
//
// HTTP REST API caching middleware
// Copyright: 2017, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use hyper::header::{ETag, Header, Vary};
use hyper::Headers;
use std::str::from_utf8;
use unicase::Ascii;

use super::defaults;
use crate::{header::request_shard::HeaderRequestBloomRequestShard, APP_CONF};

pub struct ProxyHeader;

impl ProxyHeader {
    pub fn parse_from_request(headers: Headers) -> (Headers, String, u8) {
        // Request header: 'Authorization'
        //debug!("Request headers {}", headers);
        fn get_auth_header(headers: &Headers) -> String {
            let mut auth_header = defaults::REQUEST_AUTHORIZATION_DEFAULT.to_string();
            for x in headers.iter() {
                if x.name().to_lowercase() == &*APP_CONF.cache.auth_header.to_lowercase() {
                    debug!("Authorization header found");
                    auth_header = from_utf8(x.raw().one().unwrap_or(&[]))
                        .unwrap_or(defaults::REQUEST_AUTHORIZATION_DEFAULT)
                        .to_string();
                } else {
                    debug!("Authorization header not found {} is not {} ", x.name().to_lowercase(),&*APP_CONF.cache.auth_header.to_lowercase());
                    auth_header = defaults::REQUEST_AUTHORIZATION_DEFAULT.to_string();
                }
            }
            return auth_header;
        }

        let auth = get_auth_header(&headers);

        // Request header: 'Bloom-Request-Shard'
        let shard = match headers.get::<HeaderRequestBloomRequestShard>() {
            None => APP_CONF.proxy.shard_default,
            Some(value) => value.0,
        };

        (headers, auth, shard)
    }

    pub fn set_etag(headers: &mut Headers, etag: ETag) {
        headers.set::<Vary>(Vary::Items(vec![Ascii::new(
            ETag::header_name().to_string(),
        )]));

        headers.set::<ETag>(etag);
    }
}
