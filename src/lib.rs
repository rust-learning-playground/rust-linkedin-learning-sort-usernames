pub fn sort_usernames<T: AsRef<str> + Ord>(l: &mut Vec<T>) {
    l.sort_by_cached_key(|a| a.as_ref().to_lowercase());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insensitive_order_length() {
        let mut usernames = vec!["Juan", "abraham"];
        sort_usernames(&mut usernames);
        assert_eq!(usernames.len(), 2);
    }

    #[test]
    fn insensitive_order() {
        let mut usernames = vec!["Juan", "abraham", "pablo", "1"];
        let ordered_usernames = vec!["1", "abraham", "Juan", "pablo"];
        sort_usernames(&mut usernames);
        assert_eq!(usernames, ordered_usernames);
    }
}
