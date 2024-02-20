use std::fmt;

struct Structure(i32, i64, u16, u32);
struct Range(i64, i64);
struct List(Vec<i32>);
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color {
            red,
            green,
            blue,
        } = self;
        write!(f, "RGB ({red}, {green}, {blue}) 0x{red:X}{green:X}{blue:X}")
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lat >= 0.0 { 'N' } else { 'S' };

        let name = self.name;
        let lat = self.lat.abs();
        let lon = self.lon.abs();
        write!(f, "{name}: {lat}°{lat_c}, {lon}°{lon_c}")
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[").unwrap();
        for (i, x) in self.0.iter().enumerate() {
            if i != 0 { write!(f, ", ")?; };
            write!(f, "{}: {}", i, x)?;
        }
        write!(f, "]")
    }
}

impl fmt::LowerHex for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:x}, {:x})", self.0, self.1)
    }
}

impl fmt::Binary for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:b}, {:b})", self.0, self.1)
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {} {})", self.0, self.1, self.2, self.3)
    }
}

pub fn test() {
    println!("formatting:");
    let s = Structure(-15, -1235, 2, 92);
    let r = Range(15, 28);
    let l = List(vec![2, 4, 8, 64, 16, 32, 64]);
    let c = City { name: "city", lat: -54.534, lon: 23.2342 };
    let col = Color {
        red: 128,
        green: 255,
        blue: 90,
    };

    println!("Structure:        {s}");
    println!("Range in hex:     {r:x}");
    println!("Range in binary:  {r:b}");
    println!("List:             {l}");
    println!("City:             {c}");
    println!("Color:            {col}");

    println!("{:-<1$}", "", 40);
}