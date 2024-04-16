//! Span types for representing ranges of lanes in a gate or multiple gates.

use std::{
    cmp::Ordering,
    iter::once,
    ops::{Range, RangeInclusive},
};

/// Span type for representing ranges of lanes in a gate. This is useful for
/// representing the control and target lanes of a gate, or the lanes covered
/// by a gate in a quantum circuit.
///
/// Let's say we have a span covering (0,2), in a quantum circuit with 3 lanes
/// this would represent for example a CNOT layout like this:
///
/// ```ascii
///       ┌───┐
/// ──────┤ C ├────── <--- lane 0 ┐ (control)
///       └─┬─┘                   │
/// ────────┼──────── <--- lane 1 │
///       ┌─┴─┐                   │
/// ──────┤ X ├────── <--- lane 2 ┘ (target)
///       └───┘
/// ```
///
/// Impl. Note: The inner vector of span is always sorted in ascending order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span(Vec<usize>);

impl Span {
    /// Creates a new span from a list of lanes
    pub fn new(lanes: impl Into<Vec<usize>>) -> Self {
        let mut lanes = lanes.into();
        lanes.sort_unstable();
        Span(lanes)
    }

    /// Creates a new span from a single lane
    #[inline]
    pub fn single(lane: usize) -> Self {
        Span(vec![lane])
    }

    /// Creates a new span from a range of lanes
    #[inline]
    pub fn range(range: Range<usize>) -> Self {
        Span(range.collect())
    }

    /// Creates a new span from a range of lanes
    #[inline]
    pub fn range_in(range: RangeInclusive<usize>) -> Self {
        Span(range.collect())
    }

    /// Returns the number of lanes covered in the span
    #[inline]
    pub fn span_len(&self) -> usize {
        self.0.len()
    }

    /// Returns the minimum lane in the span
    #[inline]
    pub fn start(&self) -> usize {
        *self.0.first().unwrap()
    }

    /// Returns the maximum lane in the span
    #[inline]
    pub fn end(&self) -> usize {
        *self.0.last().unwrap()
    }

    /// Return a new Span covering all lanes from start to end
    pub fn filled(&self) -> Self {
        Span::range_in(self.start()..=self.end())
    }

    /// Return if value is in the span
    #[inline]
    pub fn contains(&self, value: usize) -> bool {
        self.0.contains(&value)
    }

    /// Returns the full join of two spans.
    /// ```
    /// # use qcs_core::model::span::Span;
    /// let a = Span::new([0, 1, 2, 3]);
    /// let b = Span::new([2, 3, 4, 5]);
    /// // The full join of A and B is [0, 1, 2, 3, 4, 5]
    /// assert_eq!(a.full_join(&b), Span::new([0, 1, 2, 3, 4, 5]))
    /// ```
    pub fn full_join(&self, other: &Self) -> Self {
        let mut self_iter = self.0.iter().peekable();
        let mut other_iter = other.0.iter().peekable();
        let mut result = Vec::new();

        loop {
            match (self_iter.peek(), other_iter.peek()) {
                (Some(s), Some(o)) => match s.cmp(o) {
                    Ordering::Equal => {
                        result.push(*self_iter.next().unwrap());
                        other_iter.next();
                    }
                    Ordering::Less => {
                        result.push(*self_iter.next().unwrap());
                    }
                    Ordering::Greater => {
                        result.push(*other_iter.next().unwrap());
                    }
                },
                (Some(_), None) => {
                    result.push(*self_iter.next().unwrap());
                }
                (None, Some(_)) => {
                    result.push(*other_iter.next().unwrap());
                }
                (None, None) => break,
            }
        }

        Self(result)
    }

