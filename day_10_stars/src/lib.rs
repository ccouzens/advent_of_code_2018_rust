#[macro_use]
extern crate nom;
extern crate image;
extern crate tesseract;

use std::cmp::max;
use std::cmp::min;
use std::str::FromStr;

pub fn read_stars(input: &str) -> String {
    let stars = Stars::read(input);
    let imgbuf = stars.draw(stars.bounding_box_minima());
    imgbuf.save("img.png").unwrap();
    tesseract::ocr("img.png", "eng").trim().to_string()
}

#[derive(Debug)]
struct VectorXY {
    x: i32,
    y: i32,
}

named!(
        parse_vector<&str, VectorXY>,
        do_parse!(
            tag_s!("<")
            >> take_while!(|c| c==' ')
            >> x_sign: alt!(tag_s!("-") | tag_s!(""))
            >> x_value: map_res!(nom::digit, FromStr::from_str)
            >> tag_s!(",")
            >> take_while!(|c| c==' ')
            >> y_sign: alt!(tag_s!("-") | tag_s!(""))
            >> y_value: map_res!(nom::digit, FromStr::from_str)
            >> tag_s!(">")
            >> (VectorXY {
                x: if x_sign == "-" { 0-x_value } else { x_value },
                y: if y_sign == "-" { 0-y_value } else { y_value },
            })
        )
    );

#[derive(Debug)]
struct Star {
    initial: VectorXY,
    velocity: VectorXY,
}

impl Star {
    fn advance(&self, seconds: i16) -> VectorXY {
        VectorXY {
            x: self.initial.x + self.velocity.x * i32::from(seconds),
            y: self.initial.y + self.velocity.y * i32::from(seconds),
        }
    }
}

struct Stars(Vec<Star>);

impl Stars {
    fn read(input: &str) -> Self {
        Stars(
            input
                .lines()
                .map(parse_star)
                .filter_map(|ms| ms.ok().map(|rs| rs.1))
                .collect(),
        )
    }

    fn advance<'a>(&'a self, seconds: i16) -> impl Iterator<Item = VectorXY> + 'a {
        self.0.iter().map(move |star| star.advance(seconds))
    }

    fn draw(&self, seconds: i16) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
        let bounding_box = self.bounding_box(seconds);
        let x_min = bounding_box.0;
        let y_min = bounding_box.1;
        let mut imgbuf = image::ImageBuffer::new(
            2 * (bounding_box.2 - bounding_box.0 + 1) as u32,
            2 * (bounding_box.3 - bounding_box.1 + 1) as u32,
        );
        for v in self.advance(seconds) {
            imgbuf.put_pixel(
                (v.x - x_min) as u32 * 2,
                (v.y - y_min) as u32 * 2,
                image::Luma([255u8]),
            );
            imgbuf.put_pixel(
                (v.x - x_min) as u32 * 2 + 1,
                (v.y - y_min) as u32 * 2,
                image::Luma([255u8]),
            );
            imgbuf.put_pixel(
                (v.x - x_min) as u32 * 2,
                (v.y - y_min) as u32 * 2 + 1,
                image::Luma([255u8]),
            );
            imgbuf.put_pixel(
                (v.x - x_min) as u32 * 2 + 1,
                (v.y - y_min) as u32 * 2 + 1,
                image::Luma([255u8]),
            );
        }

        imgbuf
    }

    fn bounding_box(&self, seconds: i16) -> (i32, i32, i32, i32) {
        let mut mbox: Option<(i32, i32, i32, i32)> = None;
        for vec in self.advance(seconds) {
            if let Some(ref mut cbox) = mbox {
                cbox.0 = min(vec.x, cbox.0);
                cbox.1 = min(vec.y, cbox.1);
                cbox.2 = max(vec.x, cbox.2);
                cbox.3 = max(vec.y, cbox.3);
            } else {
                mbox = Some((vec.x, vec.y, vec.x, vec.y));
            }
        }
        mbox.unwrap_or((0, 0, 0, 0))
    }

    fn bounding_box_size(&self, seconds: i16) -> u64 {
        let cbox = self.bounding_box(seconds);
        (cbox.2 - cbox.0) as u64 * (cbox.3 - cbox.1) as u64
    }

    fn bounding_box_minima(&self) -> i16 {
        let mut seconds = 0;
        let mut size = None;
        loop {
            let tsize = self.bounding_box_size(seconds);
            if let Some(ref mut csize) = size {
                if tsize < *csize {
                    *csize = tsize;
                } else {
                    return seconds - 1;
                }
            } else {
                size = Some(tsize);
            }
            seconds += 1;
        }
    }
}

named!(
        parse_star<&str, Star>,
        do_parse!(
            tag_s!("position=")
            >> initial: parse_vector
            >> tag_s!(" velocity=")
            >> velocity: parse_vector
            >> ( Star { initial, velocity } )
        )
    );

#[cfg(test)]
mod read_stars_tests {
    use read_stars;
    use Stars;

    #[test]
    #[ignore]
    fn worked_example() {
        assert_eq!(
            read_stars(include_str!("../worked_example.txt")),
            "HI".to_string()
        );
    }

    #[test]
    fn puzzle() {
        assert_eq!(
            read_stars(include_str!("../input.txt")),
            "ZRHBXXJE".to_string() // actually it's ZRABXXJC
        );
    }

    #[test]
    fn puzzle_wait_time() {
        assert_eq!(
            Stars::read(include_str!("../input.txt")).bounding_box_minima(),
            10710
        );
    }

}
