#[derive(Debug)]
/// enum for memory for easy parsing and organization later
pub enum Memory {
    Text(String)
}

/// enum for references for easy parsing and organization later
#[derive(Debug)]
pub enum Ref {
    Text(String),
    Similar(Vec<Ref>)
}

/// The struct that defines these types and make them usable
pub struct MemoryInstance {
    pub memory: Vec<Memory>,
    pub reference: Vec<Ref>,
    pub similar: Vec<Ref>,
}

/// Impl to create new instance
impl MemoryInstance {
    /// Creates blank vector so you can implement what you want
    pub fn new() -> MemoryInstance {
        let instance: MemoryInstance = MemoryInstance {
            memory: Vec::new(),
            reference: Vec::new(),
            similar: Vec::new(),
        };
        return instance;
    }
}

/// trys looking in memory for string and returns index for all global vecs
pub fn match_memory(arr: &Vec<Memory>, mstate: String) -> usize {
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

/// trys looking in similar for string and returns index for all global vecs
pub fn match_similar(arr: &Vec<Ref>, mstate: String) -> usize {
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