/*!
# Cache Response for Rocket Framework

This crate provides a response struct used for HTTP cache control.

See `examples`.
*/

extern crate rocket;

use rocket::request::Request;
use rocket::response::{Responder, Response, Result};

/// The responder with a `Cache-Control` header.
#[derive(Debug)]
pub enum CacheResponse<R: Responder<'static>> {
    Public {
        responder: R,
        max_age: u32,
        must_revalidate: bool,
    },
    Private {
        responder: R,
        max_age: u32,
    },
    NoCache(R),
    NoStore(R),
    NoCacheControl(R),
}

impl<R: Responder<'static>> Responder<'static> for CacheResponse<R> {
    fn respond_to(self, req: &Request) -> Result<'static> {
        match self {
            CacheResponse::Public {
                responder,
                max_age,
                must_revalidate,
            } => {
                Response::build_from(responder.respond_to(req)?)
                    .raw_header(
                        "Cache-Control",
                        if must_revalidate {
                            format!("must-revalidate, public, max-age={}", max_age)
                        } else {
                            format!("public, max-age={}", max_age)
                        },
                    )
                    .ok()
            }
            CacheResponse::Private {
                responder,
                max_age,
            } => {
                Response::build_from(responder.respond_to(req)?)
                    .raw_header("Cache-Control", format!("private, max-age={}", max_age))
                    .ok()
            }
            CacheResponse::NoCache(responder) => {
                Response::build_from(responder.respond_to(req)?)
                    .raw_header("Cache-Control", "no-cache")
                    .ok()
            }
            CacheResponse::NoStore(responder) => {
                Response::build_from(responder.respond_to(req)?)
                    .raw_header("Cache-Control", "no-store")
                    .ok()
            }
            CacheResponse::NoCacheControl(responder) => {
                Response::build_from(responder.respond_to(req)?).ok()
            }
        }
    }
}

impl<R: Responder<'static>> CacheResponse<R> {
    /// Use public cache only when this program is built on the **release** mode.
    #[cfg(debug_assertions)]
    pub fn public_only_release(
        responder: R,
        _max_age: u32,
        _must_revalidate: bool,
    ) -> CacheResponse<R> {
        CacheResponse::NoCacheControl(responder)
    }

    /// Use public cache only when this program is built on the **release** mode.
    #[cfg(not(debug_assertions))]
    pub fn public_only_release(
        responder: R,
        max_age: u32,
        must_revalidate: bool,
    ) -> CacheResponse<R> {
        CacheResponse::Public {
            responder,
            max_age,
            must_revalidate,
        }
    }

    /// Use private cache only when this program is built on the **release** mode.
    #[cfg(debug_assertions)]
    pub fn private_only_release(responder: R, _max_age: u32) -> CacheResponse<R> {
        CacheResponse::NoCacheControl(responder)
    }

    /// Use private cache only when this program is built on the **release** mode.
    #[cfg(not(debug_assertions))]
    pub fn private_only_release(responder: R, max_age: u32) -> CacheResponse<R> {
        CacheResponse::Private {
            responder,
            max_age,
        }
    }
}
