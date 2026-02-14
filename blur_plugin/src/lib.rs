//! Плагин размытия изображения.

use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::ffi::{CStr, c_char};

/// Параметры плагина.
#[derive(Debug, Serialize, Deserialize)]
struct Params {
    /// Радиус размытия.
    radius: u32,

    /// Количество итераций.
    iterations: u32,
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
            log::error!("Некорректный формат переданных параметров: {}", e);
            return;
        }
    };

    let len = (width * height * 4) as usize;
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, len) };

    let height = height as i64;
    let width = width as i64;

    let radius = params.radius as i64;
    let mut buf = vec![0u8; len];

    for _ in 0..params.iterations {
        for cy in 0..height {
            for cx in 0..width {
                let mut sum = [0; 4];
                let mut weight_sum = 0;

                let y_start = (cy - radius).max(0);
                let y_end = (cy + radius).min(height - 1);
                let x_start = (cx - radius).max(0);
                let x_end = (cx + radius).min(width - 1);

                for ny in y_start..=y_end {
                    for nx in x_start..=x_end {
                        let dx = cx - nx;
                        let dy = cy - ny;
                        let dist = (dx * dx + dy * dy).isqrt();

                        if dist > radius {
                            continue;
                        }

                        let idx = (ny * width + nx) as usize * 4;
                        for c in 0..4 {
                            sum[c] += data[idx + c] as i64 * dist;
                        }
                        weight_sum += dist;
                    }
                }

                let idx = (cy * width + cx) as usize * 4;
                if weight_sum > 0 {
                    for c in 0..4 {
                        buf[idx + c] = (sum[c] / weight_sum) as u8;
                    }
                } else {
                    buf[idx..idx + 4].copy_from_slice(&data[idx..idx + 4]);
                }
            }
        }
        data.copy_from_slice(&buf);
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
    #[case(1, 1, &[0, 1, 2, 3])]
    #[case(2, 2, &[
            8, 9, 10, 11, 6, 7, 8, 9,
            5, 6, 7, 8, 4, 5, 6, 7,
        ])]
    #[case(3, 2, &[
            12, 13, 14, 15, 11, 12, 13, 14, 9, 10, 11, 12,
            10, 11, 12, 13, 8, 9, 10, 11, 7, 8, 9, 10,
        ])]
    fn test_blur(#[case] width: u32, #[case] height: u32, #[case] expected: &[u8]) {
        let mut data = prepare_data(width, height);

        let params = CString::new(
            serde_json::to_string(&Params {
                radius: 5,
                iterations: 1,
            })
            .unwrap(),
        )
        .unwrap();

        unsafe { process_image(width, height, data.as_mut_ptr(), params.as_ptr()) };

        assert_eq!(data, expected);
    }

    #[rstest]
    #[case(2, 2, &[
            5, 6, 7, 8, 5, 6, 7, 8,
            6, 7, 8, 9, 6, 7, 8, 9,
        ])]
    fn test_blur_iterations(#[case] width: u32, #[case] height: u32, #[case] expected: &[u8]) {
        let mut data = prepare_data(width, height);

        let params = CString::new(
            serde_json::to_string(&Params {
                radius: 5,
                iterations: 2,
            })
            .unwrap(),
        )
        .unwrap();

        unsafe { process_image(width, height, data.as_mut_ptr(), params.as_ptr()) };

        assert_eq!(data, expected);
    }

    #[rstest]
    #[case(3, 3, &[
            10, 11, 12, 13, 11, 12, 13, 14, 13, 14, 15, 16,
            14, 15, 16, 17, 16, 17, 18, 19, 17, 18, 19, 20,
            18, 19, 20, 21, 20, 21, 22, 23, 21, 22, 23, 24,
        ])]
    fn test_blur_small_radius(#[case] width: u32, #[case] height: u32, #[case] expected: &[u8]) {
        let mut data = prepare_data(width, height);

        let params = CString::new(
            serde_json::to_string(&Params {
                radius: 1,
                iterations: 1,
            })
            .unwrap(),
        )
        .unwrap();

        unsafe { process_image(width, height, data.as_mut_ptr(), params.as_ptr()) };

        assert_eq!(data, expected);
    }
}
