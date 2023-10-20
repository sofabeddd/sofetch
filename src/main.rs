
mod sofetch;
use crate::sofetch::SystemInfoFetcher;

fn color_string(mut input: String, color_map: [&'static str; 7]) -> String {


    for i in 0..7 {
        input = input.replace(format!("${}", i).as_str(), color_map[i])
    }

    input
}

fn main() {
    let info = SystemInfoFetcher::new();

    let package_info = info.get_packages();

    println!("{}", color_string(String::from("$4╺━━┱ $1\u{f0b00}\u{f0b39}\u{f0af3}\u{f0af2}\u{f0b01}\u{f0af0}\u{f0af5} $4╼━━━━━━━━━━━━━━━━━━━━━"), info.get_distro_colors()));
    println!("{}", color_string(format!("$1 {} $4┠ $5OS              $6{}$0", info.get_distro_nf_glyph(), info.get_distro()), info.get_distro_colors()));
    println!("{}", color_string(format!("$1 \u{ebc6} $4┠ $5Kernel          $6{}$0", info.get_kernel_version()), info.get_distro_colors()));
    println!("{}", color_string(format!("$1 \u{ebc6} $4┠ $5Packages        $6{}$0", package_info), info.get_distro_colors()));
}
