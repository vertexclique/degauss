/// Represents the exit status of a command
pub enum Status {
    Success,
    Failure,
}

impl Status {
    /// Convert a given status enum to its i32 representation
    /// Success = 0
    /// Failure = -1
    pub fn to_i32(&self) -> i32 {
        match self {
            Status::Failure => -1,
            Status::Success => 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_status() {
        let status = Status::Failure;
        assert!(status.to_i32() == -1);

        let status = Status::Success;
        assert!(status.to_i32() == 0);
    }
}
