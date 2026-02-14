//! Плагин зеркального разворота изображения.

#![deny(unreachable_pub)]

use core::ffi::c_char;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::ffi::CStr;

/// Параметры плагина.
#[derive(Debug, Serialize, Deserialize)]
struct Params {
    /// Отразить по горизонтали.
    horizontal: bool,

    /// Отразить по вертикали.
    vertical: bool,
}

/// Обработка изображения.
///
/// В случае некорректности переданных параметров будет выдано соответствующее сообщение. При этом
/// входной массив данных останется без изменений.
///
/// # Safety
/// * len(rgba_data) == width * height * 4
/// * params - валидная C-строка != NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) {
    let params = match from_slice::<Params>(unsafe { CStr::from_ptr(params) }.to_bytes()) {
        Ok(params) => params,
        Err(e) => {
            eprintln!("Некорректный формат переданных параметров: {}", e);
            return;
        }
    };

    let len = (width * height * 4) as usize;
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, len) };

    if params.horizontal {
        for y in 0..height as usize {
            for x in 0..width as usize / 2 {
                let left = y * width as usize * 4 + x * 4;
                let right = y * width as usize * 4 + (width as usize - 1 - x) * 4;
                for i in 0..4 {
                    data.swap(left + i, right + i);
                }
            }
        }
    }

    if params.vertical {
        for y in 0..height as usize / 2 {
            for x in 0..width as usize {
                let top = y * width as usize * 4 + x * 4;
                let bottom = (height as usize - 1 - y) * width as usize * 4 + x * 4;
                for i in 0..4 {
                    data.swap(top + i, bottom + i);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::ffi::CString;

    fn prepare_data(width: u32, height: u32) -> Vec<u8> {
        (0..width * height * 4)
            .map(|val| val as u8)
            .collect::<Vec<_>>()
    }

    #[rstest]
    #[case(0, 0, &[])]
    #[case(2, 2, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])]
    fn test_stable(#[case] width: u32, #[case] height: u32, #[case] expected: &[u8]) {
        let mut data = prepare_data(width, height);

        let params = CString::new(
            serde_json::to_string(&Params {
                horizontal: false,
                vertical: false,
            })
            .unwrap(),
        )
        .unwrap();

        unsafe { process_image(width, height, data.as_mut_ptr(), params.as_ptr()) };

        assert_eq!(data, expected);
    }

    #[rstest]
    #[case(2, 2, &[
            4, 5, 6, 7, 0, 1, 2, 3,
            12, 13, 14, 15, 8, 9, 10, 11,
        ])]
    #[case(3, 2, &[
            8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3, 20, 21, 22, 23, 16, 17, 18, 19, 12, 13, 14, 15,
        ])]
    fn test_horizontal(#[case] width: u32, #[case] height: u32, #[case] expected: &[u8]) {
        let mut data = prepare_data(width, height);

        let params = CString::new(
            serde_json::to_string(&Params {
                horizontal: true,
                vertical: false,
            })
            .unwrap(),
        )
        .unwrap();

        unsafe { process_image(width, height, data.as_mut_ptr(), params.as_ptr()) };

        assert_eq!(data, expected);
    }

    #[rstest]
    #[case(2, 2, &[8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7])]
    #[case(2, 3, &[
            16, 17, 18, 19, 20, 21, 22, 23, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7,
        ])]
    fn test_vertical(#[case] width: u32, #[case] height: u32, #[case] expected: &[u8]) {
        let mut data = prepare_data(width, height);

        let params = CString::new(
            serde_json::to_string(&Params {
                horizontal: false,
                vertical: true,
            })
            .unwrap(),
        )
        .unwrap();

        unsafe { process_image(width, height, data.as_mut_ptr(), params.as_ptr()) };

        assert_eq!(data, expected);
    }

    #[rstest]
    #[case(2, 2, &[
            12, 13, 14, 15, 8, 9, 10, 11,
            4, 5, 6, 7, 0, 1, 2, 3,
        ])]
    #[case(2, 3, &[
            20, 21, 22, 23, 16, 17, 18, 19,
            12, 13, 14, 15, 8, 9, 10, 11,
            4, 5, 6, 7, 0, 1, 2, 3,
        ])]
    #[case(3, 2, &[
            20, 21, 22, 23, 16, 17, 18, 19, 12, 13, 14, 15,
            8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3,
        ])]
    fn test_both(#[case] width: u32, #[case] height: u32, #[case] expected: &[u8]) {
        let mut data = prepare_data(width, height);

        let params = CString::new(
            serde_json::to_string(&Params {
                horizontal: true,
                vertical: true,
            })
            .unwrap(),
        )
        .unwrap();

        unsafe { process_image(width, height, data.as_mut_ptr(), params.as_ptr()) };

        assert_eq!(data, expected);
    }
}
