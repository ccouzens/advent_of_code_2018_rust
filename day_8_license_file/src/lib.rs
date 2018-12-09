#[macro_use]
extern crate nom;

pub fn metadata_sum(input: &str) -> Option<u32> {
    if let Ok((_rest, node)) = Node::parse(&to_byte_array(input)) {
        Some(node.metadata_sum())
    } else {
        None
    }
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    named!(
        parse<&[u8], Self>,
        do_parse!(
            child_count: map_opt!(take!(1), to_num::<usize>)
            >> metadata_count: map_opt!(take!(1), to_num::<usize>)
            >> children: count!( Self::parse, child_count )
            >> metadata: count!(
                map_opt!(take!(1), to_num::<u32>),
                metadata_count
            )
            >> (Self { children, metadata })
        )
    );

    fn metadata_sum(&self) -> u32 {
        self.metadata.iter().cloned().sum::<u32>()
            + self.children.iter().map(Self::metadata_sum).sum::<u32>()
    }
}

fn to_byte_array(input: &str) -> Vec<u8> {
    input
        .split_whitespace()
        .flat_map(|v| u8::from_str_radix(v, 10))
        .collect()
}

fn to_num<T>(v: &[u8]) -> Option<T>
where
    T: From<u8>,
{
    v.get(0).map(|v| T::from(*v))
}

#[cfg(test)]
mod metadata_sum_test {
    use metadata_sum;

    #[test]
    fn worked_example() {
        assert_eq!(metadata_sum(include_str!("../example.txt")), Some(138));
    }

    #[test]
    fn puzzle() {
        assert_eq!(metadata_sum(include_str!("../input.txt")), Some(41028));
    }

}
