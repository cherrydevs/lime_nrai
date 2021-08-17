// cargo wont stop complaining
mod delta;

#[cfg(test)]
mod tests {
    use crate::delta::*;
    #[test]
    fn test() {
        let mut instance = Instance::new();
        instance.memory.push(Memory::Text(String::from("lime")));
        instance.reference.push(Ref::Text(String::from("a green bitter fruit")));
        instance.similar.push(Ref::Similar(vec![Ref::Text(String::from("lemon"))]));


        // get the lime reference
        let index = match_memory(&instance.memory, String::from("lime"));
        println!("object: {0:?}, description: {1:?}, similar to: {2:?}", instance.memory[index], instance.reference[index], instance.similar[index]);

        let indexsim = match_similar(&instance.similar, String::from("lemon"));
        // print out things that match with lemon in similar state
        println!("what it matched: {0:?},matching string: {1:?}", instance.memory[indexsim], instance.similar[indexsim]);
    }
}


