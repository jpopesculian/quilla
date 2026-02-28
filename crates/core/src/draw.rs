use alloc::vec::Vec;
use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    Qbit(usize),
    Cbit(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControlEnd {
    Cross,
    Target,
    Arrow,
}

#[derive(Clone, Debug, PartialEq)]
enum Element {
    Qbit(usize),
    Cbit(usize),
    /// Max 3 characters
    Gate(String),
    StraightWire,
    CrossedWire,
    ControlTop(ControlEnd),
    ControlBottom(ControlEnd),
}

pub struct CircuitDrawing {
    qbits: usize,
    cbits: usize,
    operations: usize,
    elements: Vec<Element>,
}

impl CircuitDrawing {
    pub fn new(qbits: usize, cbits: usize) -> Self {
        Self {
            qbits,
            cbits,
            operations: 1,
            elements: (0..qbits)
                .map(Element::Qbit)
                .chain((0..cbits).map(Element::Cbit))
                .collect(),
        }
    }

    #[inline]
    fn wire(&self, pos: &Position) -> usize {
        match *pos {
            Position::Qbit(i) => i,
            Position::Cbit(i) => self.qbits + i,
        }
    }

    #[inline]
    fn wires(&self) -> usize {
        self.qbits + self.cbits
    }

    #[inline]
    fn push_operation(&mut self, operation: Vec<Element>) {
        self.elements.extend(operation);
        self.operations += 1;
    }

    #[inline]
    fn get(&self, operation: usize, wire: usize) -> Option<&Element> {
        self.elements.get(operation * self.wires() + wire)
    }

    pub fn push_box(&mut self, target: Position, name: impl ToString) {
        let target_wire = self.wire(&target);
        let operation = (0..self.wires())
            .map(|wire| {
                if wire == target_wire {
                    Element::Gate(name.to_string())
                } else {
                    Element::StraightWire
                }
            })
            .collect();
        self.push_operation(operation);
    }

    pub fn push_box_with_control(
        &mut self,
        target: Position,
        name: impl ToString,
        control: Position,
        end: ControlEnd,
    ) {
        let target_wire = self.wire(&target);
        let control_wire = self.wire(&control);
        let (top, bottom) = if control_wire < target_wire {
            (control_wire, target_wire)
        } else {
            (target_wire, control_wire)
        };
        let operation = (0..self.wires())
            .map(|wire| {
                if wire == target_wire {
                    Element::Gate(name.to_string())
                } else if wire == control_wire {
                    if control_wire < target_wire {
                        Element::ControlTop(end)
                    } else {
                        Element::ControlBottom(end)
                    }
                } else if wire > top && wire < bottom {
                    Element::CrossedWire
                } else {
                    Element::StraightWire
                }
            })
            .collect();
        self.push_operation(operation);
    }

    pub fn push_double_control(
        &mut self,
        control1: Position,
        end1: ControlEnd,
        control2: Position,
        end2: ControlEnd,
    ) {
        let wire1 = self.wire(&control1);
        let wire2 = self.wire(&control2);
        let (top, bottom) = if wire1 < wire2 {
            (wire1, wire2)
        } else {
            (wire2, wire1)
        };
        let operation = (0..self.wires())
            .map(|wire| {
                if wire == wire1 {
                    if wire1 < wire2 {
                        Element::ControlTop(end1)
                    } else {
                        Element::ControlBottom(end1)
                    }
                } else if wire == wire2 {
                    if wire2 < wire1 {
                        Element::ControlTop(end2)
                    } else {
                        Element::ControlBottom(end2)
                    }
                } else if wire > top && wire < bottom {
                    Element::CrossedWire
                } else {
                    Element::StraightWire
                }
            })
            .collect();
        self.push_operation(operation);
    }
}

impl fmt::Display for CircuitDrawing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let wires = self.qbits + self.cbits;
        let ops = self.operations;

        if wires == 0 {
            return Ok(());
        }

        let max_bit_dec_width = (0..wires)
            .filter_map(|wire| match self.get(0, wire)? {
                Element::Qbit(i) | Element::Cbit(i) => {
                    // calculate width of "q{i}" or "c{i}"
                    let mut width = 1;
                    let mut n = i / 10;
                    while n > 0 {
                        width += 1;
                        n /= 10
                    }
                    Some(width)
                }
                _ => None,
            })
            .max()
            .unwrap_or(0);

        let label_pad = max_bit_dec_width + 4;

        let is_gate = |op: usize, wire: usize| matches!(self.get(op, wire), Some(Element::Gate(_)));

        // Whether a cell has content on its top sub-line
        let has_top = |op: usize, wire: usize| {
            matches!(
                self.get(op, wire),
                Some(Element::Gate(_) | Element::CrossedWire | Element::ControlBottom(_))
            )
        };

        // Whether a cell has content on its bottom sub-line
        let has_bot = |op: usize, wire: usize| {
            matches!(
                self.get(op, wire),
                Some(Element::Gate(_) | Element::CrossedWire | Element::ControlTop(_))
            )
        };

        let wire_has_gate: Vec<bool> = (0..wires)
            .map(|wire| (1..ops).any(|op| is_gate(op, wire)))
            .collect();

        // can_merge[r]: merge row r's bottom sub-line with row r+1's top sub-line into one.
        // Possible when both rows have gates but no column has a gate in both rows.
        let can_merge: Vec<bool> = (0..wires)
            .map(|wire| {
                wire + 1 < wires
                    && wire_has_gate[wire]
                    && wire_has_gate[wire + 1]
                    && !(1..ops).any(|op| is_gate(op, wire) && is_gate(op, wire + 1))
            })
            .collect();

        for wire in 0..wires {
            // Top sub-line: skip if the previous row already emitted a merged line covering this.
            let show_top = wire_has_gate[wire] && (wire == 0 || !can_merge[wire - 1]);
            if show_top && let Some(last) = (1..ops).rfind(|op| has_top(*op, wire)) {
                for _ in 0..label_pad {
                    f.write_str(" ")?;
                }
                for op in 1..=last {
                    match self.get(op, wire) {
                        Some(Element::Gate(_)) => {
                            let vert = wire > 0
                                && matches!(
                                    self.get(op, wire - 1),
                                    Some(Element::ControlTop(_) | Element::CrossedWire)
                                );
                            if vert {
                                f.write_str("╭─┴─╮")?;
                            } else {
                                f.write_str("╭───╮")?;
                            }
                        }
                        Some(Element::CrossedWire | Element::ControlBottom(_)) => {
                            if op == last {
                                f.write_str("  │")?;
                            } else {
                                f.write_str("  │  ")?;
                            }
                        }
                        _ => f.write_str("     ")?,
                    }
                }
                f.write_str("\n")?;
            }

            // Middle sub-line
            match self.get(0, wire) {
                Some(Element::Qbit(i)) => {
                    write!(f, "q{:<width$} : ", i, width = max_bit_dec_width)?
                }
                Some(Element::Cbit(i)) => {
                    write!(f, "c{:<width$} : ", i, width = max_bit_dec_width)?
                }
                _ => {}
            }
            for op in 1..ops {
                if let Some(el) = self.get(op, wire) {
                    match el {
                        Element::StraightWire => f.write_str("─────")?,
                        Element::CrossedWire => f.write_str("──┼──")?,
                        Element::Gate(s) => write!(f, "┤{:^3}├", s)?,
                        Element::ControlTop(ControlEnd::Target)
                        | Element::ControlBottom(ControlEnd::Target) => f.write_str("──⊕──")?,
                        Element::ControlTop(ControlEnd::Cross)
                        | Element::ControlBottom(ControlEnd::Cross) => f.write_str("──×──")?,
                        Element::ControlTop(ControlEnd::Arrow) => f.write_str("──△──")?,
                        Element::ControlBottom(ControlEnd::Arrow) => f.write_str("──▽──")?,
                        _ => {}
                    }
                }
            }
            f.write_str("\n")?;

            // Bottom sub-line, or merged bottom+top when adjacent rows can be collapsed.
            if can_merge[wire] {
                // One line covers row r's bottom and row r+1's top.
                if let Some(last) = (1..ops).rfind(|&op| has_bot(op, wire) || has_top(op, wire + 1))
                {
                    for _ in 0..label_pad {
                        f.write_str(" ")?;
                    }
                    for op in 1..=last {
                        if is_gate(op, wire) {
                            let vert = matches!(
                                self.get(op, wire + 1),
                                Some(Element::ControlBottom(_) | Element::CrossedWire)
                            );
                            if vert {
                                f.write_str("╰─┬─╯")?;
                            } else {
                                f.write_str("╰───╯")?;
                            }
                        } else if is_gate(op, wire + 1) {
                            let vert = matches!(
                                self.get(op, wire),
                                Some(Element::ControlTop(_) | Element::CrossedWire)
                            );
                            if vert {
                                f.write_str("╭─┴─╮")?;
                            } else {
                                f.write_str("╭───╮")?;
                            }
                        } else if has_bot(op, wire) || has_top(op, wire + 1) {
                            if op == last {
                                f.write_str("  │")?;
                            } else {
                                f.write_str("  │  ")?;
                            }
                        } else {
                            f.write_str("     ")?;
                        }
                    }
                    f.write_str("\n")?;
                }
            } else if wire_has_gate[wire]
                && let Some(last) = (1..ops).rfind(|op| has_bot(*op, wire))
            {
                for _ in 0..label_pad {
                    f.write_str(" ")?;
                }
                for op in 1..=last {
                    match self.get(op, wire) {
                        Some(Element::Gate(_)) => {
                            let vert = wire + 1 < wires
                                && matches!(
                                    self.get(op, wire + 1),
                                    Some(Element::ControlBottom(_) | Element::CrossedWire)
                                );
                            if vert {
                                f.write_str("╰─┬─╯")?;
                            } else {
                                f.write_str("╰───╯")?;
                            }
                        }
                        Some(Element::CrossedWire | Element::ControlTop(_)) => {
                            if op == last {
                                f.write_str("  │")?;
                            } else {
                                f.write_str("  │  ")?;
                            }
                        }
                        _ => f.write_str("     ")?,
                    }
                }
                f.write_str("\n")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    /// Extract the last column from the elements grid.
    fn last_col(d: &CircuitDrawing) -> &[Element] {
        &d.elements[d.elements.len() - d.wires()..]
    }

    fn assert_dim(d: &CircuitDrawing, ops: usize, wires: usize) {
        assert_eq!(d.elements.len(), ops * wires);
        assert_eq!(d.wires(), wires);
        assert_eq!(d.operations, ops);
    }

    #[test]
    fn new_creates_label_column() {
        let d = CircuitDrawing::new(2, 1);
        assert_dim(&d, 1, 3);
        assert_eq!(
            last_col(&d),
            vec![Element::Qbit(0), Element::Qbit(1), Element::Cbit(0)]
        );
    }

    #[test]
    fn push_box_places_gate_at_target() {
        let mut d = CircuitDrawing::new(3, 0);
        d.push_box(Position::Qbit(1), "H");
        assert_dim(&d, 2, 3);
        assert_eq!(
            last_col(&d),
            vec![
                Element::StraightWire,
                Element::Gate("H".into()),
                Element::StraightWire
            ]
        );
    }

    #[test]
    fn push_box_targets_cbit() {
        let mut d = CircuitDrawing::new(2, 1);
        d.push_box(Position::Cbit(0), "M");
        assert_eq!(
            last_col(&d),
            vec![
                Element::StraightWire,
                Element::StraightWire,
                Element::Gate("M".into())
            ]
        );
    }

    #[test]
    fn push_box_with_control_above_target() {
        let mut d = CircuitDrawing::new(4, 0);
        d.push_box_with_control(
            Position::Qbit(3),
            "X",
            Position::Qbit(0),
            ControlEnd::Target,
        );
        assert_eq!(
            last_col(&d),
            vec![
                Element::ControlTop(ControlEnd::Target),
                Element::CrossedWire,
                Element::CrossedWire,
                Element::Gate("X".into()),
            ]
        );
    }

    #[test]
    fn push_box_with_control_below_target() {
        let mut d = CircuitDrawing::new(4, 0);
        d.push_box_with_control(Position::Qbit(0), "X", Position::Qbit(3), ControlEnd::Cross);
        assert_eq!(
            last_col(&d),
            vec![
                Element::Gate("X".into()),
                Element::CrossedWire,
                Element::CrossedWire,
                Element::ControlBottom(ControlEnd::Cross),
            ]
        );
    }

    #[test]
    fn push_box_with_control_adjacent() {
        let mut d = CircuitDrawing::new(3, 0);
        d.push_box_with_control(
            Position::Qbit(1),
            "Z",
            Position::Qbit(0),
            ControlEnd::Target,
        );
        assert_eq!(
            last_col(&d),
            vec![
                Element::ControlTop(ControlEnd::Target),
                Element::Gate("Z".into()),
                Element::StraightWire,
            ]
        );
    }

    #[test]
    fn push_double_control_target1_above_target2() {
        let mut d = CircuitDrawing::new(4, 0);
        d.push_double_control(
            Position::Qbit(0),
            ControlEnd::Target,
            Position::Qbit(3),
            ControlEnd::Cross,
        );
        assert_eq!(
            last_col(&d),
            vec![
                Element::ControlTop(ControlEnd::Target),
                Element::CrossedWire,
                Element::CrossedWire,
                Element::ControlBottom(ControlEnd::Cross),
            ]
        );
    }

    #[test]
    fn push_double_control_target1_below_target2() {
        let mut d = CircuitDrawing::new(4, 0);
        d.push_double_control(
            Position::Qbit(3),
            ControlEnd::Arrow,
            Position::Qbit(1),
            ControlEnd::Target,
        );
        assert_eq!(
            last_col(&d),
            vec![
                Element::StraightWire,
                Element::ControlTop(ControlEnd::Target),
                Element::CrossedWire,
                Element::ControlBottom(ControlEnd::Arrow),
            ]
        );
    }

    #[test]
    fn push_double_control_adjacent() {
        let mut d = CircuitDrawing::new(3, 0);
        d.push_double_control(
            Position::Qbit(0),
            ControlEnd::Cross,
            Position::Qbit(1),
            ControlEnd::Cross,
        );
        assert_eq!(
            last_col(&d),
            vec![
                Element::ControlTop(ControlEnd::Cross),
                Element::ControlBottom(ControlEnd::Cross),
                Element::StraightWire,
            ]
        );
    }

    #[test]
    fn multiple_pushes_grow_columns() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box(Position::Qbit(0), "H");
        d.push_box(Position::Qbit(1), "X");
        assert_dim(&d, 3, 2);
    }

    fn assert_drawing_str_eq(d: &CircuitDrawing, expected: &str) {
        use alloc::string::ToString;
        assert_eq!(d.to_string(), expected.trim_start_matches('\n'));
    }

    #[test]
    fn display_single_gate() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box(Position::Qbit(0), "H");
        assert_drawing_str_eq(
            &d,
            r#"
     ╭───╮
q0 : ┤ H ├
     ╰───╯
q1 : ─────
"#,
        );
    }

    #[test]
    fn display_controlled_gate() {
        let mut d = CircuitDrawing::new(3, 0);
        d.push_box_with_control(
            Position::Qbit(2),
            "X",
            Position::Qbit(0),
            ControlEnd::Target,
        );
        assert_drawing_str_eq(
            &d,
            r#"
q0 : ──⊕──
q1 : ──┼──
     ╭─┴─╮
q2 : ┤ X ├
     ╰───╯
"#,
        );
    }

    #[test]
    fn display_cross_control_above() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(Position::Qbit(1), "X", Position::Qbit(0), ControlEnd::Cross);
        assert_drawing_str_eq(
            &d,
            r#"
q0 : ──×──
     ╭─┴─╮
q1 : ┤ X ├
     ╰───╯
"#,
        );
    }

    #[test]
    fn display_cross_control_below() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(Position::Qbit(0), "X", Position::Qbit(1), ControlEnd::Cross);
        assert_drawing_str_eq(
            &d,
            r#"
     ╭───╮
q0 : ┤ X ├
     ╰─┬─╯
q1 : ──×──
"#,
        );
    }

    #[test]
    fn display_arrow_control_above() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(Position::Qbit(1), "X", Position::Qbit(0), ControlEnd::Arrow);
        assert_drawing_str_eq(
            &d,
            r#"
q0 : ──△──
     ╭─┴─╮
q1 : ┤ X ├
     ╰───╯
"#,
        );
    }

    #[test]
    fn display_arrow_control_below() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(Position::Qbit(0), "X", Position::Qbit(1), ControlEnd::Arrow);
        assert_drawing_str_eq(
            &d,
            r#"
     ╭───╮
q0 : ┤ X ├
     ╰─┬─╯
q1 : ──▽──
"#,
        );
    }

    #[test]
    fn display_target_control_below() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(
            Position::Qbit(0),
            "X",
            Position::Qbit(1),
            ControlEnd::Target,
        );
        assert_drawing_str_eq(
            &d,
            r#"
     ╭───╮
q0 : ┤ X ├
     ╰─┬─╯
q1 : ──⊕──
"#,
        );
    }

    #[test]
    fn display_two_gates_same_column() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_operation(vec![Element::Gate("H".into()), Element::Gate("X".into())]);
        assert_drawing_str_eq(
            &d,
            r#"
     ╭───╮
q0 : ┤ H ├
     ╰───╯
     ╭───╮
q1 : ┤ X ├
     ╰───╯
"#,
        );
    }

    #[test]
    fn display_multi_column() {
        let mut d = CircuitDrawing::new(2, 1);
        d.push_box(Position::Qbit(0), "H");
        d.push_box_with_control(
            Position::Qbit(1),
            "X",
            Position::Qbit(0),
            ControlEnd::Target,
        );
        d.push_box(Position::Cbit(0), "M");
        assert_drawing_str_eq(
            &d,
            r#"
     ╭───╮
q0 : ┤ H ├──⊕───────
     ╰───╯╭─┴─╮
q1 : ─────┤ X ├─────
          ╰───╯╭───╮
c0 : ──────────┤ M ├
               ╰───╯
"#,
        );
    }
}
