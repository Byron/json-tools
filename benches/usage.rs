#![feature(test)]

extern crate json_tools;
extern crate test;

use json_tools::{Lexer, FilterTypedKeyValuePairs, BufferType, TokenType, 
                 TokenReader, Token, Buffer, Span};
use std::io;

const NULL_RIDDEN: &'static str = r##"
{
  "mimeType": "text/plain",
  "lastViewedByMeDate": "2015-05-06T09:02:13.534Z",
  "appDataContents": false,
  "thumbnailLink": "https://lh4.googleusercontent.com/TWSXDyaZjIguDn-OkA34iftyT5_BcqwXg9B2hti2Cj-zihmETye4wvzkUUzYp1JlqRLNOA=s220",
  "labels": {
    "restricted": false,
    "hidden": false,
    "trashed": false,
    "starred": false,
    "viewed": true
  },
  "indexableText": null,
  "explicitlyTrashed": null,
  "etag": "\"dM4Z0GasI3ekQlrgb3F8B4ytx24/MTQzMDkwMjkzMzUzNA\"",
  "lastModifyingUserName": "Sebastian Thiel",
  "writersCanShare": true,
  "sharedWithMeDate": null,
  "sharingUser": null,
  "videoMediaMetadata": null,
  "lastModifyingUser": {
    "picture": {
      "url": "https://lh4.googleusercontent.com/-HGpc6yArgN4/AAAAAAAAAAI/AAAAAAAAAH8/pEN6OV2M3DA/s64/photo.jpg"
    },
    "kind": "drive#user",
    "isAuthenticatedUser": true,
    "displayName": "Sebastian Thiel",
    "emailAddress": "byronimo@gmail.com",
    "permissionId": "09412481287368086409"
  },
  "copyable": true,
  "folderColorRgb": null,
  "ownerNames": [
    "Sebastian Thiel"
  ],
  "kind": "drive#file",
  "id": "0B0fWD6X9_jUsR0pKNDl5QjlPcjQ",
  "webViewLink": null,
  "version": "3904",
  "parents": [
    {
      "isRoot": true,
      "kind": "drive#parentReference",
      "id": "0AEfWD6X9_jUsUk9PVA",
      "selfLink": "https://www.googleapis.com/drive/v2/files/0B0fWD6X9_jUsR0pKNDl5QjlPcjQ/parents/0AEfWD6X9_jUsUk9PVA",
      "parentLink": "https://www.googleapis.com/drive/v2/files/0AEfWD6X9_jUsUk9PVA"
    }
  ],
  "exportLinks": null,
  "shared": false,
  "iconLink": "https://ssl.gstatic.com/docs/doclist/images/icon_10_text_list.png",
  "thumbnail": null,
  "openWithLinks": null,
  "defaultOpenWithLink": null,
  "description": null,
  "webContentLink": "https://docs.google.com/uc?id=0B0fWD6X9_jUsR0pKNDl5QjlPcjQ&export=download",
  "editable": true,
  "embedLink": null,
  "markedViewedByMeDate": "1970-01-01T00:00:00.000Z",
  "fileExtension": "md",
  "fileSize": "9264",
  "createdDate": "2015-05-06T09:02:13.756Z",
  "properties": null,
  "md5Checksum": "a01d5331dbcc56bd656630f2a846ca5c",
  "permissions": null,
  "imageMediaMetadata": null,
  "owners": [
    {
      "picture": {
        "url": "https://lh4.googleusercontent.com/-HGpc6yArgN4/AAAAAAAAAAI/AAAAAAAAAH8/pEN6OV2M3DA/s64/photo.jpg"
      },
      "kind": "drive#user",
      "isAuthenticatedUser": true,
      "displayName": "Sebastian Thiel",
      "emailAddress": "byronimo@gmail.com",
      "permissionId": "09412481287368086409"
    }
  ],
  "alternateLink": "https://docs.google.com/file/d/0B0fWD6X9_jUsR0pKNDl5QjlPcjQ/edit?usp=drivesdk",
  "title": "README.md",
  "modifiedByMeDate": "2015-05-06T09:02:13.534Z",
  "downloadUrl": "https://doc-0c-40-docs.googleusercontent.com/docs/securesc/cdcfmpofhgddu1smts6vlnl56a2m4f5j/12t2jnk1b0m9tq9cppoldiasgenoodgg/1430920800000/09412481287368086409/09412481287368086409/0B0fWD6X9_jUsR0pKNDl5QjlPcjQ?e=download&gd=true",
  "userPermission": {
    "withLink": null,
    "domain": null,
    "name": null,
    "kind": "drive#permission",
    "value": null,
    "additionalRoles": null,
    "authKey": null,
    "etag": "\"dM4Z0GasI3ekQlrgb3F8B4ytx24/zq_yOEtiIpRN7r2eb9bP9h60QBI\"",
    "emailAddress": null,
    "photoLink": null,
    "role": "owner",
    "type": "user",
    "id": "me",
    "selfLink": "https://www.googleapis.com/drive/v2/files/0B0fWD6X9_jUsR0pKNDl5QjlPcjQ/permissions/me"
  },
  "originalFilename": "README.md",
  "quotaBytesUsed": "9264",
  "headRevisionId": "0B0fWD6X9_jUsdUtzVzVPSnFKYkNvS21mWlFtM2xDdExacjFRPQ",
  "selfLink": "https://www.googleapis.com/drive/v2/files/0B0fWD6X9_jUsR0pKNDl5QjlPcjQ",
  "modifiedDate": "2015-05-06T09:02:13.534Z"
}
"##;

