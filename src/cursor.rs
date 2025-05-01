use std::cell::RefCell;

pub struct VecCursor<T> {
    inner: Vec<T>,
    position: RefCell<usize>
}

impl<T> VecCursor<T> {
    pub fn new(inner: Vec<T>) -> Self {
        Self {
            inner,
            position: RefCell::new(0)
        }
    }

    pub fn next(&self) -> Option<&T> {
        let pos = *self.position.borrow();

        if pos >= self.inner.len() {
            return None;
        }

        let ret = self.inner.get(pos);
        
        *self.position.borrow_mut() += 1;

        ret
    }

    pub fn prev(&self) -> Option<&T> {
        let pos = *self.position.borrow();

        if pos == 0 {
            return None;
        }

        let ret = self.inner.get(pos);
        
        *self.position.borrow_mut() -= 1;

        ret
    }

    pub fn current(&self) -> Option<&T> {
        self.inner.get(*self.position.borrow())
    }

    pub fn position(&self) -> usize {
        *self.position.borrow()
    }

    pub fn set_position(&self, position: usize) {
        *self.position.borrow_mut() = position.clamp(0, self.inner.len());
    }
    
    pub(crate) fn reached_end(&self) -> bool {
        *self.position.borrow() >= self.inner.len()
    }
}