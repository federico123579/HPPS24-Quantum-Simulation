use std::{cmp::Ordering, iter::once, ops::Range};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateSpan(Range<usize>);

impl GateSpan {
    #[inline]
    pub fn single(lane: usize) -> Self {
        Self(lane..lane + 1)
    }

    #[inline]
    pub fn range(range: Range<usize>) -> Self {
        Self(range)
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.0.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.0.end
    }

    #[inline]
    pub fn span_len(&self) -> usize {
        self.end() - self.start()
    }

    pub fn full_join(&self, other: &Self) -> Self {
        let start = self.start().min(other.start());
        let end = self.end().max(other.end());
        Self(start..end)
    }

    pub fn inner_join(&self, other: &Self) -> Option<Self> {
        let start = self.start().max(other.start());
        let end = self.end().min(other.end());
        if start < end {
            Some(Self(start..end))
        } else {
            None
        }
    }

    pub fn into_range(self) -> Range<usize> {
        self.0
    }
}

impl std::fmt::Display for GateSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.start(), self.end() - 1)
    }
}

impl From<Range<usize>> for GateSpan {
    fn from(range: Range<usize>) -> Self {
        GateSpan(range)
    }
}

impl From<usize> for GateSpan {
    fn from(lane: usize) -> Self {
        GateSpan(lane..lane + 1)
    }
}

impl From<GateSpan> for Range<usize> {
    fn from(span: GateSpan) -> Self {
        span.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpanRegister<T: Clone> {
    spans: Vec<(MultipleSpan, T)>,
}

impl<T: Clone> SpanRegister<T> {
    pub fn new() -> Self {
        SpanRegister { spans: Vec::new() }
    }

    pub fn apply(&mut self, span: MultipleSpan, value: T) {
        let span_c = span.clone();
        self.spans = self
            .spans
            .clone()
            .into_iter()
            .filter_map(|(s, v)| Some((s.exclude(&span_c)?, v)))
            .chain(once((span, value)))
            .collect();
    }

    pub fn get(&self, span: &MultipleSpan) -> Vec<(MultipleSpan, T)> {
        self.spans
            .iter()
            .filter_map(|(s, v)| Some((s.inner_join(span)?, v.clone())))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultipleSpan(Vec<usize>);

impl MultipleSpan {
    #[inline]
    pub fn single(lane: usize) -> Self {
        MultipleSpan(vec![lane])
    }

    #[inline]
    pub fn range(range: Range<usize>) -> Self {
        MultipleSpan(range.collect())
    }

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

    pub fn full_join(&self, other: &Self) -> Self {
        let mut self_iter = self.0.iter().peekable();
        let mut other_iter = other.0.iter().peekable();
        let mut result = Vec::new();

        loop {
            match (self_iter.peek(), other_iter.peek()) {
                (Some(s), Some(o)) => {
                    if s < o {
                        result.push(*self_iter.next().unwrap());
                    } else {
                        result.push(*other_iter.next().unwrap());
                    }
                }
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

impl IntoIterator for MultipleSpan {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::fmt::Display for MultipleSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index_str = self.0.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        write!(f, "[{}]", index_str.join("|"))
    }
}

impl From<Range<usize>> for MultipleSpan {
    fn from(range: Range<usize>) -> Self {
        MultipleSpan::range(range)
    }
}

impl From<usize> for MultipleSpan {
    fn from(lane: usize) -> Self {
        MultipleSpan::single(lane)
    }
}

impl From<GateSpan> for MultipleSpan {
    fn from(span: GateSpan) -> Self {
        MultipleSpan::range(span.into_range())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::MultipleSpan;

    #[test]
    fn test_span_register() {
        use super::SpanRegister;

        let mut register = SpanRegister::new();
        register.apply(MultipleSpan::range(0..5), 1);
        register.apply(MultipleSpan::range(1..2), 2);
        dbg!(&register);
        register.apply(MultipleSpan::range(0..1), 3);
        dbg!(&register);

        assert!(register.spans.contains(&(MultipleSpan::range(0..1), 3)));
        assert!(register.spans.contains(&(MultipleSpan::range(1..2), 2)));
        assert!(register.spans.contains(&(MultipleSpan::range(2..5), 1)));

        let res = register.get(&MultipleSpan::range(1..3));
        assert_eq!(res.len(), 2);
        assert!(res.contains(&(MultipleSpan::range(1..2), 2)));
        assert!(res.contains(&(MultipleSpan::range(2..3), 1)));
    }
}
