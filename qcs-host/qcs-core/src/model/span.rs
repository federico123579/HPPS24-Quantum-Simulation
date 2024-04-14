//! Span types for representing ranges of lanes in a gate or multiple gates.

use std::{cmp::Ordering, iter::once, ops::Range};

/// Span type for representing ranges of lanes in a gate.
///
/// Let's say we have a span covering (0,2), in a quantum circuit with 3 lanes
/// this would represent for example a CNOT layout like this:
///
/// ```ascii
///       ┌───┐
/// ──────│ C │────── <--- lane 0 ┐ (control)
///       └─┬─┘                   │
/// ────────│──────── <--- lane 1 │
///       ┌─┴─┐                   │
/// ──────│ X │────── <--- lane 2 ┘ (target)
///       └───┘
/// ```
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

    /// Returns the number of lanes covered in the span
    #[inline]
    pub fn span_len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn min(&self) -> usize {
        *self.0.first().unwrap()
    }

    #[inline]
    pub fn max(&self) -> usize {
        *self.0.last().unwrap()
    }

    /// Return if value is in the span
    #[inline]
    pub fn contains(&self, value: usize) -> bool {
        self.0.contains(&value)
    }

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

#[derive(Debug, Clone, Default)]
pub struct SpanRegister<T: Clone> {
    spans: Vec<(Span, T)>,
}

impl<T: Clone> SpanRegister<T> {
    pub fn new() -> Self {
        SpanRegister { spans: Vec::new() }
    }

    pub fn apply(&mut self, span: Span, value: T) {
        let span_c = span.clone();
        self.spans = self
            .spans
            .clone()
            .into_iter()
            .filter_map(|(s, v)| Some((s.exclude(&span_c)?, v)))
            .chain(once((span, value)))
            .collect();
    }

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

    #[test]
    fn test_span_register() {
        use super::SpanRegister;

        let mut register = SpanRegister::new();
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