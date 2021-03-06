macro_rules! as_expr {
    ($e:expr) => {$e};
}

macro_rules! check {
    (@ $from:ty, $to:ty; $(;)*) => {};

    (@ $from:ty, $to:ty; uident; $($tail:tt)*) => {
        check!(@ $from, $to; v: 0;);
        check!(@ $from, $to; v: 1;);
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; sident; $($tail:tt)*) => {
        check!(@ $from, $to; v: -1;);
        check!(@ $from, $to; v: 0;);
        check!(@ $from, $to; v: 1;);
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; fident; $($tail:tt)*) => {
        check!(@ $from, $to; v: -1.0;);
        check!(@ $from, $to; v:  0.0;);
        check!(@ $from, $to; v:  1.0;);
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; uidenta; $($tail:tt)*) => {
        check!(@ $from, $to; a: 0.0;);
        check!(@ $from, $to; a: 1.0;);
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; sidenta; $($tail:tt)*) => {
        check!(@ $from, $to; a: -1.0;);
        check!(@ $from, $to; a:  0.0;);
        check!(@ $from, $to; a:  1.0;);
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; fidenta; $($tail:tt)*) => {
        check!(@ $from, $to; a: -1.0;);
        check!(@ $from, $to; a:  0.0;);
        check!(@ $from, $to; a:  1.0;);
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; v: $src:expr, !$dst:expr; $($tail:tt)*) => {
        {
            let src: $from = $src;
            let dst: Result<$to, _> = src.value_into();
            assert_eq!(dst, Err($dst));
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; v: $src:expr; $($tail:tt)*) => {
        {
            let src: $from = $src;
            let dst: Result<$to, _> = src.value_into();
            assert_eq!(dst, Ok($src as $to));
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qv: *; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, _> = v.value_into();
                dst == Ok(v as $to)
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qv: (+-$bound:expr); $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError> = v.value_into().map_err(From::from);
                if !(-$bound as $from <= v) {
                    dst == Err(conv::FloatError::Underflow)
                } else if !(v <= $bound as $from) {
                    dst == Err(conv::FloatError::Overflow)
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qv: (, $bound:expr); $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError> = v.value_into().map_err(From::from);
                if !(v <= $bound as $from) {
                    dst == Err(conv::FloatError::Overflow)
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qv: +; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError> = v.value_into().map_err(From::from);
                if !(0 <= v) {
                    dst == Err(conv::FloatError::Underflow)
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qv: +$max:ty; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError> = v.value_into().map_err(From::from);
                if !(v <= <$max>::max_value() as $from) {
                    dst == Err(conv::FloatError::Overflow)
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qv: $bound:ty; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError> = v.value_into().map_err(From::from);
                if !(<$bound>::min_value() as $from <= v) {
                    dst == Err(conv::FloatError::Underflow)
                } else if !(v <= <$bound>::max_value() as $from) {
                    dst == Err(conv::FloatError::Overflow)
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qv: $min:ty, $max:ty; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError> = v.value_into().map_err(From::from);
                if !(<$min>::min_value() as $from <= v) {
                    dst == Err(conv::FloatError::Underflow)
                } else if !(v <= <$max>::max_value() as $from) {
                    dst == Err(conv::FloatError::Overflow)
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; a: $src:expr, !$dst:expr; $($tail:tt)*) => {
        {
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx();
            assert_eq!(dst, Err($dst));
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; a: $src:expr; $($tail:tt)*) => {
        {
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx();
            assert_eq!(dst, Ok($src as $to));
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qa: *; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, _> = v.approx();
                dst == Ok(v as $to)
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qa: +; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError> = v.approx().map_err(From::from);
                if !(0 <= v) {
                    dst == Err(conv::FloatError::Underflow)
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qa: +$max:ty; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError> = v.approx().map_err(From::from);
                if !(v <= <$max>::max_value() as $from) {
                    dst == Err(conv::FloatError::Overflow)
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qa: $bound:ty; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv::FloatError> = v.approx().map_err(From::from);
                if !(<$bound>::min_value() as $from <= v) {
                    dst == Err(conv::FloatError::Underflow)
                } else if !(v <= <$bound>::max_value() as $from) {
                    dst == Err(conv::FloatError::Overflow)
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    (@ $from:ty, $to:ty; qaW: *; $($tail:tt)*) => {
        {
            extern crate quickcheck;

            fn property(v: $from) -> bool {
                let dst: Result<$to, _> = v.approx_with::<Wrapping>();
                dst == Ok(v as $to)
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qaW {:?}", err)
            }
        }
        check!(@ $from, $to; $($tail)*);
    };

    ($from:ty, $to:ty; $($tail:tt)*) => {
        check! { @ $from, $to; $($tail)*; }
    };
}

macro_rules! for_bitness {
    (32 {$($bits32:tt)*} 64 {$($bits64:tt)*}) => {
        as_expr!(
            {
                #[cfg(target_pointer_width="32")]
                fn for_bitness() {
                    $($bits32)*
                }

                #[cfg(target_pointer_width="64")]
                fn for_bitness() {
                    $($bits64)*
                }

                for_bitness()
            }
        )
    };
}
