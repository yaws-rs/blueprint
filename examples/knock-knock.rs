//! Knick Knock Who's There - In Orbit form

use blueprint::Orbit;

use core::convert::Infallible;

static TRANSCRIPT: [(&str, &str); 3] = [
    ("Knock, knock!", "Who's there?"),
    ("Blåhaj", "Blue?"),
    ("I have come to eat you.", "I don't care"),
];

#[derive(Debug)]
struct KnockPos {
    pos: usize,
}

#[derive(Debug)]
enum KnockKnock {
    LeftSide(KnockPos),
    RightSide(KnockPos),
}

impl Orbit for KnockKnock {
    type Position = Option<&'static str>;
    type Error = Infallible;
    fn advance_with<B>(
        &mut self,
        _ud: &mut B,
        b_in: &mut [u8],
    ) -> Result<Self::Position, Self::Error> {
        match self {
            Self::LeftSide(ref mut p) => {
                assert_eq!(core::str::from_utf8(b_in).unwrap(), TRANSCRIPT[p.pos].0);
                let ret = TRANSCRIPT[p.pos].1;
                p.pos += 1;
                Ok(Some(ret))
            }
            Self::RightSide(ref mut p) => {
                assert_eq!(core::str::from_utf8(b_in).unwrap(), TRANSCRIPT[p.pos].1);
                match TRANSCRIPT.len() > p.pos + 1 {
                    false => Ok(None),
                    true => {
                        let ret = TRANSCRIPT[p.pos + 1].0;
                        p.pos += 1;
                        Ok(Some(ret))
                    }
                }
            }
        }
    }
}

struct EmptyData;

fn main() {
    let mut teller = KnockKnock::LeftSide(KnockPos { pos: 0 });
    let mut recipient = KnockKnock::RightSide(KnockPos { pos: 0 });

    let mut empty = EmptyData;
    let mut buf_in: Vec<u8> = "Knock, knock!".as_bytes().to_vec();

    while let Ok(Some(teller_resp)) = teller.advance_with(&mut empty, &mut buf_in) {
        println!(">> {}", core::str::from_utf8(&buf_in).unwrap());
        let mut buf_out: Vec<u8> = teller_resp.as_bytes().to_vec();
        match recipient.advance_with(&mut empty, &mut buf_out) {
            Ok(Some(recipient_resp)) => {
                println!("<< {}", core::str::from_utf8(&buf_out).unwrap());
                buf_in = recipient_resp.into();
            }
            Ok(None) => break,
            _ => todo!(),
        }
    }
}
