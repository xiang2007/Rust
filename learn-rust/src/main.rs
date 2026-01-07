// fn	i32_input(a: i32) {
// 	println!("i32 a: {a}");
// }

// fn i8_input(a: i8) {
// 	println!("i8 a: {a}");
// }

// fn	multi(a: i8, b: i8) -> i8
// {
// 	return (a * b).saturating_add(a *b).saturating_add(a * b);
// }

// fn	fib(n: u32) -> u32
// {
// 	if n < 2
// 	{
// 		return n;
// 	}
// 	else
// 	{
// 		return fib(n - 1) + fib(n - 2);
// 	}
// }

// fn check_size(a: i32)
// {
// 	let size = if a < 20 {"small"} else {"large"};
// }

// fn matching() {
// 	let	x = 100;
// 	match x
// 	{
// 		10 => println!("10"),
// 		100 => println!("100"),
// 		_ => 
// 		{
// 			println!("something else");
// 		}
// 	}
// }

// fn match_return_value() {
// 	let	flag = true;
// 	let	val = match flag
// 	{
// 		true => 1,
// 		false => 0,
// 	};
// 	println!("The value of {flag} is {val}");
// }

// fn while_loop() {
// 	let mut	x = 200;
// 	while x >= 10
// 	{
// 		x = x / 2;
// 	}
// 	dbg!(x);
// }

// fn for_loop() {
// 	for x in 1..5
// 	{
// 		dbg!(x);
// 	}
// 	for elem in [2, 4, 6, 8, 16, 32]
// 	{
// 		println!("value of elem is {elem}");
// 	}
// }

// fn _loop() {
// 	let mut	i = 0;
// 	loop {
// 		i += 1;
// 		dbg!(i);
// 		if i > 100{
// 			break ;
// 		}
// 	}
// }

// fn break_and_continue() {
// 	let mut i = 0;
// 	loop{
// 		i += 1;
// 		if i > 100{
// 			break ;
// 		}
// 		if i % 2 == 0{
// 			continue ;
// 		}
// 		dbg!(i);
// 	}
// }

// fn labels() {
// 	let s = [[5, 6, 7], [8, 9, 10], [21, 55, 32]];
// 	let mut elementsearch = 0;
// 	let target = 10;
// 	'outer: for i in 0..=2{
// 		for j in 0..=2{
// 			elementsearch += 1;
// 			if s[i][j] == target{
// 				break 'outer;
// 			}
// 		}
// 	}
// 	dbg!(elementsearch);
// }

// fn gcd(a: u32, b: u32) -> u32 {
// 	if b > 0 { gcd(b, a % b) } else { a }
// }

// fn factorial(n: u32) -> u32 {
// 	let mut product = 1;
// 	for i in 1..=n{
// 		product *= i;
	
// 	}
// 	product
// }

// fn fizbuzz(n: u32) -> u32{
// 	todo!()
// }

// fn collatz_length(mut n: i32) -> u32 {
// 	let mut i = 1;
// 	while n > 1{
// 		if n % 2 == 0{
// 			n = n / 2;
// 		}
// 		else{
// 			n = 3 * n + 1;
// 		}
// 		dbg!(n);
// 		i += 1;
// 	}
// 	return i
// }

// fn collatz_length(mut n: i32) -> u32 {
// 	let mut len = 1;
// 	while n > 1{
// 		n = if n % 2 == 0 {n / 2} else {3 * n + 1};
// 		len += 1;
// 	}
// 	return len;
// }

// fn get_index() -> usize {
// 	6
// }

// fn array() {
// 	let mut n: [i8; 5] = [5, 4, 3, 2 ,1];
// 	n[get_index()] = 0; // panicked
// 	println!("n: {n:#?}");
// }

// fn tuple() {
// 	let t: (i8, bool) = (1, true);
// 	dbg!(t.0);
// 	dbg!(t.1);
// }

// fn array_iter() {
// 	let primes = [2, 3, 5, 7, 11, 13, 17, 19];
// 	for prime in primes{
// 		for i in 2..prime {
// 			dbg!(i);
// 			assert_ne!(prime % i, 0);
// 		}
// 	}
// }

// fn check_order(tuple: (i32, i32, i32)) -> bool {
// 	let (left, middle, right) = tuple;
// 	left < middle && middle < right
// }

// fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {

// 	let mut result = [[0; 3]; 3];
// 	for i in 0..3{
// 		for j in 0..3 {
// 			result[j][i] = matrix[i][j];
// 		}
// 	}
// 	result
// }

// fn borrowing() {
// 	let a = 'A';
// 	let b = 'B';

// 	let mut r: &char = &a;
// 	dbg!(r);

// 	r = &b;
// 	dbg!(r);
// }

// fn exclusive() {
// 	let mut point = (1, 2);
// 	let x_coord = &mut point.0;
// 	*x_coord = 20;
// 	println!("point: {point:?}");
// }

// fn slices() {
// 	let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
// 	println!("a: {a:?}");

// 	let s: &[i32] = &a[0..4];
// 	println!("s: {s:?}");
// }

// fn strings() {
// 	let s1: &str = "World";
// 	println!("s1: {s1}");

// 	let mut s2: String = String::from("Hello ");
// 	println!("s2: {s2}");

// 	s2.push_str(s1);
// 	println!("s2: {s2}");

// 	let s3 = &s2[..];
// 	println!("s3: {s3}");
// }

// fn byte_strings() {
// 	println!("{:?}", b"abc");
// 	println!("{:?}", &[97, 98, 99]);
// }

fn main()
{
	
}
