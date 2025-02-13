use yazi_shared::{event::Cmd, render, InputError};

use crate::{completion::Completion, input::Input};

pub struct Opt {
	submit: bool,
}

impl From<Cmd> for Opt {
	fn from(c: Cmd) -> Self { Self { submit: c.named.contains_key("submit") } }
}
impl From<bool> for Opt {
	fn from(submit: bool) -> Self { Self { submit } }
}

impl Input {
	pub fn close(&mut self, opt: impl Into<Opt>) {
		let opt = opt.into() as Opt;

		if self.completion {
			Completion::_close();
		}

		if let Some(cb) = self.callback.take() {
			let value = self.snap_mut().value.clone();
			_ = cb.send(if opt.submit { Ok(value) } else { Err(InputError::Canceled(value)) });
		}

		self.ticket = self.ticket.wrapping_add(1);
		self.visible = false;
		render!();
	}
}
