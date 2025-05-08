#[cfg(test)]
mod instr_opt_test {
    use core_std_lib::instruction::inst::Instruction;

    #[test]
    fn optimalization_does_not_remove_single_instr() {
        let add = Instruction::Add("foo".to_string(), 1);
        let add_2 = Instruction::Add("foobar".to_string(), 1);
        let rem = Instruction::Rem("foo".to_string(), 1);
        let opt = Instruction::optimize(vec![add.combine(add_2).combine(rem)]);
        assert_eq!(opt, Instruction::Add("foobar".to_string(), 1));
    }
}

#[cfg(test)]
mod instr_combine_test {
    use core_std_lib::instruction::inst::Instruction;
    use rstest::rstest;

    #[rstest]
    #[case(Instruction::NoOp)]
    #[case(Instruction::Add("foobar".to_string(), 10))]
    #[case(Instruction::Add("bar".to_string(), 100))]
    #[case(Instruction::Add("".to_string(), 100))]
    #[case(Instruction::Rem("foobar".to_string(), 10))]
    #[case(Instruction::Rem("bar".to_string(), 100))]
    #[case(Instruction::Rem("".to_string(), 100))]
    #[case(Instruction::Then(Box::new(Instruction::NoOp), Box::new(Instruction::NoOp)))]
    #[case(Instruction::Then(
        Box::new(Instruction::NoOp),
        Box::new(Instruction::Then(Box::new(Instruction::NoOp), Box::new(Instruction::NoOp)))
    ))]
    #[case(Instruction::Then(Box::new(Instruction::Add("foobar".to_string(), 10)), Box::new(Instruction::NoOp)))]
    #[case(Instruction::Then(Box::new(Instruction::NoOp), Box::new(Instruction::Add("foobar".to_string(), 10))))]
    #[case(Instruction::Then(Box::new(Instruction::Add("foobar".to_string(), 10)), Box::new(Instruction::Add("foobar".to_string(), 10))))]
    fn noop_is_unit(#[case] instr: Instruction<i32>) {
        let unit = Instruction::NoOp;
        assert_eq!(instr.clone().combine(unit.clone()), instr);
    }

    #[rstest]
    #[case(Instruction::NoOp, Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Add("foobar".to_string(), 10), Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Add("bar".to_string(), 100), Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Add("".to_string(), 100), Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Rem("foobar".to_string(), 10), Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Rem("bar".to_string(), 100), Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Rem("".to_string(), 100), Instruction::NoOp, Instruction::NoOp)]
    #[case(
        Instruction::Then(Box::new(Instruction::NoOp), Box::new(Instruction::NoOp)),
        Instruction::NoOp,
        Instruction::NoOp
    )]
    #[case(
        Instruction::Then(
            Box::new(Instruction::NoOp),
            Box::new(Instruction::Then(Box::new(Instruction::NoOp), Box::new(Instruction::NoOp)))
        ),
        Instruction::NoOp,
        Instruction::NoOp
    )]
    #[case(Instruction::Then(Box::new(Instruction::Add("foobar".to_string(), 10)), Box::new(Instruction::NoOp)), Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Then(Box::new(Instruction::NoOp), Box::new(Instruction::Add("foobar".to_string(), 10))), Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Then(Box::new(Instruction::Add("foobar".to_string(), 10)), Box::new(Instruction::Add("foobar".to_string(), 10))), Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Add("foobar".to_string(), 10), Instruction::Add("".to_string(), 0), Instruction::NoOp)]
    #[case(Instruction::Add("bar".to_string(), 100), Instruction::Add("".to_string(), 0), Instruction::NoOp)]
    #[case(Instruction::Add("".to_string(), 100), Instruction::Add("".to_string(), 0), Instruction::NoOp)]
    #[case(Instruction::Rem("foobar".to_string(), 10), Instruction::Add("".to_string(), 0), Instruction::NoOp)]
    #[case(Instruction::Rem("bar".to_string(), 100), Instruction::Add("".to_string(), 0), Instruction::NoOp)]
    #[case(Instruction::Rem("".to_string(), 100), Instruction::Add("".to_string(), 0), Instruction::NoOp)]
    #[case(
        Instruction::Then(Box::new(Instruction::Add("".to_string(), 0)), Box::new(Instruction::NoOp)),
        Instruction::Add("".to_string(), 0),
        Instruction::Add("".to_string(), 0)
    )]
    #[case(
        Instruction::Then(
            Box::new(Instruction::Add("".to_string(), 0)),
            Box::new(Instruction::Then(Box::new(Instruction::Add("".to_string(), 0)), Box::new(Instruction::NoOp)))
        ),
        Instruction::Add("".to_string(), 0),
        Instruction::Add("".to_string(), 0)
    )]
    #[case(Instruction::Then(Box::new(Instruction::Add("foobar".to_string(), 10)), Box::new(Instruction::Add("".to_string(), 0))), Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Then(Box::new(Instruction::Add("".to_string(), 0)), Box::new(Instruction::Add("foobar".to_string(), 10))), Instruction::NoOp, Instruction::NoOp)]
    #[case(Instruction::Then(Box::new(Instruction::Add("foobar".to_string(), 10)), Box::new(Instruction::Add("foobar".to_string(), 10))), Instruction::Add("".to_string(), 0), Instruction::NoOp)]
    fn combine_is_assoc(
        #[case] a: Instruction<i32>,
        #[case] b: Instruction<i32>,
        #[case] c: Instruction<i32>,
    ) {
        // We are doing `flatten`, because the Instruction trees are different, but we only care about order
        assert_eq!(
            a.clone().combine(b.clone().combine(c.clone())).flatten(),
            a.combine(b).combine(c).flatten()
        );
    }
}
