pub trait StackError: std::error::Error {
    fn next(&self) -> Option<&dyn StackError>;

    fn location(&self) -> Option<std::panic::Location<'static>>;

    fn format(&self, layer: usize, buf: &mut Vec<String>) {
        let mut msg = format!("{layer}: {self}");
        if let Some(location) = self.location() {
            msg.push_str(&format!(", at {location}"));
        }
        buf.push(msg);

        if let Some(next) = self.next() {
            next.format(layer + 1, buf);
        } else {
            format_sources(self, layer, buf);
        }
    }
}

fn format_sources(err: impl std::error::Error, layer: usize, buf: &mut Vec<String>) {
    let mut layer = layer;
    let mut err = err.source();
    while let Some(cause) = err {
        layer += 1;
        buf.push(format!("{layer}: {cause}"));
        err = cause.source();
    }
}
