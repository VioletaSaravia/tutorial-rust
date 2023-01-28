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

struct Coordinate(i32, i32);

struct ScreenObject {
    size: (usize, usize),
    placement: Coordinate,
    content: Vec<Vec<char>>,
    layer: u8,
}

fn build_screenobject(content: Vec<Vec<char>>) -> ScreenObject {
    ScreenObject {
        size: (content.len(), content[0].len()),
        placement: Coordinate(0, 0),
        content,
        layer: 0,
    }
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
    assert!(qtest == [1, 2, 3, 9, 17]);

    let objcont: Vec<Vec<char>> = [['a']];
    let _objtest = build_screenobject(objcont);

    let _objtest2 = ScreenObject {
        layer: 2,
        .._objtest
    };

    // _objtest.layer += 1; // error?
}
