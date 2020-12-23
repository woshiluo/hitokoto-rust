use crate::{Hitokoto, HitokotoError, HitokotoSegment};

use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Serialize)]
pub enum ParseError {
    WrongArgumentNumber,
    Notnumber,
}

#[derive(Serialize)]
pub enum OutputEnum<'a> {
    Hitokoto(&'a HitokotoSegment),
    ParseError(ParseError),
    HitokotoError(HitokotoError),
}

#[derive(Serialize)]
struct Output<'a> {
    code: i32,
    msg: Option<String>,
    data: Option<&'a HitokotoSegment>,
}

impl Output<'_> {
    fn new<'a>(res: OutputEnum<'a>) -> Output<'a> {
        let code = match res {
            OutputEnum::Hitokoto(_) => 0,
            OutputEnum::ParseError(_) => 1,
            OutputEnum::HitokotoError(_) => 2,
        };
        let msg = match &res {
            OutputEnum::Hitokoto(_) => Some("Success".to_string()),
            OutputEnum::ParseError(err) => Some(
                match err {
                    ParseError::Notnumber => "No a number",
                    ParseError::WrongArgumentNumber => "expected 2 arguments",
                }
                .to_string(),
            ),
            OutputEnum::HitokotoError(err) => Some(serde_json::to_string(err).unwrap()),
        };
        let data = match res {
            OutputEnum::Hitokoto(segment) => Some(segment),
            _ => None,
        };
        Output { code, msg, data }
    }
}

fn process_path(path: Option<&str>) -> Result<(u32, u32), ParseError> {
    match path {
        Some(path) => {
            let path = std::path::Path::new(path);
            if path.file_name().is_none()
                || path.parent().is_none()
                || path.parent().unwrap().file_name().is_none()
            {
                return Err(ParseError::WrongArgumentNumber);
            };
            let hitokoto_id = path.file_name().unwrap().to_str().unwrap();
            let cat_id = path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            if cat_id.parse::<u32>().is_err() || hitokoto_id.parse::<u32>().is_err() {
                return Err(ParseError::Notnumber);
            };
            let cat_id = cat_id.parse::<u32>().unwrap();
            let hitokoto_id = hitokoto_id.parse::<u32>().unwrap();
            Ok((hitokoto_id, cat_id))
        }
        _ => Err(ParseError::WrongArgumentNumber),
    }
}

fn process<'a>(hitokoto: &'a Hitokoto, req: httparse::Request) -> OutputEnum<'a> {
    let (hitokoto_id, cat_id) = match process_path(req.path) {
        Ok(a) => a,
        Err(err) => return OutputEnum::ParseError(err),
    };
    let segment = match hitokoto.get_hitokoto(cat_id, hitokoto_id) {
        Ok(segment) => segment,
        Err(err) => return OutputEnum::HitokotoError(err),
    };

    OutputEnum::Hitokoto(segment)
}

pub fn handle_client(hitokoto: &Hitokoto, mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    let mut headers = [httparse::EMPTY_HEADER; 16];
    stream.read(&mut buffer).unwrap();
    let mut req = httparse::Request::new(&mut headers);
    match req.parse(&buffer) {
        Err(err) => {
            log::warn!(
                "Failed to parse request: {:?}, give up to process this.",
                err
            );
            stream.write(format!("HTTP/1.1 500").as_bytes()).unwrap();
            stream.flush().unwrap();
            return ();
        }
        _ => (),
    };
    let response = "HTTP/1.1 200 OK\r\ncontent-type: application/json; charset=utf-8\r\n\r\n";
    let res_json = serde_json::to_string(&Output::new(process(hitokoto, req))).unwrap();
    stream
        .write(format!("{}{}", response, res_json).as_bytes())
        .unwrap();
    stream.flush().unwrap();
}
