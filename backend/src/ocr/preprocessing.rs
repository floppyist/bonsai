use std::fs;

use opencv::prelude::*;
use opencv::imgproc;
use opencv::imgcodecs;
use opencv::core::no_array;
use opencv::core::DECOMP_LU;
use opencv::core::BORDER_DEFAULT;
use opencv::core::BORDER_CONSTANT;
use opencv::core::AlgorithmHint;
use opencv::core::Scalar;
use opencv::core::Size;
use opencv::core::Mat;
use opencv::core::Point2f;
use opencv::core::Point;
use opencv::core::Vector;

use anyhow::Result;
use base64::Engine;

const DEBUG_DIR: &str = "target/debug_output";

pub fn maximize_text_quality(img_np: &Mat) -> Result<Mat> {
    println!("[DEBUG] Starting text optimization...");

    let mut gray = Mat::default();

    imgproc::cvt_color(
        img_np,
        &mut gray,
        imgproc::COLOR_RGB2GRAY,
        0,
        AlgorithmHint::ALGO_HINT_APPROX,
    )?;

    let kernel = imgproc::get_structuring_element(
        imgproc::MORPH_RECT,
        Size::new(50, 50),
        Point::new(-1, -1),
    )?;

    let mut background = Mat::default();

    imgproc::morphology_ex(
        &gray,
        &mut background,
        imgproc::MORPH_DILATE,
        &kernel,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        imgproc::morphology_default_border_value()?,
    )?;

    let mut background_blur = Mat::default();

    imgproc::gaussian_blur(
        &background,
        &mut background_blur,
        Size::new(51, 51),
        0.0,
        0.0,
        BORDER_DEFAULT,
        AlgorithmHint::ALGO_HINT_APPROX,
    )?;

    let mut normalized = Mat::default();

    opencv::core::divide2(
        &gray,
        &background_blur,
        &mut normalized,
        255.0,
        -1,
    )?;

    let mut denoised = Mat::default();

    imgproc::bilateral_filter(&normalized, &mut denoised, 7, 50.0, 50.0, BORDER_DEFAULT)?;

    const WHITE_THRESHOLD: f64 = 200.0;
    let mut ghosting_mask = Mat::default();

    imgproc::threshold(
        &denoised,
        &mut ghosting_mask,
        WHITE_THRESHOLD,
        255.0,
        imgproc::THRESH_BINARY,
    )?;

    let mut filtered = Mat::default();

    opencv::core::bitwise_or(&denoised, &ghosting_mask, &mut filtered, &no_array())?;

    let mut clahe = imgproc::create_clahe(1.5, Size::new(8, 8))?;
    let mut enhanced = Mat::default();

    clahe.apply(&filtered, &mut enhanced)?;

    println!("[DEBUG] Ghosting filter applied with threshold {}", WHITE_THRESHOLD);

    let mut result = Mat::default();

    imgproc::cvt_color(
        &enhanced,
        &mut result,
        imgproc::COLOR_GRAY2RGB,
        0,
        AlgorithmHint::ALGO_HINT_APPROX,
    )?;

    Ok(result)
}

