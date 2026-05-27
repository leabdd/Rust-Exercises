use std::ops::Deref;

pub struct ListHead<T> {
    pub first: Option<Box<ListElem<T>>>,
}

#[derive (Default, Clone)]
pub struct ListElem<T> {
    pub value: T,
    pub next: Option<Box<ListElem<T>>>,
}

impl<T> ListHead<T> {
    pub fn insert(&mut self, value: T) {
        self.first = Some(Box::new(ListElem {value, next: self.first.take()}))
    }
}

impl<T: std::fmt::Display> std::fmt::Display for ListHead<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.first {
            Some(x) => { write!(f,"{}",x) },
            _ => { write!(f, "empty list")}
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for ListElem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current: &ListElem<T> = self;
        write!(f, "list: ")?;
        loop {
            write!(f, "{}", current.value)?;
            match &current.next {
                Some(x) => {current = x},
                None => {break},
            }
            write!(f, ", ")?;
        }
        Ok(())
    }
}