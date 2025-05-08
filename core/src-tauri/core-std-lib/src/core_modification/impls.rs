use crate::attrs::Attr;
use crate::instruction::inst::Instruction;
use crate::state::Value;
use crate::{
    core_modification::CoreModification,
    html::{Html, UIInstructionBuilder},
    state::{State, StateInstructionBuilder},
};

impl Default for CoreModification {
    fn default() -> Self {
        Self {
            state: Instruction::NoOp,
            ui: (Instruction::NoOp, Instruction::NoOp, Instruction::NoOp),
        }
    }
}

impl CoreModification {
    pub fn from_instr(
        state: Instruction<Value>,
        ui: (Instruction<Html>, Instruction<String>, Instruction<Attr>),
    ) -> Self {
        Self { state, ui }
    }

    pub fn ui(ui: UIInstructionBuilder) -> Self {
        CoreModification {
            state: Instruction::NoOp,
            ui: ui.instruction(),
        }
    }

    pub fn append(a: Self, b: Self) -> Self {
        a.combine(b)
    }

    pub fn set_state(self, builder: StateInstructionBuilder) -> Self {
        Self {
            state: builder.instruction(),
            ..self
        }
    }

    pub fn set_ui(self, builder: UIInstructionBuilder) -> Self {
        Self {
            ui: builder.instruction(),
            ..self
        }
    }

    pub fn combine(self, other: Self) -> Self {
        let (node, text, attr) = self.ui;
        let (n, t, a) = other.ui;
        Self {
            state: self.state.combine(other.state),
            ui: (node.combine(n), text.combine(t), attr.combine(a)),
        }
    }

    pub fn build(self, state: State, ui: Html) -> (State, Html) {
        (
            StateInstructionBuilder::new(self.state).build(state),
            UIInstructionBuilder::new(self.ui).build(ui),
        )
    }

    pub fn build_state(self, state: State) -> (State, UIInstructionBuilder) {
        (
            StateInstructionBuilder::new(self.state).build(state),
            UIInstructionBuilder::new(self.ui),
        )
    }

    pub fn get_attr_instr(&self) -> Instruction<Attr> {
        self.ui.2.clone()
    }

    pub fn get_html_instr(&self) -> Instruction<Html> {
        self.ui.0.clone()
    }

    /// Optimizes the modification
    ///
    /// Since an `Instruction<T>` is a group, we can reduce any `Instruction<T>`
    /// where T implements `Eq`, by removing reduntant instructions. A
    /// redundant instruction is an instruction, that when `combine`-ed results
    /// in a `NoOp`, one that results in no change, or a `NoOp`.
    ///
    /// A small example:
    /// ```rust
    /// use core_std_lib::instruction::inst::Instruction;
    ///
    /// let add = Instruction::Add("foo".to_string(), 1);
    /// let rem = Instruction::Rem("foo".to_string(), 1);
    /// let combined = add.combine(rem);
    /// assert_eq!(combined, Instruction::NoOp);
    /// ```
    ///
    /// This combination does not recursivly check for `NoOp`s:
    ///
    /// ```rust
    /// use core_std_lib::instruction::inst::Instruction;
    ///
    /// let add = Instruction::Add("foo".to_string(), 1);
    /// let add_2 = Instruction::Add("foobar".to_string(), 1);
    /// let rem = Instruction::Rem("foo".to_string(), 1);
    /// let combined = add.combine(add_2).combine(rem);
    /// assert_ne!(combined, Instruction::NoOp);
    /// ```
    ///
    /// This is "fixed" by using the `Instruction::optimize`, which recursivly
    /// optimizes for this:
    ///
    /// ```rust
    /// use core_std_lib::instruction::inst::Instruction;
    ///
    /// let add = Instruction::Add("foo".to_string(), 1);
    /// let add_2 = Instruction::Add("foobar".to_string(), 1);
    /// let rem = Instruction::Rem("foo".to_string(), 1);
    /// let combined = Instruction::optimize(vec![add.combine(add_2).combine(rem)]);
    /// assert_eq!(combined, Instruction::Add("foobar".to_string(), 1));
    /// ```
    ///
    /// We can also do further optimalization, since we don't on only work
    /// on `T`, we can also work on different `T`, the way we have structured
    /// the UI, makes it so that there are three instructions for changing the
    /// UI, and these three instructions habe dependencies. For example, if we
    /// `Rem`-ove an Html node, that another `Add`-instruction were going to
    /// add an `Attr` to, we can optimize away this `Add`-instruction, as it is
    /// effectivly an `NoOp`.
    ///
    pub fn optimize(self) -> Self {
        Self {
            state: Instruction::optimize(vec![self.state]),
            ui: (
                Instruction::optimize(vec![self.ui.0]),
                Instruction::optimize(vec![self.ui.1]),
                Instruction::optimize(vec![self.ui.2]),
            ),
        }
    }
}
