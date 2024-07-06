pub fn linked_list() {
    let mut list: SingleLinkedList<u8> = SingleLinkedList::new();
    println!("{:?}", list);
    list.append(2);
    println!("{:?}", list);
    list.append(3);
    println!("{:?}", list);
    list.append(4);
    println!("{:?}", list);
    list.prepend(1);
    println!("{:?}", list);
    list.append(5);
    println!("{:?}", list);
    list.prepend(6);
    println!("{:?}", list);

    let traversed = list.len();
    println!("Count: {:?}", traversed);
    println!("{:?}", list);

    list.insert_at_position(7, 4);
    println!("{:?}", list);
}

#[derive(Debug, Clone, PartialEq)]
pub struct SingleLinkedList<T> {
    pub front: Option<Box<Link<T>>>,
    length: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link<T> {
    elem: T,
    next: Option<Box<Link<T>>>,
}

impl<T: Clone> SingleLinkedList<T> {
    fn new() -> SingleLinkedList<T> {
        SingleLinkedList {
            front: None,
            length: 0,
        }
    }

    fn len(&mut self) -> usize {
        self.length
    }

    fn prepend(&mut self, elem: T) {
        //let current = ;
        if self.length == 0 {
            self.front = Some(Box::new(Link::new(elem, None)));
        } else {
            self.front = Some(Box::new(Link::new(elem, self.front.clone())))
        }
        self.length += 1;
    }

    fn append(&mut self, elem: T) {
        let mut current: &mut Option<Box<Link<T>>> = &mut self.front;
        while current.is_some() && current.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
        if self.length == 0 {
            self.front = Some(Box::new(Link::new(elem, None)));
        } else {
            current.as_mut().unwrap().next = Some(Box::new(Link::new(elem, None)));
        }
        self.length += 1;
    }

    fn insert_at_position(&mut self, elem: T, position: usize) {
        if self.length == 0 {
            self.front = Some(Box::new(Link::new(elem, None)));
            return;
        }
        let mut current = &mut self.front;
        let mut count: usize = 1;
        while current.is_some() && current.as_ref().unwrap().next.is_some() && count < position - 1
        {
            current = &mut current.as_mut().unwrap().next;
            count += 1;
        }

        if current.as_ref().unwrap().next.is_some() {
            current.as_mut().unwrap().next = Some(Box::new(Link::new(
                elem,
                current.as_mut().unwrap().next.clone(),
            )));
        } else {
            current.as_mut().unwrap().next = Some(Box::new(Link::new(elem, None)))
        }
        self.length += 1;
    }
}

impl<T> Link<T> {
    fn new(elem: T, next: Option<Box<Link<T>>>) -> Self {
        Link { elem, next }
    }
}

impl<T> std::fmt::Display for Link<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", &self.elem)
    }
}

impl<T> std::fmt::Display for SingleLinkedList<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", &self.front.as_ref().unwrap())
    }
}
