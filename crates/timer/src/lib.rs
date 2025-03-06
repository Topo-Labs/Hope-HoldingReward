// Timer: 每日00:00定时执行
// - 将数据库符合要求的数据过滤出来，并通过telegram通知给管理员

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
