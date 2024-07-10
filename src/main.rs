use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::fs::File;
use csv::Writer;
use regex::Regex;


struct Cli {
    ipaddres: String,
    path: String,
}



fn main() {
    // IP адрес для команды crackmapexec
    let ip_addr = std::env::args().nth(1).expect("no ip given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        ipaddres: ip_addr,
        path: path,
    };

    // Выполнение команды и захват её вывода
    let output = Command::new("crackmapexec")
        .arg("smb")
        .arg(args.ipaddres)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    // Захват stdout команды
    let stdout = output.stdout.expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    // Регулярное выражение для извлечения IP адреса и hostname
    let re = Regex::new(r"(\d+\.\d+\.\d+\.\d+)(\s+)(\d\d\d)(\s+)([^\s]+)").unwrap();

    // Открываем CSV файл для записи
    let file = File::create(args.path).expect("Failed to create file");
    let mut wtr = Writer::from_writer(file);

    // Пишем заголовок таблицы
    wtr.write_record(&["IP Address", "Hostname"]).expect("Failed to write header");
    let mut count_smb1 = 0;
    // Фильтрация вывода по строке "SMBv1:True" и запись в CSV файл
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.contains("SMBv1:True") {
                    

                    if let Some(cap) = re.captures(&line) {
                        let adress_host = &cap[1];
                        //println!("Found {}", &cap[1]);
                        //println!("Found {}", &cap[5]);
                        let hostname = &cap[5];
                        wtr.write_record(&[adress_host, hostname]).expect("Failed to write record");
                        count_smb1 += 1;
                    }
                    
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    // Завершаем запись в CSV файл
    wtr.flush().expect("Failed to flush writer");
    println!("Отчет составлен!");
    println!("Всего машин использующих SmbV1: {count_smb1}");
}
