enum Stat { Push, Pop, Ret, Func(usize), Native(fn(&mut Vec<bool>)), Get(usize) }

fn run(code: &str, memory:&mut Vec<bool>) {
	let mut functions = std::collections::HashMap::from([ //func definitions
		("nand", Err( ( |x: &mut Vec<bool>| { //nand: !(A & B)
			let (a, b) = (x.pop().expect("pop fail"), x.pop().expect("pop fail"));
			x.push(!(a && b))
		} ) as _ ) ),
		("T", Err( ( |x: &mut Vec<bool>| { x.push(true) } ) as _ ) ), //push T
	]);
	let (mut call_tail, mut stack, mut jmp) = (vec![], vec![], usize::MAX - 1);
	let statements = code.split_whitespace().enumerate().map(|(i, word)| {
			let func = functions.get(&word);
			if let Ok(n) = word.parse() { Stat::Get(n) }
			else if word=="+" { Stat::Push } else if word=="-" { Stat::Pop }
			else if let Some(&Err(func)) = func { Stat::Native(func) }
			else if let Some(&Ok(func)) = func { Stat::Func(func) }
			else { functions.insert(word, Ok(i)); jmp = i; Stat::Ret }
	} ).collect::<Vec<Stat>>(); //separate the code in statements
	while {jmp += 1; jmp}<statements.len() { match &statements[jmp] { //run
		&Stat::Native(func) => { func(memory) }
		&Stat::Func(func) => { call_tail.push(jmp); jmp = func }
		&Stat::Get(n) => { memory.push(stack[stack.len().checked_sub(n+1).expect("get fail")]) }
		&Stat::Push => { stack.push(memory.pop().expect("push fail")) }
		&Stat::Pop => { stack.pop().expect("pop fail"); }
		&Stat::Ret => { jmp = call_tail.pop().expect("ret fail") }
	} }
}

fn main() {
	let code = "
F T T nand
not + 0 0 nand -
or + + 0 not 1 not nand - -
and nand not
xor + + 0 1 or 0 1 and not and - -
nor or not
xnor xor not
+1️⃣ + + + + + + + +
0 1 2 3 4 5 6 7 and and and and and and xor
1 2 3 4 5 6 7 and and and and and xor
2 3 4 5 6 7 and and and and xor
3 4 5 6 7 and and and xor
4 5 6 7 and and xor
5 6 7 and xor
6 7 xor
7 not
- - - - - - - -
main +1️⃣ +1️⃣ +1️⃣
"; //this program sum by 3 a 8 bits digit
	let mut args = vec![false, false, false, false, false, true, true, false];
	run(code, &mut args);
	println!("{:?}", args);
}