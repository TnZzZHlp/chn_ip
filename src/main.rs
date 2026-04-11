fn main() {
    // 下载IP数据
    let ip_data =
        reqwest::blocking::get("https://ftp.apnic.net/apnic/stats/apnic/delegated-apnic-latest")
            .unwrap()
            .text()
            .unwrap();
    let geo_as_ip_data =
        reqwest::blocking::get(
            "https://raw.githubusercontent.com/DH-Teams/DH-Geo_AS_IP_CN/refs/heads/main/Geo_AS_IP_CN_All.txt",
        )
        .unwrap()
        .text()
        .unwrap();

    // 解析IP数据
    let ip_data = ip_data
        .lines()
        .filter(|line| {
            line.starts_with("apnic|CN|ipv4|") && line.ends_with("allocated")
                || line.starts_with("apnic|CN|ipv6|") && line.ends_with("allocated")
        })
        .map(|line| {
            if line.starts_with("apnic|CN|ipv4|") {
                let mut parts = line.split('|');
                let _ = parts.next();
                let _ = parts.next();
                let _ = parts.next();
                let ip = parts.next().unwrap();
                let mask = 32 - parts.next().unwrap().parse::<u32>().unwrap().ilog2();
                (ip, mask)
            } else {
                let mut parts = line.split('|');
                let _ = parts.next();
                let _ = parts.next();
                let _ = parts.next();
                let ip = parts.next().unwrap();
                let mask = parts.next().unwrap().parse::<u32>().unwrap();
                (ip, mask)
            }
        })
        .collect::<Vec<_>>();

    // 合并并去重
    let mut merged_ip_data = std::collections::BTreeSet::new();
    for (ip, mask) in &ip_data {
        merged_ip_data.insert(format!("{}/{}", ip, mask));
    }
    for line in geo_as_ip_data.lines() {
        let cidr = line.trim();
        if !cidr.is_empty() {
            merged_ip_data.insert(cidr.to_string());
        }
    }

    // 写入文件
    let mut output = String::new();
    for cidr in merged_ip_data {
        output.push_str(&format!("{}\n", cidr));
    }
    std::fs::write("ip_data.txt", output).unwrap();
}
