struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn with_capacity(cap: usize) -> Self {
        Stack { data: Vec::with_capacity(cap) }
    }

    fn push(&mut self, value: T) { self.data.push(value) }
    fn pop(&mut self) -> Option<T> { self.data.pop() }
    fn peek(&self) -> Option<&T> { self.data.last() }
    //fn peek(&self) -> Option<&T> { Some(&self.data[self.data.len() - 1]) }
    fn is_empty(&self) -> bool { self.data.is_empty() }
    fn size(&self) -> usize { self.data.len() }
}

impl<T: std::fmt::Display> std::fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[");
        for item in &self.data {
            write!(f, "{}, ", item);
        }
        write!(f, "]")
    }
}

struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self { Queue { data: VecDeque::new() } }
    fn with_capacity(cap: usize) -> Self { Queue { data: VecDeque::with_capacity(cap) } }
    fn enqueue(&mut self, value: T) { self.data.push_back(value); }
    fn dequeue(&mut self) -> Option<T> { self.data.pop_front() }
    fn peek(&self) -> Option<&T> { self.data.front() }
    fn is_empty(&self) -> bool { self.data.is_empty() }
    fn size(&self) -> usize { self.data.len() }
}

impl<T: std::fmt::Display> std::fmt::Display for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[");
        for val in &self.data {
            write!(f, "{val}, ");
        }
        write!(f, "]")
    }
}

fn main_main(){
    let mut stack: Stack<String> = Stack::new();
    stack.push("lizzz".into());
    stack.push("timoo".into());
    stack.push("mushh".into());
    println!("{stack}");

    let popped = stack.pop();
    println!("stack: {stack} | popped: {popped:?}");

    let mut queue: Queue<f32> = Queue::new();
    queue.enqueue(3.15);
    queue.enqueue(8.7);
    queue.enqueue(98.0);

    println!("queue: {queue}");
    let popped = queue.dequeue();
    println!("queue: {queue} | popped: {popped:?}\n\n");

    let text = "Rust makes systems programming safer!! and faster!!!! while fearless concurrency helps
        developers build reliable software without worrying constantly about memory corruption and
        unpredictable multithreaded bugs. A a a a a bbbba aaa abbaa a aaa ppp   p  p p../??.!";

    let (x, y, z) = word_frequency_analyzer(text);

    println!("Most reccuring data:\n {:?}\n", x);
    println!("Data appearing once:\n {:?}\n", y);
    println!("Unique Entries:\n {z}\n");
}
