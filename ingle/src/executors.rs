enum Error {}

// TODO: Wonder if I actually need read vs write executors.
// That's a Q for later
trait ReadExecutor {}

trait WriteExecutor {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestExecutor {}

    impl ReadExecutor for TestExecutor {}

    #[test]
    fn executor_is_object_safe() {
        let _: Box<dyn ReadExecutor> = Box::new(TestExecutor {});
    }
}
