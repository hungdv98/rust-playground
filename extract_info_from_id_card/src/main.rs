use std::collections::HashMap;

#[derive(Debug)]
struct CitizenInfo {
    province: String,
    gender: String,
    year_of_birth: u16,
}

fn province_codes() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("001", "Hà Nội");
    map.insert("002", "Hà Giang");
    map.insert("004", "Cao Bằng");
    map.insert("006", "Bắc Kạn");
    map.insert("008", "Tuyên Quang");
    map.insert("010", "Lào Cai");
    map.insert("011", "Điện Biên");
    map.insert("012", "Lai Châu");
    map.insert("014", "Sơn La");
    map.insert("015", "Yên Bái");
    map.insert("017", "Hoà Bình");
    map.insert("019", "Thái Nguyên");
    map.insert("020", "Lạng Sơn");
    map.insert("022", "Quảng Ninh");
    map.insert("024", "Bắc Giang");
    map.insert("025", "Phú Thọ");
    map.insert("026", "Vĩnh Phúc");
    map.insert("027", "Bắc Ninh");
    map.insert("030", "Hải Dương");
    map.insert("031", "Hải Phòng");
    map.insert("033", "Hưng Yên");
    map.insert("034", "Thái Bình");
    map.insert("035", "Hà Nam");
    map.insert("036", "Nam Định");
    map.insert("037", "Ninh Bình");
    map.insert("038", "Thanh Hóa");
    map.insert("040", "Nghệ An");
    map.insert("042", "Hà Tĩnh");
    map.insert("044", "Quảng Bình");
    map.insert("045", "Quảng Trị");
    map.insert("046", "Thừa Thiên - Huế");
    map.insert("048", "Đà Nẵng");
    map.insert("049", "Quảng Nam");
    map.insert("051", "Quảng Ngãi");
    map.insert("052", "Bình Định");
    map.insert("054", "Phú Yên");
    map.insert("056", "Khánh Hòa");
    map.insert("058", "Ninh Thuận");
    map.insert("060", "Bình Thuận");
    map.insert("062", "Kon Tum");
    map.insert("064", "Gia Lai");
    map.insert("066", "Đắk Lắk");
    map.insert("067", "Đắk Nông");
    map.insert("068", "Lâm Đồng");
    map.insert("070", "Bình Phước");
    map.insert("072", "Tây Ninh");
    map.insert("074", "Bình Dương");
    map.insert("075", "Đồng Nai");
    map.insert("077", "Bà Rịa - Vũng Tàu");
    map.insert("079", "TP. Hồ Chí Minh");
    map.insert("080", "Long An");
    map.insert("082", "Tiền Giang");
    map.insert("083", "Bến Tre");
    map.insert("084", "Trà Vinh");
    map.insert("086", "Vĩnh Long");
    map.insert("087", "Đồng Tháp");
    map.insert("089", "An Giang");
    map.insert("091", "Kiên Giang");
    map.insert("092", "Cần Thơ");
    map.insert("093", "Hậu Giang");
    map.insert("094", "Sóc Trăng");
    map.insert("095", "Bạc Liêu");
    map.insert("096", "Cà Mau");
    map
}

fn decode_cccd(cccd: &str) -> Option<CitizenInfo> {
    if cccd.len() != 12 || !cccd.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("CCCD không hợp lệ: phải đúng 12 chữ số.");
        return None;
    }

    let province_code = &cccd[0..3];
    let gender_code = cccd.chars().nth(3).unwrap();
    let year_code = &cccd[4..6];

    let provinces = province_codes();
    let province_name = provinces.get(province_code).unwrap_or(&"Không xác định");

    let (century, gender) = match gender_code {
        '0' => (1900, "Nam"),
        '1' => (1900, "Nữ"),
        '2' => (2000, "Nam"),
        '3' => (2000, "Nữ"),
        '4' => (2100, "Nam"),
        '5' => (2100, "Nữ"),
        '6' => (2200, "Nam"),
        '7' => (2200, "Nữ"),
        '8' => (2300, "Nam"),
        '9' => (2300, "Nữ"),
        _ => return None,
    };

    let year_last_two: u16 = year_code.parse().ok()?;
    let year_of_birth = century + year_last_two;

    Some(CitizenInfo {
        province: province_name.to_string(),
        gender: gender.to_string(),
        year_of_birth,
    })
}

fn main() {
    let samples = vec!["030198123123", "001098123123"];

    for cccd in samples {
        println!("Phân tích số CCCD: {}", cccd);
        match decode_cccd(cccd) {
            Some(info) => {
                println!("Tỉnh/Thành: {}", info.province);
                println!("Giới tính: {}", info.gender);
                println!("Năm sinh: {}\n", info.year_of_birth);
            }
            None => println!("Không thể phân tích (định dạng sai).\n"),
        }
    }
}
