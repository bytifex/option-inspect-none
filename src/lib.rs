pub trait OptionInspectNone<T> {
    fn inspect_none(self, inspector_function: impl FnOnce()) -> Self;
}

impl<T> OptionInspectNone<T> for Option<T> {
    fn inspect_none(self, inspector_function: impl FnOnce()) -> Self {
        match &self {
            Some(_) => (),
            None => inspector_function(),
        }
        self
    }
}

impl<T> OptionInspectNone<T> for &Option<T> {
    fn inspect_none(self, inspector_function: impl FnOnce()) -> Self {
        match &self {
            Some(_) => (),
            None => inspector_function(),
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::OptionInspectNone;

    #[test]
    fn inspect_none_on_some() {
        let mut inspector_function_called = false;
        Some(()).inspect_none(|| inspector_function_called = true);
        assert!(!inspector_function_called);
    }

    #[test]
    fn inspect_none_on_none() {
        let mut inspector_function_called = false;
        None::<()>.inspect_none(|| inspector_function_called = true);
        assert!(inspector_function_called);
    }
}
