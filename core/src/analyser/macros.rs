/// Constructs a type fact.
#[macro_export]
macro_rules! typefact {
    (_) => {
        $crate::analyser::types::TypeFact::default()
    };
    ($arg:expr) => {{
        let fact: $crate::analyser::types::TypeFact =
            $crate::analyser::types::GenericFact::Only($arg);
        fact
    }};
}

/// Constructs a shape fact.
#[macro_export]
macro_rules! shapefact {
    () =>
        ($crate::analyser::types::ShapeFact::closed(tvec![]));
    (..) =>
        ($crate::analyser::types::ShapeFact::open(tvec![]));
    ($($arg:tt),+; ..) =>
        ($crate::analyser::types::ShapeFact::open(tvec![$(dimfact!($arg)),+]));
    ($($arg:tt),+) =>
        ($crate::analyser::types::ShapeFact::closed(tvec![$(dimfact!($arg)),+]));
}

/// Constructs a dimension fact.
#[macro_export]
macro_rules! dimfact {
    (_) => {
        $crate::analyser::types::DimFact::default()
    };
    (S) => {
        $crate::analyser::types::GenericFact::Only(TDim::s())
    };
    ($arg:expr) => {
        $crate::analyser::types::GenericFact::Only($arg.to_dim())
    };
}

/// Constructs an value fact.
#[macro_export]
macro_rules! valuefact {
    (_) => {
        $crate::analyser::types::ValueFact::default()
    };
    ($arg:expr) => {{
        let fact: $crate::analyser::types::ValueFact =
            $crate::analyser::types::GenericFact::Only($arg);
        fact
    }};
}

/// Tries to unwrap an option, or returns Ok(None) otherwise.
#[macro_export]
macro_rules! unwrap_or_none {
    ($e:expr) => {{
        let e = $e;
        if e.is_none() {
            return Ok(None);
        } else {
            e.unwrap()
        }
    }};
}

#[cfg(tests)]
mod tests {
    #[test]
    fn shape_macro_closed_1() {
        assert_eq!(shapefact![], ShapeFact::closed(tvec![]));
    }

    #[test]
    fn shape_macro_closed_2() {
        assert_eq!(
            shapefact![1],
            ShapeFact::closed(tvec![GenericFact::Only(1)])
        );
    }

    #[test]
    fn shape_macro_closed_3() {
        assert_eq!(
            shapefact![(1 + 1)],
            ShapeFact::closed(vec![GenericFact::Only(2)])
        );
    }

    #[test]
    fn shape_macro_closed_4() {
        assert_eq!(
            shapefact![_, 2],
            ShapeFact::closed(vec![GenericFact::Any, GenericFact::Only(2)])
        );
    }

    #[test]
    fn shape_macro_closed_5() {
        assert_eq!(
            shapefact![(1 + 1), _, 2],
            ShapeFact::closed(vec![
                GenericFact::Only(2),
                GenericFact::Any,
                GenericFact::Only(2),
            ])
        );
    }

    #[test]
    fn shape_macro_open_1() {
        assert_eq!(shapefact![..], ShapeFact::open(tvec![]));
    }

    #[test]
    fn shape_macro_open_2() {
        assert_eq!(
            shapefact![1; ..],
            ShapeFact::open(vec![GenericFact::Only(1)])
        );
    }

    #[test]
    fn shape_macro_open_3() {
        assert_eq!(
            shapefact![(1 + 1); ..],
            ShapeFact::open(vec![GenericFact::Only(2)])
        );
    }

    #[test]
    fn shape_macro_open_4() {
        assert_eq!(
            shapefact![_, 2; ..],
            ShapeFact::open(vec![GenericFact::Any, GenericFact::Only(2)])
        );
    }

    #[test]
    fn shape_macro_open_5() {
        assert_eq!(
            shapefact![(1 + 1), _, 2; ..],
            ShapeFact::open(tvec![
                GenericFact::Only(2),
                GenericFact::Any,
                GenericFact::Only(2),
            ])
        );
    }
}
