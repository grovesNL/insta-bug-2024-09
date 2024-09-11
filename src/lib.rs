
#[cfg(test)]
mod tests {
    #[test]
    fn root_of_workspace() {
        let result = "a";
        insta::assert_ron_snapshot!(result, @r#""b""#);
    }
}
