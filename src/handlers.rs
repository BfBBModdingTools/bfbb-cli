use rustyline::{Cmd, ConditionalEventHandler, Event, EventContext, EventHandler, RepeatCount};
pub struct CtrlCHandler;
impl ConditionalEventHandler for CtrlCHandler {
    fn handle(&self, _: &Event, _: RepeatCount, _: bool, ctx: &EventContext) -> Option<Cmd> {
        if !ctx.line().is_empty() {
            Some(Cmd::Interrupt)
        } else {
            // None // default interrupt
            Some(Cmd::EndOfFile)
        }
    }
}

impl From<CtrlCHandler> for EventHandler {
    fn from(_: CtrlCHandler) -> Self {
        EventHandler::Conditional(Box::new(CtrlCHandler))
    }
}
