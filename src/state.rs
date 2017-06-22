use asprim::AsPrim;

use vst2::plugin::HostCallback;
use vst2::host::Host;
use vst2::api;

use std::marker::PhantomData;
use std::ptr::null;

use param::*;

pub trait UserState<PID> {
	fn new() -> Self where Self: Sized;
	fn param_changed(&mut self, host: &mut HostCallback, param_id: PID, val: f32);
	fn format_param(&self, param_id: PID, val: f32) -> String;
}

pub struct PluginState<PID, S: UserState<PID>> {
	pub host: HostCallback,
	pub params: Vec<Param>,
	pub user_state: S,
	pub(crate) api_events: *const api::Events,
	phantom: PhantomData<PID>,
}

impl<PID: Into<usize> + Copy, S: UserState<PID>> PluginState<PID, S> {
	pub fn new(host: HostCallback, params: Vec<ParamDef>) -> Self {
		Self {
			host: host,
			params: params.into_iter().map(Param::new).collect(),
			user_state: S::new(),
			api_events: null(),
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
		self.user_state.param_changed(&mut self.host, param_id, val.as_());
		self.host.automate(pid as i32, param.norm());
	}

	pub fn user_sets_param_norm(&mut self, param_id: usize, val: f32) {
		let param = &mut self.params[param_id];
		param.user_sets_norm(val);
		// TODO: find out if this is necessary
		// self.host.automate(param_id as i32, val);
	}
}

impl<PID: Into<usize> + Copy, S: UserState<PID>> Default for PluginState<PID, S> {
	fn default() -> Self {
		Self {
			host: Default::default(),
			params: Default::default(),
			user_state: S::new(),
			api_events: null(),
			phantom: Default::default(),
		}
	}
}