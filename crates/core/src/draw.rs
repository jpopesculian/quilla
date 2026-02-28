use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum DrawPosition {
    Qbit(usize),
    Cbit(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ControlEnd {
    Cross,
    Circle,
    Arrow,
}

#[derive(Clone, Debug, PartialEq)]
enum Element {
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
            operations: 0,
            elements: Vec::new(),
        }
    }

    #[inline]
    pub fn wires(&self) -> usize {
        self.qbits + self.cbits
    }

    #[inline]
    pub fn operations(&self) -> usize {
        self.operations
    }

    #[inline]
    fn wire(&self, pos: &DrawPosition) -> usize {
        match *pos {
            DrawPosition::Qbit(i) => i,
            DrawPosition::Cbit(i) => self.qbits + i,
        }
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

    pub fn draw(&mut self, op: impl DrawOperation) {
        op.draw_to(self);
    }

    pub(crate) fn push_box(&mut self, target: DrawPosition, name: impl ToString) {
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

    pub(crate) fn push_box_with_control(
        &mut self,
        target: DrawPosition,
        name: impl ToString,
        control: DrawPosition,
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

    pub(crate) fn push_double_control(
        &mut self,
        control1: DrawPosition,
        end1: ControlEnd,
        control2: DrawPosition,
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

fn write_dashes(f: &mut fmt::Formatter<'_>, n: usize) -> fmt::Result {
    for _ in 0..n {
        f.write_str("─")?;
    }
    Ok(())
}

fn write_spaces(f: &mut fmt::Formatter<'_>, n: usize) -> fmt::Result {
    for _ in 0..n {
        f.write_str(" ")?;
    }
    Ok(())
}

// Write a single symbol centered in `width` columns, padded with ─
fn write_wire_element(f: &mut fmt::Formatter<'_>, sym: &str, width: usize) -> fmt::Result {
    let sym_len = sym.chars().count();
    let left = (width - sym_len) / 2;
    let right = width - sym_len - left;
    write_dashes(f, left)?;
    f.write_str(sym)?;
    write_dashes(f, right)
}

// ╭─...─╮ or ╭─...connector...─╮ centered
fn write_box_top(f: &mut fmt::Formatter<'_>, width: usize, connector: Option<char>) -> fmt::Result {
    f.write_str("╭")?;
    let inner = width - 2;
    match connector {
        None => write_dashes(f, inner)?,
        Some(c) => {
            let left = (inner - 1) / 2;
            write_dashes(f, left)?;
            write!(f, "{c}")?;
            write_dashes(f, inner - 1 - left)?;
        }
    }
    f.write_str("╮")
}

// ╰─...─╯ or ╰─...connector...─╯ centered
fn write_box_bottom(
    f: &mut fmt::Formatter<'_>,
    width: usize,
    connector: Option<char>,
) -> fmt::Result {
    f.write_str("╰")?;
    let inner = width - 2;
    match connector {
        None => write_dashes(f, inner)?,
        Some(c) => {
            let left = (inner - 1) / 2;
            write_dashes(f, left)?;
            write!(f, "{c}")?;
            write_dashes(f, inner - 1 - left)?;
        }
    }
    f.write_str("╯")
}

// │ centered in `width` columns; trailing spaces omitted when `last`
fn write_vertical(f: &mut fmt::Formatter<'_>, width: usize, last: bool) -> fmt::Result {
    let left = (width - 1) / 2;
    write_spaces(f, left)?;
    f.write_str("│")?;
    if !last {
        write_spaces(f, width - 1 - left)?;
    }
    Ok(())
}

impl fmt::Display for CircuitDrawing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let wires = self.qbits + self.cbits;
        let ops = self.operations;

        if wires == 0 {
            return Ok(());
        }

        let max_bit_dec_width = {
            let i = core::cmp::max(self.qbits, self.cbits) - 1;
            let mut width = 1;
            let mut n = i / 10;
            while n > 0 {
                width += 1;
                n /= 10
            }
            width
        };

        let bit_label_pad = max_bit_dec_width + 4;

        // Per-operation-column width: max gate label width (name + 4), minimum 5.
        let col_widths: Vec<usize> = (0..ops)
            .map(|op| {
                (0..wires)
                    .filter_map(|wire| match self.get(op, wire) {
                        Some(Element::Gate(s)) => Some(s.chars().count() + 4),
                        _ => None,
                    })
                    .max()
                    .unwrap_or(5)
                    .max(5)
            })
            .collect();

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
            .map(|wire| (0..ops).any(|op| is_gate(op, wire)))
            .collect();

        // can_merge[r]: merge row r's bottom sub-line with row r+1's top sub-line into one.
        // Possible when both rows have gates but no column has a gate in both rows.
        let can_merge: Vec<bool> = (0..wires)
            .map(|wire| {
                wire + 1 < wires
                    && wire_has_gate[wire]
                    && wire_has_gate[wire + 1]
                    && !(0..ops).any(|op| is_gate(op, wire) && is_gate(op, wire + 1))
            })
            .collect();

        for wire in 0..wires {
            // Top sub-line: skip if the previous row already emitted a merged line covering this.
            let show_top = wire_has_gate[wire] && (wire == 0 || !can_merge[wire - 1]);
            if show_top && let Some(last) = (0..ops).rfind(|op| has_top(*op, wire)) {
                for _ in 0..bit_label_pad {
                    f.write_str(" ")?;
                }
                for (op, &w) in col_widths.iter().enumerate().take(last + 1) {
                    match self.get(op, wire) {
                        Some(Element::Gate(_)) => {
                            let vert = wire > 0
                                && matches!(
                                    self.get(op, wire - 1),
                                    Some(Element::ControlTop(_) | Element::CrossedWire)
                                );
                            if vert {
                                write_box_top(f, w, Some('┴'))?;
                            } else {
                                write_box_top(f, w, None)?;
                            }
                        }
                        Some(Element::CrossedWire | Element::ControlBottom(_)) => {
                            write_vertical(f, w, op == last)?;
                        }
                        _ => write_spaces(f, w)?,
                    }
                }
                f.write_str("\n")?;
            }

            // Middle sub-line
            if wire < self.qbits {
                write!(f, "q{:<width$} : ", wire, width = max_bit_dec_width)?;
            } else {
                write!(
                    f,
                    "c{:<width$} : ",
                    wire - self.qbits,
                    width = max_bit_dec_width
                )?;
            }
            for (op, &w) in col_widths.iter().enumerate() {
                if let Some(el) = self.get(op, wire) {
                    match el {
                        Element::StraightWire => write_dashes(f, w)?,
                        Element::CrossedWire => write_wire_element(f, "┼", w)?,
                        Element::Gate(s) => write!(f, "┤ {s} ├")?,
                        Element::ControlTop(ControlEnd::Circle)
                        | Element::ControlBottom(ControlEnd::Circle) => {
                            write_wire_element(f, "●", w)?
                        }
                        Element::ControlTop(ControlEnd::Cross)
                        | Element::ControlBottom(ControlEnd::Cross) => {
                            write_wire_element(f, "✖", w)?
                        }
                        Element::ControlTop(ControlEnd::Arrow) => write_wire_element(f, "▲", w)?,
                        Element::ControlBottom(ControlEnd::Arrow) => write_wire_element(f, "▼", w)?,
                    }
                }
            }
            f.write_str("\n")?;

            // Bottom sub-line, or merged bottom+top when adjacent rows can be collapsed.
            if can_merge[wire] {
                // One line covers row r's bottom and row r+1's top.
                if let Some(last) = (0..ops).rfind(|&op| has_bot(op, wire) || has_top(op, wire + 1))
                {
                    for _ in 0..bit_label_pad {
                        f.write_str(" ")?;
                    }
                    for (op, &w) in col_widths.iter().enumerate().take(last + 1) {
                        if is_gate(op, wire) {
                            let vert = matches!(
                                self.get(op, wire + 1),
                                Some(Element::ControlBottom(_) | Element::CrossedWire)
                            );
                            if vert {
                                write_box_bottom(f, w, Some('┬'))?;
                            } else {
                                write_box_bottom(f, w, None)?;
                            }
                        } else if is_gate(op, wire + 1) {
                            let vert = matches!(
                                self.get(op, wire),
                                Some(Element::ControlTop(_) | Element::CrossedWire)
                            );
                            if vert {
                                write_box_top(f, w, Some('┴'))?;
                            } else {
                                write_box_top(f, w, None)?;
                            }
                        } else if has_bot(op, wire) || has_top(op, wire + 1) {
                            write_vertical(f, w, op == last)?;
                        } else {
                            write_spaces(f, w)?;
                        }
                    }
                    f.write_str("\n")?;
                }
            } else if wire_has_gate[wire]
                && let Some(last) = (0..ops).rfind(|op| has_bot(*op, wire))
            {
                for _ in 0..bit_label_pad {
                    f.write_str(" ")?;
                }
                for (op, &w) in col_widths.iter().enumerate().take(last + 1) {
                    match self.get(op, wire) {
                        Some(Element::Gate(_)) => {
                            let vert = wire + 1 < wires
                                && matches!(
                                    self.get(op, wire + 1),
                                    Some(Element::ControlBottom(_) | Element::CrossedWire)
                                );
                            if vert {
                                write_box_bottom(f, w, Some('┬'))?;
                            } else {
                                write_box_bottom(f, w, None)?;
                            }
                        }
                        Some(Element::CrossedWire | Element::ControlTop(_)) => {
                            write_vertical(f, w, op == last)?;
                        }
                        _ => write_spaces(f, w)?,
                    }
                }
                f.write_str("\n")?;
            }
        }

        Ok(())
    }
}

pub trait DrawOperation {
    fn draw_to(&self, d: &mut CircuitDrawing);
}

impl<T> DrawOperation for &T
where
    T: DrawOperation + ?Sized,
{
    fn draw_to(&self, d: &mut CircuitDrawing) {
        T::draw_to(self, d)
    }
}

impl<T> DrawOperation for Box<T>
where
    T: DrawOperation + ?Sized,
{
    fn draw_to(&self, d: &mut CircuitDrawing) {
        T::draw_to(self, d)
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
    fn new_starts_empty() {
        let d = CircuitDrawing::new(2, 1);
        assert_dim(&d, 0, 3);
    }

    #[test]
    fn push_box_places_gate_at_target() {
        let mut d = CircuitDrawing::new(3, 0);
        d.push_box(DrawPosition::Qbit(1), "H");
        assert_dim(&d, 1, 3);
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
        d.push_box(DrawPosition::Cbit(0), "M");
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
            DrawPosition::Qbit(3),
            "X",
            DrawPosition::Qbit(0),
            ControlEnd::Circle,
        );
        assert_eq!(
            last_col(&d),
            vec![
                Element::ControlTop(ControlEnd::Circle),
                Element::CrossedWire,
                Element::CrossedWire,
                Element::Gate("X".into()),
            ]
        );
    }

    #[test]
    fn push_box_with_control_below_target() {
        let mut d = CircuitDrawing::new(4, 0);
        d.push_box_with_control(
            DrawPosition::Qbit(0),
            "X",
            DrawPosition::Qbit(3),
            ControlEnd::Cross,
        );
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
            DrawPosition::Qbit(1),
            "Z",
            DrawPosition::Qbit(0),
            ControlEnd::Circle,
        );
        assert_eq!(
            last_col(&d),
            vec![
                Element::ControlTop(ControlEnd::Circle),
                Element::Gate("Z".into()),
                Element::StraightWire,
            ]
        );
    }

    #[test]
    fn push_double_control_target1_above_target2() {
        let mut d = CircuitDrawing::new(4, 0);
        d.push_double_control(
            DrawPosition::Qbit(0),
            ControlEnd::Circle,
            DrawPosition::Qbit(3),
            ControlEnd::Cross,
        );
        assert_eq!(
            last_col(&d),
            vec![
                Element::ControlTop(ControlEnd::Circle),
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
            DrawPosition::Qbit(3),
            ControlEnd::Arrow,
            DrawPosition::Qbit(1),
            ControlEnd::Circle,
        );
        assert_eq!(
            last_col(&d),
            vec![
                Element::StraightWire,
                Element::ControlTop(ControlEnd::Circle),
                Element::CrossedWire,
                Element::ControlBottom(ControlEnd::Arrow),
            ]
        );
    }

    #[test]
    fn push_double_control_adjacent() {
        let mut d = CircuitDrawing::new(3, 0);
        d.push_double_control(
            DrawPosition::Qbit(0),
            ControlEnd::Cross,
            DrawPosition::Qbit(1),
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
        d.push_box(DrawPosition::Qbit(0), "H");
        d.push_box(DrawPosition::Qbit(1), "X");
        assert_dim(&d, 2, 2);
    }

    fn assert_drawing_str_eq(d: &CircuitDrawing, expected: &str) {
        use alloc::string::ToString;
        pretty_assertions::assert_eq!(d.to_string(), expected.trim_start_matches('\n'));
    }

    #[test]
    fn display_single_gate() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box(DrawPosition::Qbit(0), "H");
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
            DrawPosition::Qbit(2),
            "X",
            DrawPosition::Qbit(0),
            ControlEnd::Circle,
        );
        assert_drawing_str_eq(
            &d,
            r#"
q0 : ──●──
q1 : ──┼──
     ╭─┴─╮
q2 : ┤ X ├
     ╰───╯
"#,
        );
    }

    #[test]
    fn display_wide_gate() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(
            DrawPosition::Qbit(1),
            "Hello",
            DrawPosition::Qbit(0),
            ControlEnd::Circle,
        );
        assert_drawing_str_eq(
            &d,
            r#"
q0 : ────●────
     ╭───┴───╮
q1 : ┤ Hello ├
     ╰───────╯
"#,
        );
    }

    #[test]
    fn display_cross_control_above() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(
            DrawPosition::Qbit(1),
            "X",
            DrawPosition::Qbit(0),
            ControlEnd::Cross,
        );
        assert_drawing_str_eq(
            &d,
            r#"
q0 : ──✖──
     ╭─┴─╮
q1 : ┤ X ├
     ╰───╯
"#,
        );
    }

    #[test]
    fn display_cross_control_below() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(
            DrawPosition::Qbit(0),
            "X",
            DrawPosition::Qbit(1),
            ControlEnd::Cross,
        );
        assert_drawing_str_eq(
            &d,
            r#"
     ╭───╮
q0 : ┤ X ├
     ╰─┬─╯
q1 : ──✖──
"#,
        );
    }

    #[test]
    fn display_arrow_control_above() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(
            DrawPosition::Qbit(1),
            "X",
            DrawPosition::Qbit(0),
            ControlEnd::Arrow,
        );
        assert_drawing_str_eq(
            &d,
            r#"
q0 : ──▲──
     ╭─┴─╮
q1 : ┤ X ├
     ╰───╯
"#,
        );
    }

    #[test]
    fn display_arrow_control_below() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(
            DrawPosition::Qbit(0),
            "X",
            DrawPosition::Qbit(1),
            ControlEnd::Arrow,
        );
        assert_drawing_str_eq(
            &d,
            r#"
     ╭───╮
q0 : ┤ X ├
     ╰─┬─╯
q1 : ──▼──
"#,
        );
    }

    #[test]
    fn display_target_control_below() {
        let mut d = CircuitDrawing::new(2, 0);
        d.push_box_with_control(
            DrawPosition::Qbit(0),
            "X",
            DrawPosition::Qbit(1),
            ControlEnd::Circle,
        );
        assert_drawing_str_eq(
            &d,
            r#"
     ╭───╮
q0 : ┤ X ├
     ╰─┬─╯
q1 : ──●──
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
        d.push_box(DrawPosition::Qbit(0), "H");
        d.push_box_with_control(
            DrawPosition::Qbit(1),
            "X",
            DrawPosition::Qbit(0),
            ControlEnd::Circle,
        );
        d.push_box(DrawPosition::Cbit(0), "M");
        assert_drawing_str_eq(
            &d,
            r#"
     ╭───╮
q0 : ┤ H ├──●───────
     ╰───╯╭─┴─╮
q1 : ─────┤ X ├─────
          ╰───╯╭───╮
c0 : ──────────┤ M ├
               ╰───╯
"#,
        );
    }
}
