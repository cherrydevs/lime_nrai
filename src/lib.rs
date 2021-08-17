// cargo wont stop complaining
mod delta;

#[cfg(test)]
mod tests {
    use crate::delta::*;
    use futures::executor::block_on;
    #[test]
    fn test() {
        let mut instance = Instance::new();
        instance.memory.push(Memory::Text(String::from("lime")));
        instance.reference.push(Ref::Text(String::from("a green bitter fruit")));
        instance.similar.push(Ref::Similar(vec![Ref::Text(String::from("lemon"))]));
        for x in 0..4 {
            // they all have the same weight, weight translation coming soon
            let node = neraul_node(&instance, String::from("lemon"));
            block_on(node);
        }
    }

    async fn neraul_node(instance: &Instance, matcher: String) {
        // get the lime reference

        let indexsim = match_similar(&instance.similar, String::from(matcher));

        // print out things that match with lemon in similar state
        println!("what it matched: {0:?},matching string: {1:?}", instance.memory[indexsim], instance.similar[indexsim]);
    }
}


