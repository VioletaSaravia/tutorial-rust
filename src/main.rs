use rand::Rng;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, Read};

pub struct Guess {
    value: i32,
    _test: String,
}

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    nums.iter()
        .zip(nums.iter().skip(n as usize))
        .clone()
        .for_each(|(x, y)| {
            result.push(*x);
            result.push(*y);
        });

    result
}

pub fn testtest<T>(list: &[T], cost: &[T], start: usize) {
    let mut test = list
        .iter()
        .peekable()
        .skip(start)
        .cycle()
        .zip(cost.iter().peekable().skip(start).cycle())
        .enumerate();

    for _ in 0..list.len() {
        test.next();
    }

    return;
}

// clásico check en el constructor
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn get(&self) -> i32 {
        self.value
    }

    pub fn set(&mut self, new: i32) -> Result<(), Box<dyn Error>> {
        if new < 1 || new > 100 {
            return Err("Valor incorrecto".into());
        }

        self.value = new;

        Ok(())
    }
}

// equivalente a fs::read_to_string("hello.txt")
fn _read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    // ? sólo se puede usar en Result, Option, u otro tipo que implemente FromResidual
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let mut result = 0;
    let mut dividend = dividend;
    let mut divisor = divisor;
    let mut neg = (false, false);
    if divisor < 0 {
        neg.0 = true;
        divisor = divisor.abs();
    }
    if dividend < 0 {
        neg.1 = true;
        dividend = dividend.abs();
    }
    loop {
        if dividend < divisor {
            match neg {
                (s, p) if s == p => break result,
                _ => break result * -1,
            }
        }
        dividend -= divisor;
        result += 1;
    }
}

pub fn reverse(x: i32) -> i32 {
    let x = x.to_string();

    let x = match x.starts_with('-') {
        true => [
            "-",
            &(x.chars().filter(|x| *x != '-').collect::<String>())
                .chars()
                .rev()
                .collect::<String>(),
        ]
        .join(""),
        false => x.chars().rev().collect::<String>(),
    };

    x.parse::<i32>().unwrap_or(0)
}

pub fn is_palindrome(x: i32) -> bool {
    let x = x.to_string();

    x.chars()
        .zip(x.chars().rev())
        .map(|(begin, end)| begin == end)
        .all(|x| x)
}

fn quicksort<T: std::cmp::PartialOrd>(list: &mut [T]) {
    let pivot = match list.len() {
        2 if &list[0] > &list[1] => {
            list.swap(0, 1);
            return;
        }
        2 | 1 | 0 => return,
        len => rand::thread_rng().gen_range(0..len),
    };

    list.swap(0, pivot);

    let mut i = 1;
    for j in 1..list.len() {
        if &list[j] < &list[pivot] {
            list.swap(j, i);
            i += 1;
        }
    }

    list.swap(i - i, pivot);

    {
        let left = &mut list[..pivot];
        quicksort(left);
    }

    {
        let right = &mut list[pivot..];
        quicksort(right);
    }
}
fn _check_cuil(cuil: &str) -> Result<(), Box<dyn Error>> {
    if cuil.len() != 11 {
        return Err("Cuil debe tener 11 números".into());
    };

    let mults = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2, 0];

    let codigo = cuil
        .chars()
        .zip(mults.iter())
        .filter_map(|(n, m)| Some(n.to_digit(10)? * m))
        .sum();

    match codigo {
        0 if cuil.ends_with('0') => Ok(()),
        1 if (cuil.ends_with('9') || cuil.ends_with('4')) && cuil.starts_with("23") => Ok(()),
        x if cuil.ends_with(char::from_digit(11 - x, 10).unwrap()) => Ok(()),
        _ => Err("Cuil inválido".into()),
    }
}

