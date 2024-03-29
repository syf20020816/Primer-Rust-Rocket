use http_parser::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;
use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => {
                let s = &req.resource;
                let route: Vec<&str> = s.split("/").collect();
                match route[1] {
                    "api" => {
                        let resp: HttpResponse = WebServiceHandler::handle(&req);
                        let _ = resp.send_response(stream);
                    }
                    _ => {
                        let resp: HttpResponse = StaticPageHandler::handle(&req);
                        let _ = resp.send_response(stream);
                    }
                }
            }
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}