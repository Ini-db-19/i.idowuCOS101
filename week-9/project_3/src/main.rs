use std::io::Write;

fn main() {

    let head = "\nFULL NAME                     MINISTRY        GEOPOLITICAL ZONE";
    let daudu = ["Aigbogun Alamba Daudu","Internal Affairs","South West"];
    let afeez = ["Murtala Afeez Bendu","Justice","North East"];
    let ogbona = ["Okorocha Calistus Ogbona","Defence","South South"];
    let akanbi = ["Adewale Jimoh Akanbi","Power & Steel","South West"];
    let faith = ["Osazuwa Faith Etieye","Petroleum","South East"];

    println!("{}", head);
    println!("\n{}      {}          {}", daudu[0], daudu[1], daudu[2]);
    println!("{}           {}                {}", afeez[0], afeez[1], afeez[2]);
    println!("{}      {}               {}", ogbona[0], ogbona[1], ogbona[2]);
    println!("{}        {}            {}", akanbi[0], akanbi[1], akanbi[2]);
    println!("{}         {}               {}", faith[0], faith[1], faith[2]);

    let line = format!("\n{}      {}          {}", daudu[0], daudu[1], daudu[2]);
    let line1 = format!("\n{}           {}                {}", afeez[0], afeez[1], afeez[2]);
    let line2 = format!("\n{}      {}               {}", ogbona[0], ogbona[1], ogbona[2]);
    let line3 = format!("\n{}        {}            {}", akanbi[0], akanbi[1], akanbi[2]);
    let line4 = format!("\n{}         {}               {}", faith[0], faith[1], faith[2]);

    let mut file = std::fs::File::create("Convicted Minister Records.txt").expect("Create failed");
    file.write_all(head.as_bytes()).expect("Write failed");
    file.write_all(line.as_bytes()).expect("Write failed");
    file.write_all(line1.as_bytes()).expect("Write failed");
    file.write_all(line2.as_bytes()).expect("Write failed");
    file.write_all(line3.as_bytes()).expect("Write failed");
    file.write_all(line4.as_bytes()).expect("Write failed");

    println!("\nFile creation successful.");
}