    /// Returns the inner join of two spans.
    /// ```
    /// # use qcs_core::model::span::Span;
    /// let a = Span::new([0, 1, 2, 3]);
    /// let b = Span::new([2, 3, 4, 5]);
    /// // The inner join of A and B is [2, 3]
    /// assert_eq!(a.inner_join(&b).unwrap(), Span::new([2, 3]))
    /// ```
    pub fn inner_join(&self, other: &Self) -> Option<Self> {
        let mut self_iter = self.0.iter().peekable();
        let mut other_iter = other.0.iter().peekable();
        let mut result = Vec::new();

        while let (Some(s), Some(o)) = (self_iter.peek(), other_iter.peek()) {
            match s.cmp(o) {
                Ordering::Equal => {
                    result.push(*self_iter.next().unwrap());
                    other_iter.next();
                }
                Ordering::Less => {
                    self_iter.next();
                }
                Ordering::Greater => {
                    other_iter.next();
                }
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(Self(result))
        }
    }

    /// Returns the spans that are not covered by the other span
    ///
    /// ```
    /// # use qcs_core::model::span::Span;
    /// let a = Span::new([0, 1, 2, 3]);
    /// let b = Span::new([2, 3, 4, 5]);
    /// // The exclusion of A and B is [0, 1]
    /// assert_eq!(a.exclude(&b).unwrap(), Span::new([0, 1]))
    /// ```
    pub fn exclude(self, other: &Self) -> Option<Self> {
        let self_iter = self.0.into_iter();
        let mut result = Vec::new();

        for s in self_iter {
            if other.0.binary_search(&s).is_err() {
                result.push(s);
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(Self(result))
        }
    }
}

impl IntoIterator for Span {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index_str = self.0.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        write!(f, "[{}]", index_str.join(","))
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Span::range(range)
    }
}

impl From<usize> for Span {
    fn from(lane: usize) -> Self {
        Span::single(lane)
    }
}

/// A `GateSliceView` is like a sliding vertical view that walks over a quantum
/// circuit and keeps track of the spans that are covered by gates.
/// Every time a gate is applied to the circuit, the view is updated with
/// the span of lanes that the gate covers (from start to end).
///
/// It is important to note that the entire range of values between the start
/// and end of the span will be added to the register.
///
/// ```
/// # use qcs_core::model::span::{Span, GateSliceView};
/// let mut reg = GateSliceView::new();
///
/// reg.apply(Span::new(&[0, 5]), 1);
/// // This will override the value 1 in the span [1] with the value 2
/// reg.apply(Span::range(1..2), 2);
/// // This will override the value 1 in the span [0] with the value 3
/// reg.apply(Span::range(0..1), 3);
///
/// let res = reg.get(&Span::range(1..3));
///
/// assert_eq!(res.len(), 2);
/// // The span [1] is covered by the value 2
/// assert!(res.contains(&(Span::range(1..2), 2)));
/// ```
#[derive(Debug, Clone, Default)]
pub struct GateSliceView<T: Clone> {
    spans: Vec<(Span, T)>,
}

impl<T: Clone> GateSliceView<T> {
    /// Creates an empty SpanRegister
    pub fn new() -> Self {
        GateSliceView { spans: Vec::new() }
    }

    /// Applies value to span in span register, overwriting all portions of other
    /// spans that are covered by the new span.
    pub fn apply(&mut self, span: Span, value: T) {
        self.spans = self
            .spans
            .clone()
            .into_iter()
            .filter_map(|(s, v)| Some((s.exclude(&span.filled())?, v)))
            .chain(once((span.filled(), value)))
            .collect();
    }

    /// Returns the spans and the values associated with the given span
    pub fn get(&self, span: &Span) -> Vec<(Span, T)> {
        self.spans
            .iter()
            .filter_map(|(s, v)| Some((s.inner_join(span)?, v.clone())))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test if the SpanRegister works as expected
    #[test]
    fn test_span_register() {
        use super::GateSliceView;

        let mut register = GateSliceView::new();
        register.apply(Span::range(0..5), 1);
        register.apply(Span::range(1..2), 2);
        dbg!(&register);
        register.apply(Span::range(0..1), 3);
        dbg!(&register);

        assert!(register.spans.contains(&(Span::range(0..1), 3)));
        assert!(register.spans.contains(&(Span::range(1..2), 2)));
        assert!(register.spans.contains(&(Span::range(2..5), 1)));

        let res = register.get(&Span::range(1..3));
        assert_eq!(res.len(), 2);
        assert!(res.contains(&(Span::range(1..2), 2)));
        assert!(res.contains(&(Span::range(2..3), 1)));
    }
}
