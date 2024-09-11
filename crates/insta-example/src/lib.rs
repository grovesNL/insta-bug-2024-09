pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::add(1u64, 2u64);
        insta::assert_ron_snapshot!(result, @r###"4"###);
    }
}
