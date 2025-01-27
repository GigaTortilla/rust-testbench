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

#[derive(Debug)]
struct Vector3D {
    x: f32,
    y: f32,
    z: f32
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

// Every format has to be implemented individually
impl fmt::LowerHex for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:x}, {:x})", self.0, self.1)
    }
}

impl fmt::UpperHex for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:X}, {:X})", self.0, self.1)
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

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
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
    let vec_example: Vector3D = Vector3D { x: 1.0f32, y: 2.0f32, z: -4.5f32 };

    // Tuples can contain multiple different types
    let tup = (1.5f32, true, 'a', 12u8, "Hello");
    let (v, w, x, y, z) = tup;
    
    println!("Structure:          {s}");
    println!("Range in hex:       {r:x}");
    println!("Range in cap hex:   {r:X}");
    println!("Range in binary:    {r:b}");
    println!("List:               {l}");
    println!("City:               {c}");
    println!("Color:              {col}");
    println!("Vector3D:           {vec_example}");
    println!("Tuple:              {tup:?}");
    println!("Tuple entries:      {v} {w} {x} {y} {z}");

    print_end!();
}