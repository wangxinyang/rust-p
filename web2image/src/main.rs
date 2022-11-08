use web2image::{generate_qr_code, get_args, save_screen_shot};

fn main() {
    let args = get_args().unwrap();
    // call screenshot method
    save_screen_shot(args.url.as_str(), args.output.to_str().unwrap()).unwrap();
    // generated qrcode png file
    generate_qr_code(args.url.as_str(), args.output.to_str().unwrap()).unwrap();
}