pub fn crop_to_receipt(img_np: &Mat) -> Result<Mat> {
    println!("[DEBUG] Analyzing image contours for auto-crop...");

    let size = img_np.size()?;
    let h = size.height;
    let w = size.width;
    let scale = 500.0 / (h.max(w) as f32);
    let mut small = Mat::default();

    imgproc::resize(
        img_np,
        &mut small,
        Size::new((w as f32 * scale) as i32, (h as f32 * scale) as i32),
        0.0,
        0.0,
        imgproc::INTER_AREA,
    )?;

    let mut gray = Mat::default();

    imgproc::cvt_color(
        &small,
        &mut gray,
        imgproc::COLOR_RGB2GRAY,
        0,
        AlgorithmHint::ALGO_HINT_APPROX,
    )?;

    let mut blurred = Mat::default();

    imgproc::gaussian_blur(
        &gray,
        &mut blurred,
        Size::new(5, 5),
        0.0,
        0.0,
        BORDER_DEFAULT,
        AlgorithmHint::ALGO_HINT_APPROX,
    )?;

    let mut thresh = Mat::default();

    imgproc::threshold(
        &blurred,
        &mut thresh,
        0.0,
        255.0,
        imgproc::THRESH_BINARY | imgproc::THRESH_OTSU,
    )?;

    let mut contours = Vector::<Vector<Point>>::new();

    imgproc::find_contours(
        &thresh,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )?;

    if !contours.is_empty() {
        let mut max_cnt = contours.get(0)?;
        let mut max_area = imgproc::contour_area(&max_cnt, false)?;

        for i in 1..contours.len() {
            let cnt = contours.get(i)?;
            let area = imgproc::contour_area(&cnt, false)?;

            if area > max_area {
                max_area = area;
                max_cnt = cnt;
            }
        }

        let small_size = small.size()?;

        if max_area > (small_size.width * small_size.height) as f64 * 0.08 {
            println!("[DEBUG] Valid contour found with area {:.0}px. Processing crop...", max_area);

            let mut max_cnt_f32 = Vector::<Point2f>::new();

            for pt in max_cnt.iter() {
                max_cnt_f32.push(Point2f::new(pt.x as f32, pt.y as f32));
            }

            let rect = imgproc::min_area_rect(&max_cnt_f32)?;
            let mut pts = [Point2f::default(); 4];

            rect.points(&mut pts)?;

            for pt in pts.iter_mut() {
                pt.x /= scale;
                pt.y /= scale;
            }

            let mut tl = pts[0];
            let mut tr = pts[0];
            let mut br = pts[0];
            let mut bl = pts[0];

            let mut min_sum = f32::MAX;
            let mut max_sum = f32::MIN;
            let mut min_diff = f32::MAX;
            let mut max_diff = f32::MIN;

            for pt in pts.iter() {
                let sum = pt.x + pt.y;
                let diff = pt.y - pt.x;

                if sum < min_sum { min_sum = sum; tl = *pt; }
                if sum > max_sum { max_sum = sum; br = *pt; }
                if diff < min_diff { min_diff = diff; tr = *pt; }
                if diff > max_diff { max_diff = diff; bl = *pt; }
            }

            let w_a = ((br.x - bl.x).powi(2) + (br.y - bl.y).powi(2)).sqrt();
            let w_b = ((tr.x - tl.x).powi(2) + (tr.y - tl.y).powi(2)).sqrt();
            let max_w = w_a.max(w_b) as i32;

            let h_a = ((tr.x - br.x).powi(2) + (tr.y - br.y).powi(2)).sqrt();
            let h_b = ((tl.x - bl.x).powi(2) + (tl.y - bl.y).powi(2)).sqrt();
            let max_h = h_a.max(h_b) as i32;

            let src_pts = Vector::<Point2f>::from_iter(vec![tl, tr, br, bl]);

            let dst_pts = Vector::<Point2f>::from_iter(vec![
                Point2f::new(0.0, 0.0),
                Point2f::new((max_w - 1) as f32, 0.0),
                Point2f::new((max_w - 1) as f32, (max_h - 1) as f32),
                Point2f::new(0.0, (max_h - 1) as f32),
            ]);

            let m = imgproc::get_perspective_transform(
                &src_pts,
                &dst_pts,
                DECOMP_LU,
            )?;

            let mut warped = Mat::default();

            imgproc::warp_perspective(
                img_np,
                &mut warped,
                &m,
                Size::new(max_w, max_h),
                imgproc::INTER_LINEAR,
                BORDER_CONSTANT,
                Scalar::default(),
            )?;

            return Ok(warped);
        }
    }

    println!("[DEBUG] No distinct receipt contour detected. Falling back to original image dimensions.");
    Ok(img_np.clone())
}

pub fn process_image_pipeline_from_mat(img_bgr: &Mat) -> Result<String> {
    let mut img_rgb = Mat::default();

    imgproc::cvt_color(
        img_bgr,
        &mut img_rgb,
        imgproc::COLOR_BGR2RGB,
        0,
        AlgorithmHint::ALGO_HINT_APPROX
    )?;

    let cropped_rgb = crop_to_receipt(&img_rgb)?;
    let enhanced_rgb = maximize_text_quality(&cropped_rgb)?;
    let cur_size = enhanced_rgb.size()?;
    let max_side = 680;
    let mut final_rgb = enhanced_rgb;

    if cur_size.height.max(cur_size.width) > max_side {
        let scale_vlm = max_side as f32 / (cur_size.height.max(cur_size.width) as f32);
        let mut resized = Mat::default();

        imgproc::resize(
            &final_rgb,
            &mut resized,
            Size::new(
                (cur_size.width as f32 * scale_vlm) as i32,
                (cur_size.height as f32 * scale_vlm) as i32,
            ),
            0.0,
            0.0,
            imgproc::INTER_AREA,
        )?;

        final_rgb = resized;

        let new_size = final_rgb.size()?;

        println!("[DEBUG] Image scaled down to {}x{}", new_size.width, new_size.height);
    }

    fs::create_dir_all(DEBUG_DIR)?;

    let debug_out_path = format!("{}/prep_output.jpg", DEBUG_DIR);
    let mut final_bgr = Mat::default();

    imgproc::cvt_color(
        &final_rgb,
        &mut final_bgr,
        imgproc::COLOR_RGB2BGR,
        0,
        AlgorithmHint::ALGO_HINT_APPROX,
    )?;

    imgcodecs::imwrite(
        &debug_out_path,
        &final_bgr,
        &Vector::new()
    )?;

    println!("[DEBUG] Preprocessed debug image saved to {}", debug_out_path);

    let mut buffer = Vector::<u8>::new();
    let mut params = Vector::<i32>::new();

    params.push(imgcodecs::IMWRITE_JPEG_QUALITY);
    params.push(90);

    imgcodecs::imencode(
        ".jpg",
        &final_bgr,
        &mut buffer,
        &params
    )?;

    let b64_encoded = base64::engine::general_purpose::STANDARD.encode(buffer.to_vec());

    Ok(b64_encoded)
}
