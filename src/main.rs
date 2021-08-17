#[derive(Debug)]
enum Memory {
    Text(String)
}

#[derive(Debug)]
enum Ref {
    Text(String),
    Similar(Vec<Ref>)
}

fn main() {
    let mut memory: Vec<Memory> = Vec::new();
    let mut reference: Vec<Ref> = Vec::new();
    let mut similar: Vec<Ref> = Vec::new();

    // create the lime variables
    memory.push(Memory::Text(String::from("lime")));
    reference.push(Ref::Text(String::from("a green bitter fruit")));
    similar.push(Ref::Similar(vec![Ref::Text(String::from("lemon"))]));


    // get the lime reference
    let index = match_memory(&memory, String::from("lime"));
    println!("object: {0:?}, description: {1:?}, similar to: {2:?}", memory[index], reference[index], similar[index]);

    let indexsim = match_similar(&similar, String::from("lemon"));
    // print out things that match with lemon in similar state
    println!("what it matched: {0:?},matching string: {1:?}", memory[indexsim], similar[indexsim]);
}

fn match_memory(arr: &Vec<Memory>, mstate: String) -> usize {
    let mut y: usize = 0;
    let mut i: usize = 1;
    for x in arr {
        if i == 1 {

        } else {
            y = y + 1;
        }
        i = i + 1;
        match x {
            Memory::Text(mstatecomp) if mstatecomp.to_string() == mstate => return y,
            _ => continue
        }

    }
    return arr.len() + 1;
}

fn match_similar(arr: &Vec<Ref>, mstate: String) -> usize {
    let mut y: usize = 0;
    let mut i: usize = 0;
    for x in arr {
        if let Ref::Similar(j) = x {
            for x in j {
                i = i + 1;
                if i == 1 {
                    y = 0;
                } else {
                    y += 1;
                }
                match x {
                    Ref::Text(mstatecomp) if mstatecomp.to_string() == mstate => return y,
                    _ => continue
                }
            }
        }
    }
    return arr.len() + 1;
}
