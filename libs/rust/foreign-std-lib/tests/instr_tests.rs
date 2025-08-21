#[cfg(test)]
mod instr_combine_test {
    use core_std_lib::instruction::inst::Instruction;
    use foreign_std_lib::instr::rs_instr::RInstr;
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
    fn trans_to_and_back_is_eq(#[case] instr: Instruction<i32>) {
        assert_eq!(RInstr::from(instr.clone()).to_instr(), instr,);
    }

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
    fn trans_noop_is_unit(#[case] instr: Instruction<i32>) {
        let unit = Instruction::NoOp;
        assert_eq!(
            RInstr::from(instr.clone()).combine(RInstr::from(unit.clone())),
            RInstr::from(instr)
        );
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
    fn trans_combine_is_assoc(
        #[case] a: Instruction<i32>,
        #[case] b: Instruction<i32>,
        #[case] c: Instruction<i32>,
    ) {
        // We are doing `flatten`, because the Instruction trees are different, but we only care about order

        assert_eq!(
            RInstr::from(a.clone().combine(b.clone().combine(c.clone()))).flatten(),
            RInstr::from(a.combine(b).combine(c)).flatten()
        );
    }
}

#[cfg(test)]
mod instr_flatten {
    use core_std_lib::instruction::inst::Instruction;
    use foreign_std_lib::instr::rs_instr::RInstr;
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
    #[case(
    Instruction::Then(
        Box::new(Instruction::Then(
            Box::new(Instruction::Add("root".to_string(), 100)),
            Box::new(
        Instruction::Then(
            Box::new(
                Instruction::Then(
                Box::new(Instruction::Add("child".to_string(), 200)),
                Box::new(Instruction::NoOp)
            )
            ),
                Box::new(Instruction::Rem("root".to_string(), 50))
            )
            )
        )),
        Box::new(Instruction::Then(
            Box::new(Instruction::Rem("child".to_string(), 100)),
            Box::new(Instruction::Add("grandchild".to_string(), 300))
        ))
    )
)]
    #[case(
    Instruction::Then(
        Box::new(Instruction::Add("a".to_string(), 10)),
        Box::new(Instruction::Then(
            Box::new(Instruction::Rem("a".to_string(), 5)),
            Box::new(Instruction::Then(
                Box::new(Instruction::Then(
                    Box::new(Instruction::Rem("b".to_string(), 10)),
Box::new(Instruction::Add("b".to_string(), 20)),
                )),
    Box::new(Instruction::Add("c".to_string(), 30))
            ))
        ))
    )
)]
    #[case(
    Instruction::Then(
        Box::new(Instruction::Then(
            Box::new(Instruction::Then(
                Box::new(Instruction::Add("node1".to_string(), 42)),
                Box::new(Instruction::Rem("node1".to_string(), 21))
            )),
            Box::new(Instruction::Then(
                Box::new(Instruction::Add("node2".to_string(), 84)),
                Box::new(Instruction::Rem("node2".to_string(), 42))
            ))
        )),
        Box::new(Instruction::Then(
            Box::new(Instruction::Then(
                Box::new(Instruction::Rem("node3".to_string(), 100)),
                Box::new(Instruction::Then(
                Box::new(Instruction::Add("node4".to_string(), 300)),
                Box::new(Instruction::Add("node3".to_string(), 200)),)))),
            Box::new(Instruction::Rem("node4".to_string(), 150))
        ))
    )
)]
    #[case(
        Instruction::Then(
            Box::new(
                Instruction::Then(
                    Box::new(Instruction::NoOp),
                    Box::new(
                        Instruction::Then(
                            Box::new(Instruction::Add("main".to_string(), 500)),
                            Box::new(Instruction::Then(
                            Box::new(Instruction::Rem("main".to_string(), 250)),
                            Box::new(Instruction::NoOp)))
                    )
                )
            )
        ),
            Box::new(Instruction::Then(Box::new(Instruction::Then(Box::new(Instruction::Add("secondary".to_string(), 1000)),Box::new(Instruction::Add("secondary".to_string(), 500)))),Box::new(Instruction::Rem("secondary".to_string(), 750))))
        )
    )]
    fn trans_flattening_unflatteing_instr_are_equivalent(#[case] instr: Instruction<i32>) {
        assert!(
            RInstr::unflatten(RInstr::from(instr.clone()).flatten())
                .equivalent(&RInstr::from(instr))
        );
    }

    #[rstest]
    #[case(Instruction::Add("foobar".to_string(), 10))]
    #[case(Instruction::Add("bar".to_string(), 100))]
    #[case(Instruction::Add("".to_string(), 100))]
    #[case(Instruction::Rem("foobar".to_string(), 10))]
    #[case(Instruction::Rem("bar".to_string(), 100))]
    #[case(Instruction::Rem("".to_string(), 100))]
    #[case(Instruction::Then(Box::new(Instruction::Add("foobar".to_string(), 10)), Box::new(Instruction::NoOp)))]
    #[case(Instruction::Then(Box::new(Instruction::NoOp), Box::new(Instruction::Add("foobar".to_string(), 10))))]
    #[case(Instruction::Then(Box::new(Instruction::Add("foobar".to_string(), 10)), Box::new(Instruction::Add("foobar".to_string(), 10))))]
    #[case(
    Instruction::Then(
        Box::new(Instruction::Then(
            Box::new(Instruction::Add("root".to_string(), 100)),
            Box::new(
        Instruction::Then(
            Box::new(
                Instruction::Then(
                Box::new(Instruction::Add("child".to_string(), 200)),
                Box::new(Instruction::NoOp)
            )
            ),
                Box::new(Instruction::Rem("root".to_string(), 50))
            )
            )
        )),
        Box::new(Instruction::Then(
            Box::new(Instruction::Rem("child".to_string(), 100)),
            Box::new(Instruction::Add("grandchild".to_string(), 300))
        ))
    )
)]
    #[case(
    Instruction::Then(
        Box::new(Instruction::Add("a".to_string(), 10)),
        Box::new(Instruction::Then(
            Box::new(Instruction::Rem("a".to_string(), 5)),
            Box::new(Instruction::Then(
                                Box::new(Instruction::Then(
                    Box::new(Instruction::Rem("b".to_string(), 10)),
Box::new(Instruction::Add("b".to_string(), 20)),)),
    Box::new(Instruction::Add("c".to_string(), 30))
            ))
        ))
    )
)]
    #[case(
    Instruction::Then(
        Box::new(Instruction::Then(
            Box::new(Instruction::Then(
                Box::new(Instruction::Add("node1".to_string(), 42)),
                Box::new(Instruction::Rem("node1".to_string(), 21))
            )),
            Box::new(Instruction::Then(
                Box::new(Instruction::Add("node2".to_string(), 84)),
                Box::new(Instruction::Rem("node2".to_string(), 42))
            ))
        )),
        Box::new(Instruction::Then(
            Box::new(Instruction::Then(
                Box::new(Instruction::Rem("node3".to_string(), 100)),
                Box::new(Instruction::Then(
                Box::new(Instruction::Add("node4".to_string(), 300)),
                Box::new(Instruction::Add("node3".to_string(), 200)))) 
            )),
            Box::new(Instruction::Rem("node4".to_string(), 150))
        ))
    )
)]
    #[case(
        Instruction::Then(
            Box::new(
                Instruction::Then(
                    Box::new(Instruction::NoOp),
                    Box::new(
                        Instruction::Then(
                            Box::new(Instruction::Add("main".to_string(), 500)),
                            Box::new(Instruction::Then(
                            Box::new(Instruction::Rem("main".to_string(), 250)),
                            Box::new(Instruction::NoOp)))
                    )
                )
            )
        ),
            Box::new(Instruction::Then(Box::new(Instruction::Then(Box::new(Instruction::Add("secondary".to_string(), 1000)),Box::new(Instruction::Add("secondary".to_string(), 500)))),Box::new(Instruction::Rem("secondary".to_string(), 750))))
        )
    )]
    fn trans_flattening_opt_does_not_contain_noop(#[case] instr: Instruction<i32>) {
        assert!(
            !RInstr::opt(&RInstr::from(instr).flatten())
                .flatten()
                .contains(&RInstr::NoOp)
        )
    }
}
