use std::{iter::once, ops::Range};

#[derive(Debug, Clone)]
pub struct SpanRegister<T: Clone> {
    spans: Vec<(GateSpan, T)>,
    len: usize,
}

impl<T: Clone> SpanRegister<T> {
    pub fn new(len: usize) -> Self {
        SpanRegister {
            spans: Vec::with_capacity(len),
            len,
        }
    }

    pub fn apply(&mut self, span: GateSpan, value: T) {
        assert!(span.end() <= self.len, "Span out of bounds");
        let span_c = span.clone();
        self.spans = self
            .spans
            .clone()
            .into_iter()
            .flat_map(|(s, v)| s.exclude(&span_c).into_iter().map(move |s| (s, v.clone())))
            .chain(once((span, value)))
            .collect();
    }

    pub fn get(&self, span: &GateSpan) -> Vec<(GateSpan, T)> {
        self.spans
            .iter()
            .filter_map(|(s, v)| s.inner_join(span).map(|s| (s, v.clone())))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateSpan(Range<usize>);

impl GateSpan {
    #[inline]
    pub fn single(lane: usize) -> Self {
        GateSpan(lane..lane + 1)
    }

    #[inline]
    pub fn range(range: Range<usize>) -> Self {
        GateSpan(range)
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
        GateSpan(start..end)
    }

    pub fn inner_join(&self, other: &Self) -> Option<Self> {
        let start = self.start().max(other.start());
        let end = self.end().min(other.end());
        if start < end {
            Some(GateSpan(start..end))
        } else {
            None
        }
    }

    /// Returns the spans that are not covered by the other span
    pub fn exclude(self, other: &Self) -> Vec<Self> {
        let other = match self.inner_join(other) {
            Some(o) => o,
            None => return vec![self],
        };

        let mut result = Vec::new();
        if self.start() < other.start() {
            result.push(GateSpan(self.start()..other.start()));
        }
        if self.end() > other.end() {
            result.push(GateSpan(other.end()..self.end()));
        }
        result
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

#[cfg(test)]
mod tests {
    use crate::utils::GateSpan;

    #[test]
    fn test_span_register() {
        use super::SpanRegister;

        let mut register = SpanRegister::new(5);
        register.apply(GateSpan(0..5), 1);
        register.apply(GateSpan(1..2), 2);
        dbg!(&register);
        register.apply(GateSpan(0..1), 3);
        dbg!(&register);

        assert!(register.spans.contains(&(GateSpan(0..1), 3)));
        assert!(register.spans.contains(&(GateSpan(1..2), 2)));
        assert!(register.spans.contains(&(GateSpan(2..5), 1)));

        let res = register.get(&GateSpan(1..3));
        assert_eq!(res.len(), 2);
        assert!(res.contains(&(GateSpan(1..2), 2)));
        assert!(res.contains(&(GateSpan(2..3), 1)));
    }
}
