use alloc::vec::Vec;
use core::fmt;
use ndarray::{Array2, Axis, concatenate};

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Element {
    Qbit(usize),
    Cbit(usize),
    /// Max 3 characters
    Gate(&'static str),
    StraightWire,
    CrossedWire,
    ControlTop(ControlEnd),
    ControlBottom(ControlEnd),
}

pub struct CircuitDrawing {
    qbits: usize,
    cbits: usize,
    elements: Array2<Element>,
}

impl CircuitDrawing {
    pub fn new(qbits: usize, cbits: usize) -> Self {
        Self {
            qbits,
            cbits,
            elements: Array2::from_shape_vec(
                (qbits + cbits, 1),
                (0..qbits)
                    .map(Element::Qbit)
                    .chain((0..cbits).map(Element::Cbit))
                    .collect(),
            )
            .unwrap(),
        }
    }

    fn row(&self, pos: &Position) -> usize {
        match *pos {
            Position::Qbit(i) => i,
            Position::Cbit(i) => self.qbits + i,
        }
    }

    fn push_column(&mut self, col: Vec<Element>) {
        let rows = self.qbits + self.cbits;
        let col = Array2::from_shape_vec((rows, 1), col).unwrap();
        self.elements = concatenate(Axis(1), &[self.elements.view(), col.view()]).unwrap();
    }

    pub fn push_box(&mut self, target: Position, name: &'static str) {
        let target_row = self.row(&target);
        let rows = self.qbits + self.cbits;
        let col: Vec<Element> = (0..rows)
            .map(|r| {
                if r == target_row {
                    Element::Gate(name)
                } else {
                    Element::StraightWire
                }
            })
            .collect();
        self.push_column(col);
    }

    pub fn push_box_with_control(
        &mut self,
        target: Position,
        name: &'static str,
        control: Position,
        end: ControlEnd,
    ) {
        let target_row = self.row(&target);
        let control_row = self.row(&control);
        let (top, bottom) = if control_row < target_row {
            (control_row, target_row)
        } else {
            (target_row, control_row)
        };
        let rows = self.qbits + self.cbits;
        let col: Vec<Element> = (0..rows)
            .map(|r| {
                if r == target_row {
                    Element::Gate(name)
                } else if r == control_row {
                    if control_row < target_row {
                        Element::ControlTop(end)
                    } else {
                        Element::ControlBottom(end)
                    }
                } else if r > top && r < bottom {
                    Element::CrossedWire
                } else {
                    Element::StraightWire
                }
            })
            .collect();
        self.push_column(col);
    }

    pub fn push_double_control(
        &mut self,
        target1: Position,
        end1: ControlEnd,
        target2: Position,
        end2: ControlEnd,
    ) {
        let row1 = self.row(&target1);
        let row2 = self.row(&target2);
        let (top, bottom) = if row1 < row2 {
            (row1, row2)
        } else {
            (row2, row1)
        };
        let rows = self.qbits + self.cbits;
        let col: Vec<Element> = (0..rows)
            .map(|r| {
                if r == row1 {
                    if row1 < row2 {
                        Element::ControlTop(end1)
                    } else {
                        Element::ControlBottom(end1)
                    }
                } else if r == row2 {
                    if row2 < row1 {
                        Element::ControlTop(end2)
                    } else {
                        Element::ControlBottom(end2)
                    }
                } else if r > top && r < bottom {
                    Element::CrossedWire
                } else {
                    Element::StraightWire
                }
            })
            .collect();
        self.push_column(col);
    }
}

impl fmt::Display for CircuitDrawing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = self.qbits + self.cbits;
        let ncols = self.elements.ncols();

        // Compute label width: max of "q{i}" / "c{i}" labels
        fn digit_count(mut n: usize) -> usize {
            if n == 0 {
                return 1;
            }
            let mut count = 0;
            while n > 0 {
                count += 1;
                n /= 10;
            }
            count
        }
        let max_label_width = (0..rows)
            .map(|r| match self.elements[(r, 0)] {
                Element::Qbit(i) | Element::Cbit(i) => 1 + digit_count(i),
                _ => 0,
            })
            .max()
            .unwrap_or(1);

        struct Label(char, usize);
        impl fmt::Display for Label {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}", self.0, self.1)
            }
        }

        let label_pad = max_label_width + 3;

        // Whether a cell has content on its top sub-line
        let has_top = |r: usize, c: usize| {
            matches!(
                self.elements[(r, c)],
                Element::Gate(_) | Element::CrossedWire | Element::ControlBottom(_)
            )
        };

        // Whether a cell has content on its bottom sub-line
        let has_bot = |r: usize, c: usize| {
            matches!(
                self.elements[(r, c)],
                Element::Gate(_) | Element::CrossedWire | Element::ControlTop(_)
            )
        };

        for r in 0..rows {
            let row_has_gate =
                (1..ncols).any(|c| matches!(self.elements[(r, c)], Element::Gate(_)));

            // Top sub-line (only for rows with gates)
            if row_has_gate {
                if let Some(last) = (1..ncols).rfind(|&c| has_top(r, c)) {
                    for _ in 0..label_pad {
                        f.write_str(" ")?;
                    }
                    for c in 1..=last {
                        match self.elements[(r, c)] {
                            Element::Gate(_) => {
                                let vert = r > 0
                                    && matches!(
                                        self.elements[(r - 1, c)],
                                        Element::ControlTop(_) | Element::CrossedWire
                                    );
                                if vert {
                                    f.write_str("╭─┴─╮")?;
                                } else {
                                    f.write_str("╭───╮")?;
                                }
                            }
                            Element::CrossedWire | Element::ControlBottom(_) => {
                                if c == last {
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

            // Middle sub-line
            match self.elements[(r, 0)] {
                Element::Qbit(i) => {
                    write!(f, "{:<width$} : ", Label('q', i), width = max_label_width)?
                }
                Element::Cbit(i) => {
                    write!(f, "{:<width$} : ", Label('c', i), width = max_label_width)?
                }
                _ => {}
            }
            for c in 1..ncols {
                match self.elements[(r, c)] {
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
            f.write_str("\n")?;

            // Bottom sub-line (only for rows with gates)
            if row_has_gate {
                if let Some(last) = (1..ncols).rfind(|&c| has_bot(r, c)) {
                    for _ in 0..label_pad {
                        f.write_str(" ")?;
                    }
                    for c in 1..=last {
                        match self.elements[(r, c)] {
                            Element::Gate(_) => {
                                let vert = r + 1 < rows
                                    && matches!(
                                        self.elements[(r + 1, c)],
                                        Element::ControlBottom(_) | Element::CrossedWire
                                    );
                                if vert {
                                    f.write_str("╰─┬─╯")?;
                                } else {
                                    f.write_str("╰───╯")?;
                                }
                            }
                            Element::CrossedWire | Element::ControlTop(_) => {
                                if c == last {
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
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    /// Extract the last column from the elements grid.
    fn last_col(d: &CircuitDrawing) -> Vec<Element> {
        let ncols = d.elements.ncols();
        d.elements.column(ncols - 1).to_vec()
    }

    #[test]
    fn new_creates_label_column() {
        let d = CircuitDrawing::new(2, 1);
        assert_eq!(d.elements.dim(), (3, 1));
        assert_eq!(
            last_col(&d),
            vec![Element::Qbit(0), Element::Qbit(1), Element::Cbit(0)]
        );
    }

    #[test]
    fn push_box_places_gate_at_target() {
        let mut d = CircuitDrawing::new(3, 0);
        d.push_box(Position::Qbit(1), "H");
        assert_eq!(d.elements.dim(), (3, 2));
        assert_eq!(
            last_col(&d),
            vec![
                Element::StraightWire,
                Element::Gate("H"),
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
                Element::Gate("M")
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
                Element::Gate("X"),
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
                Element::Gate("X"),
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
                Element::Gate("Z"),
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
        assert_eq!(d.elements.dim(), (2, 3));
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
        d.push_box_with_control(Position::Qbit(0), "X", Position::Qbit(1), ControlEnd::Target);
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
     ╰───╯  │
          ╭─┴─╮
q1 : ─────┤ X ├─────
          ╰───╯
               ╭───╮
c0 : ──────────┤ M ├
               ╰───╯
"#,
        );
    }
}
