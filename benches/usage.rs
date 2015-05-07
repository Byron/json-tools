#![feature(test, io)]

extern crate json_tools;
extern crate test;

use json_tools::{Lexer, FilterNull, BufferType};

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

#[bench]
fn lexer_throughput_in_bytes(b: &mut test::Bencher) {
    b.iter(|| {
        let it = Lexer::new(NULL_RIDDEN.bytes(), BufferType::Span);
        for t in it {
            test::black_box(t);
        }
    });
    b.bytes = NULL_RIDDEN.len() as u64;
}



#[bench]
fn filter_null_throughput_in_bytes(b: &mut test::Bencher) {
    b.iter(|| {
        let f = FilterNull::new(Lexer::new(NULL_RIDDEN.bytes(), BufferType::Span));
        for t in f {
            test::black_box(t);
        }
    });
    b.bytes = NULL_RIDDEN.len() as u64;
}


#[bench]
fn lexer_throughput_in_bytes_with_cursor(b: &mut test::Bencher) {
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


#[bench]
fn lexer_throughput_in_bytes_with_cursor_and_tee(b: &mut test::Bencher) {
    use std::io::{Cursor, Read};
    
    b.iter(|| {
        let mut keeper = Cursor::new(Vec::<u8>::new());
        {
          let it = Lexer::new(Cursor::new(NULL_RIDDEN.as_bytes())
                                      .tee(&mut keeper)
                                      .bytes()
                                      .filter_map(|r|r.ok()),
                              BufferType::Span);
          for t in it {
              test::black_box(t);
          }
        }
        assert_eq!(keeper.into_inner().len(), NULL_RIDDEN.len());
    });
    b.bytes = NULL_RIDDEN.len() as u64;
}

