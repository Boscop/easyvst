use asprim::AsPrim;

use vst2::plugin::HostCallback;
use vst2::host::Host;

use std::marker::PhantomData;

use param::*;

pub trait UserState<PID>: Default {
	fn param_changed(&mut self, param_id: PID, val: f32);
}

#[derive(Default)]
pub struct PluginState<PID, S: UserState<PID>> {
	pub host: HostCallback,
	pub params: Vec<Param>,
	pub user_state: S,
	phantom: PhantomData<PID>,
}

impl<PID: Into<usize> + Copy, S: UserState<PID>> PluginState<PID, S> {
	pub fn new(host: HostCallback, params: Vec<ParamDef>) -> Self {
		Self {
			host: host,
			params: params.into_iter().map(Param::new).collect(),
			user_state: Default::default(),
			phantom: PhantomData,
		}
	}

	pub fn get_param_def(&self, param_id: PID) -> &ParamDef {
		&self.params[param_id.into()].def
	}

	pub fn get_param(&self, param_id: PID) -> f32 {
		self.params[param_id.into()].val
	}

	pub fn set_param(&mut self, param_id: PID, val: f32) {
		let pid = param_id.into();
		let param = &mut self.params[pid];
		param.set(val);
		self.user_state.param_changed(param_id, val.as_());
		self.host.automate(pid as i32, param.norm());
	}

	pub fn user_sets_param_norm(&mut self, param_id: usize, val: f32) {
		let param = &mut self.params[param_id];
		param.user_sets_norm(val);
		// TODO: find out if this is necessary
		// self.host.automate(param_id as i32, val);
	}
}