fn _is_valid_cbu(cbu: &str) -> Result<(), Box<dyn Error>> {
    if cbu.len() != 22 {
        return Err("CBU debe tener 22 números".into());
    }

    let mult1 = vec![7, 1, 3, 9, 7, 1, 3];
    let mult2 = vec![3, 9, 7, 1, 3, 9, 7, 1, 3, 9, 7, 1, 3];

    let cod1 = &cbu[0..7];
    let cod2 = &cbu[8..21];

    let mut ver = vec!['_'; 2];

    for (n, (i, j)) in [(mult1, cod1), (mult2, cod2)].iter().enumerate() {
        ver[n] = match char::from_u32(
            10 - j
                .chars()
                .zip(i.iter())
                .filter_map(|(c, m)| Some(c.to_digit(10)? * m))
                .sum::<u32>()
                % 10,
        ) {
            Some(x) => x,
            None => return Err("CBU Inválido".into()),
        };
    }

    if cbu.chars().nth(8).unwrap() == ver[0] && cbu.chars().nth(22).unwrap() == ver[1] {
        Ok(())
    } else {
        Err("CBU Inválido".into())
    }
}

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

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn _some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    5
}

// Simplificacion del metodo que está en quicksort<T>
// Solo para casos sencillos
pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let _x = divide(-2147483648, -1);
    // let _x = reverse(123);

    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // let _guess: u8 = "42".parse().expect("Not a number!");

    // let _left = String::from("3");
    // let _right = String::from("4");
    // // assert!(mult(&left, &right) == String::from("12"));
    // let mut qtest = vec![2, 3, 17, 9, 1];
    // quicksort(&mut qtest);
    // // assert!(qtest == [1, 2, 3, 9, 17]);

    // let objcont = vec![vec!['a']];
    // let _none_opt = &objcont.get(999);
    // let mut objtest = ScreenObject::new(objcont.clone(), Shading::Opaque);

    // // se queda con objtest :/
    // // let _objtest2 = ScreenObject {
    // //     _layer: 2,
    // //     _shading: Shading::Transparent,
    // //     ..objtest
    // // };

    // let _objtest2 = ScreenObject::new(objcont.clone(), Shading::Transparent);

    // objtest._move_to(Coordinate { _x: 2, _y: 4 });

    // println!("Object 2 is {:#?}", _objtest2);

    // // objtest.layer += 1; // error?

    // // Ya existe en STL, ver 6.1
    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }
    // let _home = IpAddr::V4(127, 0, 0, 1);
    // let _loopback = IpAddr::V6(String::from("::1"));

    // let _some_number = Some(5);
    // let _some_char = Some('e');
    // let _some_vec_of_some_char: Option<Vec<Option<char>>> =
    //     Some(vec![None, None, Some('a'), None, None, Some('e')]);

    // let _absent_number: Option<i32> = None;

    // let x: i8 = 7;
    // let y: Option<i8> = Some(5);

    // let _sum_rara = match y {
    //     Some(7) => -7,
    //     Some(2) | Some(1) => -7,
    //     Some(val) => x + val,
    //     None => x,
    // };

    // let _testest = match objtest {
    //     ScreenObject {
    //         _shading: Shading::Opaque,
    //         ..
    //     } => Some(objtest),

    //     ScreenObject {
    //         _shading: Shading::Transparent,
    //         ..
    //     } => None,
    // };

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let _row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // let mut _testest = vec![2, 3, 17, 4];

    // let _x = String::from("XIV");

    // let mut _x = _x.chars().rev().peekable();

    // match &_testest[..] {
    //     [2, .., mut _x] => _x = 2,
    //     _ => println!("OTROS"),
    // };

    // assert_eq!(_testest[_testest.len()], 2);

    // let _asd: &str = "asd asd \n lakjsdasd";
    // // main() no puede usar '?' porque tiene return Result, no Option
    // // let _asd = _asd.lines().next()?.chars().last();

    // // este '?' sí se puede usar
    // let _greeting_file = File::open("hello.txt")?;

    // let _test = String::from("hello bye");

    // _is_valid_cbu("2232")?;
    // _is_valid_cbu("2232123412341234987634")?;
    // _is_valid_cbu("0140123903710551407490")?;

    Ok(())
}
