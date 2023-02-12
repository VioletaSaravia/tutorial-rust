use std::borrow::Borrow;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, Read};
use std::rc::Rc;

// pub struct BNode {
//     left: Option<Box<BNode>>,
//     right: Option<Box<BNode>>,
//     parent: Box<BNode>,
// }

struct LNode<T> {
    val: T,
    next: Option<Box<LNode<T>>>,
}

impl<T> LNode<T> {
    fn new(val: T, next: Option<Box<LNode<T>>>) -> LNode<T> {
        LNode { val, next }
    }
}

pub struct LList<T> {
    root: Option<Box<LNode<T>>>,
    size: usize,
}

impl<T> LList<T> {
    pub fn new() -> LList<T> {
        LList {
            root: None,
            size: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        let new = Some(Box::new(LNode::new(val, self.root.take())));

        self.root = new;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Result<T, Box<dyn Error>> {
        match self.root.take() {
            Some(x) => {
                let result = x.val;
                self.root = x.next;
                self.size -= 1;
                Ok(result)
            }
            None => Err("Lista vacía".into()),
        }
    }
}

type Link<T> = Option<Rc<RefCell<DNode<T>>>>;

struct DNode<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> DNode<T> {
    fn new_front(val: T, next: Link<T>) -> DNode<T> {
        DNode {
            val,
            next,
            prev: None,
        }
    }

    fn new_back(val: T, prev: Link<T>) -> DNode<T> {
        DNode {
            val,
            next: None,
            prev,
        }
    }
}

pub struct DList<T> {
    root: Link<T>,
    end: Link<T>,
    size: usize,
}

impl<T> DList<T> {
    pub fn new() -> DList<T> {
        DList {
            root: None,
            end: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, val: T) {
        {
            let new: Link<T> = Some(Rc::new(RefCell::new(DNode::new_front(
                val,
                self.root.take(),
            ))));
            self.root = new;
            self.size += 1;
        }
        // TODO asignar prev a next (y next a prev en push_back())
        // let asd = &mut self.root.as_mut().unwrap().borrow_mut().next;
        // let asd = asd.as_mut().unwrap().borrow_mut().prev.as_mut().unwrap();
        // asd = self.root.borrow();
    }

    pub fn pop_front(&mut self) {
        self.root = match self.root.take() {
            Some(x) => x.borrow_mut().next.take(),
            None => return,
        };
        self.size -= 1;
    }

    pub fn push_back(&mut self, val: T) {
        let new: Link<T> = Some(Rc::new(RefCell::new(DNode::new_back(val, self.end.take()))));

        self.end = new;
        self.size += 1;
    }

    pub fn pop_back(&mut self) {
        self.end = match self.end.take() {
            Some(x) => x.borrow_mut().next.take(),
            None => return,
        };
        self.size -= 1;
    }
}

pub struct Guess {
    value: i32,
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
