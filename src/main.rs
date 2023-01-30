use rand::Rng;

fn quicksort<T: Ord>(list: &mut [T]) {
    // match list.len() {
    //     1 | 0 => return,
    // }
    if list.len() == 0 || list.len() == 1 {
        return;
    }

    let pivot = rand::thread_rng().gen_range(0..list.len());

    {
        let left = &mut list[..pivot];
        quicksort(left);
    }
    {
        let right = &mut list[pivot..];
        quicksort(right);
    }
}

// fn quicksort(list: &mut [i32]) {
//     let (left, right) = list.split_at(list.len() / 2);
// }

// fn check_cuil(cuil: &str) -> bool {
//     assert!(cuil.len() == 11);

//     let mults = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2, 0];

//     let codigo: i32 = cuil
//         .chars()
//         .zip(mults.iter())
//         .map(|(c, &m)| c.to_digit(10).unwrap_or(0) * m)
//         .collect()
//         .sum() as i32;

//     let codigo = (codigo % 11) as u8;

//     cuil.starts_with("20") && cuil.ends_with(codigo as char)
// }

// fn mult(lhs: &str, rhs: &str) -> String {
//     match (lhs, rhs) {
//         (x:xs, y) => todo!(),
//     }

//     let (a, b) = lhs.split_at(lhs.len() / 2);
//     let (c, d) = rhs.split_at(rhs.len() / 2);

//     let ac = mult(a, c);
//     let bd = mult(b, d);

//     a.to_string()
// }

#[derive(Debug)]
enum _Object {
    _Visible(ScreenObject),
    _Invisible(_DataObject),
}

#[derive(Debug)]
struct Size {
    _x: usize,
    _y: usize,
}

#[derive(Debug)]
struct Coordinate {
    _x: i32,
    _y: i32,
}

// impl Coordinate {
//     fn AddAssign<Coordinate>(&self, rhs: Coordinate) {
//         self._x += rhs._x;
//     }
// }

#[derive(Debug)]
struct _DataObject {
    _data: String,
}

#[derive(Debug)]
struct ScreenObject {
    _dimentions: Size,
    _placement: Coordinate,
    _content: Vec<Vec<char>>,
    _layer: u8,
    _shading: Shading,
    _child: Vec<_Object>,
}

impl ScreenObject {
    fn new(_content: Vec<Vec<char>>, _shading: Shading) -> Self {
        Self {
            _content,
            _shading,
            _dimentions: Size { _x: 1, _y: 2 },
            _placement: Coordinate { _x: 0, _y: 0 },
            _layer: 0,
            _child: vec![],
        }
    }
    fn _move_by(&mut self, amount: Coordinate) {
        self._placement._x += amount._x;
        self._placement._y += amount._y;
    }

    fn _move_to(&mut self, amount: Coordinate) {
        self._placement._x = amount._x;
        self._placement._y = amount._y;
    }

    // borrar y hacer custom para board
    fn _store(self) -> _DataObject {
        _DataObject {
            _data: self._content.concat().iter().collect(),
            // _data: String::from(self._content),
        }
    }
}

#[derive(Debug)]
enum Shading {
    Opaque,
    Transparent,
}

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let _guess: u8 = "42".parse().expect("Not a number!");

    let _left = String::from("3");
    let _right = String::from("4");
    // assert!(mult(&left, &right) == String::from("12"));
    let mut qtest = [2, 3, 17, 9, 1];
    quicksort(&mut qtest);
    // assert!(qtest == [1, 2, 3, 9, 17]);

    let objcont = vec![vec!['a']];
    let mut objtest = ScreenObject::new(objcont.clone(), Shading::Opaque);

    // se queda con objtest :/
    // let _objtest2 = ScreenObject {
    //     _layer: 2,
    //     _shading: Shading::Transparent,
    //     ..objtest
    // };

    let _objtest2 = ScreenObject::new(objcont.clone(), Shading::Transparent);

    objtest._move_to(Coordinate { _x: 2, _y: 4 });

    println!("Object 2 is {:#?}", _objtest2);

    // objtest.layer += 1; // error?

    // Ya existe en STL, ver 6.1
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let _sum = match y {
        Some(val) => x + val,
        None => x,
    };

    let _testest = match objtest {
        ScreenObject {
            _shading: Shading::Opaque,
            ..
        } => Some(objtest),

        ScreenObject {
            _shading: Shading::Transparent,
            ..
        } => None,
    };
}