const NUM_TOKENS: usize = 4;
const KEY_VALUE_SRC: &'static str = r#""key":"some string value""#;

struct KeyValueProducer {
  buf: [Token; NUM_TOKENS],
  cur: usize,
}

impl KeyValueProducer {
  fn new(bt: BufferType) -> KeyValueProducer {
    KeyValueProducer {
      buf: [
            Token {
              kind: TokenType::String,
              buf: match bt {
                BufferType::Bytes(_) => Buffer::MultiByte(KEY_VALUE_SRC[0..5].into()),
                BufferType::Span => Buffer::Span(Span { first: 0, end: 5 })
              }
            },
            Token {
              kind: TokenType::Colon,
              buf: Buffer::Span(Span::default()),
            },
            Token {
              kind: TokenType::String,
              buf: match bt {
                BufferType::Bytes(_) => Buffer::MultiByte(KEY_VALUE_SRC[6..25].into()),
                BufferType::Span => Buffer::Span(Span { first: 6, end: 25 })
              }
            },
            Token {
              kind: TokenType::Comma,
              buf: Buffer::Span(Span::default()),
            },
          ],
      cur: 0,
    }
  }
}

impl Iterator for KeyValueProducer {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    if self.cur == NUM_TOKENS {
      self.cur = 0;
    }

    let res = self.buf[self.cur].clone();
    self.cur += 1;
    Some(res)
  }
}

#[bench]
fn span_lexer_throughput(b: &mut test::Bencher) {
    b.iter(|| {
        let it = Lexer::new(NULL_RIDDEN.bytes(), BufferType::Span);
        for t in it {
            test::black_box(t);
        }
    });
    b.bytes = NULL_RIDDEN.len() as u64;
}

#[bench]
fn span_lexer_span_token_reader_throughput(b: &mut test::Bencher) {
    b.iter(|| {
        let mut r = TokenReader::new(Lexer::new(NULL_RIDDEN.bytes(), BufferType::Span), 
                                     Some(NULL_RIDDEN));
        io::copy(&mut r, &mut io::sink()).ok();
    });
    b.bytes = NULL_RIDDEN.len() as u64;
}

#[bench]
fn span_lexer_bytes_token_reader_throughput(b: &mut test::Bencher) {
    b.iter(|| {
        let mut r = TokenReader::new(Lexer::new(NULL_RIDDEN.bytes(), BufferType::Bytes(128)), 
                                     None);
        io::copy(&mut r, &mut io::sink()).ok();
    });
    b.bytes = NULL_RIDDEN.len() as u64;
}

#[bench]
fn bytes_token_producer_bytes_token_reader_throughput(b: &mut test::Bencher) {
    let mut ncb = 0u64;
    b.iter(|| {
        let mut r = TokenReader::new(KeyValueProducer::new(BufferType::Bytes(0)).take(30000), None);
        ncb = io::copy(&mut r, &mut io::sink()).unwrap();
    });
    b.bytes = ncb;
}

#[bench]
fn span_token_producer_bytes_token_reader_throughput(b: &mut test::Bencher) {
    let mut ncb = 0u64;
    b.iter(|| {
        let mut r = TokenReader::new(KeyValueProducer::new(BufferType::Span).take(30000), Some(KEY_VALUE_SRC));
        ncb = io::copy(&mut r, &mut io::sink()).unwrap();
    });
    b.bytes = ncb;
}

#[bench]
fn bytes_lexer_throughput(b: &mut test::Bencher) {
    b.iter(|| {
        let it = Lexer::new(NULL_RIDDEN.bytes(), BufferType::Bytes(128));
        for t in it {
            test::black_box(t);
        }
    });
    b.bytes = NULL_RIDDEN.len() as u64;
}


#[bench]
fn span_filter_null_throughput(b: &mut test::Bencher) {
    b.iter(|| {
        let f = FilterTypedKeyValuePairs::new(Lexer::new(NULL_RIDDEN.bytes(), BufferType::Span),
                                              TokenType::Null);
        for t in f {
            test::black_box(t);
        }
    });
    b.bytes = NULL_RIDDEN.len() as u64;
}


#[bench]
fn span_lexer_throughput_with_cursor(b: &mut test::Bencher) {
    use std::io::{Cursor, Read};
    
    b.iter(|| {
        let it = Lexer::new(Cursor::new(NULL_RIDDEN.as_bytes()).bytes().filter_map(|r|r.ok()),
                            BufferType::Span);
        for t in it {
            test::black_box(t);
        }
    });
    b.bytes = NULL_RIDDEN.len() as u64;
}
