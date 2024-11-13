#[tokio::main]
async fn main() {
    // 下载IP数据
    let ip_data = reqwest::get("https://ftp.apnic.net/apnic/stats/apnic/delegated-apnic-latest")
        .await
        .unwrap()
        .text()
        .await
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

    // 写入文件
    let mut output = String::new();
    for (ip, mask) in &ip_data {
        output.push_str(&format!("{}/{}\n", ip, mask));
    }
    std::fs::write("ip_data.txt", output).unwrap();
}
