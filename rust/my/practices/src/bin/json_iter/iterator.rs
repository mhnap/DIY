#[derive(Debug, Clone)]
pub enum Value {
    Number(usize),
    Array(Vec<Value>),
}

impl IntoIterator for Value {
    type Item = Value;

    type IntoIter = ValueIter;

    fn into_iter(self) -> Self::IntoIter {
        ValueIter {
            index: 0,
            value: Some(self),
            curr_iter: None,
        }
    }
}

pub struct ValueIter {
    index: usize,
    value: Option<Value>,
    curr_iter: Option<Box<ValueIter>>,
}

impl Iterator for ValueIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        // Use the current iter first until it's empty,
        // before going into a next element
        if let Some(curr) = self.curr_iter.as_mut()
            && let Some(next) = curr.next()
        {
            return Some(next);
        }
        self.curr_iter = None;

        // No current iter
        if let Some(value) = self.value.take() {
            let next = match value {
                Value::Array(array) => {
                    if self.index >= array.len() {
                        return None;
                    }
                    let next_elem = array[self.index].clone();
                    self.value = Some(Value::Array(array));
                    self.index += 1;

                    // Make next elem as current iter (only for arrays?)
                    if matches!(next_elem, Value::Array(_)) {
                        let mut curr = Box::new(next_elem.into_iter());
                        let next = curr.next();
                        self.curr_iter = Some(curr);
                        next
                    } else {
                        Some(next_elem)
                    }
                }
                value => Some(value),
            };
            next
        } else {
            None
        }
    }
}
