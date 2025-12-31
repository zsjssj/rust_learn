use image::{DynamicImage, ImageBuffer, Luma, imageops::FilterType};
use qrcode::{EcLevel, QrCode};

pub fn set_qr_code() {
    let url = "https://www.deepsensing.cn/";
    let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::H).unwrap(); // 1. 生成二维码，并设置高纠错等级 H
    let qr_image: ImageBuffer<Luma<u8>, Vec<u8>> = code.render::<Luma<u8>>().min_dimensions(512, 512).build(); // 2. 渲染成大图（512x512，保证清晰）
    let mut qr: ImageBuffer<image::Rgb<u8>, Vec<u8>> = DynamicImage::ImageLuma8(qr_image).to_rgb8();
    let mut logo = image::open("src/assets/company-icon.png").expect("无法打开 logo 图片"); // 3. 打开 logo 图片

    // 缩放 logo（二维码宽度的 20%）
    let qr_width: u32 = qr.width();
    let logo_size: u32 = qr_width / 5;
    logo = logo.resize(logo_size, logo_size, FilterType::Lanczos3);

    // 计算位置 → 中心点
    let x: u32 = (qr_width - logo.width()) / 2;
    let y: u32 = (qr.height() - logo.height()) / 2;

    image::imageops::overlay(&mut qr, &logo.to_rgb8(), x.into(), y.into()); // 6. 覆盖到二维码中间
    qr.save("qrcode_with_logo.png").unwrap(); // 7. 保存最终结果
    println!("✅ 二维码已生成：qrcode_with_logo.png\n扫码后会打开：{}", url);
}
