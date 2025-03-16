// 3. Common Programming Concepts

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    // 3.1.
    // let x = 5;
    // println!("The value of x is : {x}");
    // x = 6;
    // x 는 한번 초기화 된 이후 수정할 수 없다.

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // mut 키워드를 붙여주면 수정할 수 있다.

    println!("Time is : {THREE_HOURS_IN_SECONDS}");
    // const 는 C++ 의 constexpr 과 비슷하게 컴파일 타임에 평가 받는 상수이다.
    // 거기에 static 과 같이 전역적으로 사용될 수 있다.

    // let spaces = "   ";
    // let spaces = spaces.len();
    // Shadowing 으로 해당 구문 사용 가능, string으로써 spaces는 사용 불가

    // let mut spaces = "   ";
    // spaces = spaces.len(); 
    // 변수의 유형은 변경 불가능


    // 3.2.
    // 러스트는 컴파일 타임에 모든 변수의 타입을 알아야 하는 정적 타입 언어
    let guess: u32 = "42".parse().expect("Not a number!");

    let int32: i32 = 12;
    let xf = 2.0; // f64
    let yf: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    println!("sum : {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference : {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product : {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("quotient : {quotient}");
    println!("truncated : {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder : {remainder}");

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tuple of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [1, 2, 3, 4, 5]; 와 같다.

    let first = a[0];
    let second = a[1];
    // 배열 요소 접근하기.

    let aa = [3; 5];
    // let a = [3, 3, 3, 3, 3]; 와 같다.
}
