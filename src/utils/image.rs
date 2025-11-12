use std::env;
use std::path::{Path, PathBuf};

// AI-generated code
// reference 'rtw_stb_image.h' from https://raytracing.github.io/books/RayTracingTheNextWeek.html

pub struct RtwImage {
    pub width: u32,
    pub height: u32,
    data_rgb8: Vec<u8>,
}

impl Default for RtwImage {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            data_rgb8: Vec::new(),
        }
    }
}

impl RtwImage {
    pub fn from_file(image_filename: impl AsRef<str>) -> Self {
        let filename = image_filename.as_ref();

        if let Ok(dir) = env::var("RTW_IMAGES") {
            let mut p = PathBuf::from(dir);
            p.push(filename);
            if let Some(img) = Self::try_load(&p) {
                return img;
            }
        }

        let candidates = Self::candidate_paths(filename);
        for p in candidates {
            if let Some(img) = Self::try_load(&p) {
                return img;
            }
        }

        eprintln!("ERROR: Could not load image file '{filename}'.");
        Self::default()
    }

    pub fn pixel_data(&self, x: i32, y: i32) -> [u8; 3] {
        if self.data_rgb8.is_empty() || self.width == 0 || self.height == 0 {
            return [255, 0, 255];
        }

        let cx = clamp_i32(x, 0, self.width as i32);
        let cy = clamp_i32(y, 0, self.height as i32);

        let idx = (cy as usize) * (self.width as usize) * 3 + (cx as usize) * 3;
        [
            self.data_rgb8[idx],
            self.data_rgb8[idx + 1],
            self.data_rgb8[idx + 2],
        ]
    }

    fn try_load(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }
        let p = path.to_string_lossy();
        match stb_image::image::load(p.as_ref()) {
            stb_image::image::LoadResult::ImageU8(img) => {
                let data = img.data;
                match img.depth {
                    3 => Some(Self {
                        width: img.width as u32,
                        height: img.height as u32,
                        data_rgb8: data,
                    }),
                    4 => {
                        let mut rgb = Vec::with_capacity(img.width * img.height * 3);
                        for px in data.chunks_exact(4) {
                            rgb.extend_from_slice(&px[0..3]);
                        }
                        Some(Self {
                            width: img.width as u32,
                            height: img.height as u32,
                            data_rgb8: rgb,
                        })
                    }
                    1 => {
                        let mut rgb = Vec::with_capacity(img.width * img.height * 3);
                        for &g in &data {
                            rgb.extend_from_slice(&[g, g, g]);
                        }
                        Some(Self {
                            width: img.width as u32,
                            height: img.height as u32,
                            data_rgb8: rgb,
                        })
                    }
                    _ => None,
                }
            }
            stb_image::image::LoadResult::ImageF32(img) => {
                let to_byte = |v: f32| -> u8 {
                    if v <= 0.0 {
                        0
                    } else if v >= 1.0 {
                        255
                    } else {
                        (v * 256.0) as u8
                    }
                };
                match img.depth {
                    3 => {
                        let mut rgb = Vec::with_capacity(img.width * img.height * 3);
                        for px in img.data.chunks_exact(3) {
                            rgb.push(to_byte(px[0]));
                            rgb.push(to_byte(px[1]));
                            rgb.push(to_byte(px[2]));
                        }
                        Some(Self {
                            width: img.width as u32,
                            height: img.height as u32,
                            data_rgb8: rgb,
                        })
                    }
                    4 => {
                        let mut rgb = Vec::with_capacity(img.width * img.height * 3);
                        for px in img.data.chunks_exact(4) {
                            rgb.push(to_byte(px[0]));
                            rgb.push(to_byte(px[1]));
                            rgb.push(to_byte(px[2]));
                        }
                        Some(Self {
                            width: img.width as u32,
                            height: img.height as u32,
                            data_rgb8: rgb,
                        })
                    }
                    1 => {
                        let mut rgb = Vec::with_capacity(img.width * img.height * 3);
                        for &g in &img.data {
                            let b = to_byte(g);
                            rgb.extend_from_slice(&[b, b, b]);
                        }
                        Some(Self {
                            width: img.width as u32,
                            height: img.height as u32,
                            data_rgb8: rgb,
                        })
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn candidate_paths(filename: &str) -> Vec<PathBuf> {
        let mut list = Vec::with_capacity(9);
        list.push(PathBuf::from(filename));
        list.push(PathBuf::from("images").join(filename));

        for up in 1..=6 {
            let mut p = PathBuf::new();
            for _ in 0..up {
                p.push("..");
            }
            p.push("images");
            p.push(filename);
            list.push(p);
        }

        list
    }
}

fn clamp_i32(x: i32, low: i32, high: i32) -> i32 {
    if x < low {
        return low;
    }
    if x < high {
        return x;
    }
    high - 1
}
