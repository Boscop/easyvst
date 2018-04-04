#![feature(const_fn)]

extern crate vst;
#[macro_use] extern crate log;
extern crate num_traits;
extern crate asprim;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;

use vst::buffer::AudioBuffer;
use vst::plugin::{HostCallback, Plugin, Info, CanDo};
use vst::editor::Editor;
use vst::api::{self, Supported};
use vst::channels::ChannelInfo;

use num_traits::Float;
use asprim::AsPrim;

use std::os::raw::c_void;

pub mod util;
mod param;
mod state;
pub mod fs;

pub use param::*;
pub use state::*;

#[macro_export]
macro_rules! impl_clike {
	($t:ty, $($c:ty) +) => {
		$(
			impl From<$c> for $t {
				fn from(v: $c) -> $t {
					use std::mem;
					unsafe { mem::transmute(v as usize) }
				}
			}

			impl Into<$c> for $t {
				fn into(self) -> $c {
					self as $c
				}
			}
		)*
	};
	($t:ty) => {
		impl_clike!($t, i8 i16 i32 i64 isize u8 u16 u32 u64 usize);
	}
}

#[macro_export]
macro_rules! easyvst {
	($param:ty, $state:ty, $plugin:ty) => {
		impl_clike!($param);
		plugin_main!(EasyVstWrapper<$param, $state, $plugin>);
		impl Default for $param {
			fn default() -> Self { 0.into() }
		}
	}
}

#[allow(unused_variables)]
pub trait EasyVst<PID, S: UserState<PID>> {
	fn get_info(&self) -> Info;
	fn new(state: PluginState<PID, S>) -> Self;
	fn init(&mut self) {}
	fn change_preset(&mut self, preset: i32) {}
	fn get_preset_num(&self) -> i32 { 0 }
	fn set_preset_name(&mut self, name: String) {}
	fn get_preset_name(&self, preset: i32) -> String { "".to_string() }
	fn can_be_automated(&self, index: i32) -> bool { true }
	fn string_to_parameter(&mut self, index: i32, text: String) -> bool { false }
	fn set_sample_rate(&mut self, rate: f32) {}
	fn set_block_size(&mut self, size: i64) {}
	fn resume(&mut self) {}
	fn suspend(&mut self) {}
	fn vendor_specific(&mut self, index: i32, value: isize, ptr: *mut c_void, opt: f32) -> isize { 0 }
	fn can_do(&self, can_do: CanDo) -> Supported {
		trace!("Host is asking if plugin can: {:?}.", can_do);
		Supported::Maybe
	}
	fn get_tail_size(&self) -> isize { 0 }
	fn get_editor(&mut self) -> Option<&mut Editor> { None }
	fn get_preset_data(&mut self) -> Vec<u8> { Vec::new() }
	fn get_bank_data(&mut self) -> Vec<u8> { Vec::new() }
	fn load_preset_data(&mut self, data: &[u8]) {}
	fn load_bank_data(&mut self, data: &[u8]) {}
	fn get_input_info(&self, input: i32) -> ChannelInfo {
		ChannelInfo::new(format!("Input channel {}", input),
						 Some(format!("In {}", input)),
						 true, None)
	}
	fn get_output_info(&self, output: i32) -> ChannelInfo {
		ChannelInfo::new(format!("Output channel {}", output),
						 Some(format!("Out {}", output)),
						 true, None)
	}

	fn state(&self) -> &PluginState<PID, S>;
	fn state_mut(&mut self) -> &mut PluginState<PID, S>;
	fn params() -> Vec<ParamDef>;
	fn process<T: Float + AsPrim>(&mut self, events: &api::Events, buffer: &mut AudioBuffer<T>);
}

use std::marker::PhantomData;

#[derive(Default)]
pub struct EasyVstWrapper<PID, S: UserState<PID>, P: EasyVst<PID, S>>(P, PhantomData<fn(PID, S)>);

impl<PID: Into<usize> + From<usize> + Copy, S: UserState<PID>, P: EasyVst<PID, S>> EasyVstWrapper<PID, S, P> {
	#[inline(always)]
	fn process_f<T: Float + AsPrim>(&mut self, buffer: &mut AudioBuffer<T>) {
		use std::ptr::{null, null_mut};
		let empty: api::Events = api::Events {
			num_events: 0,
			_reserved: 0,
			events: [null_mut(); 2]
		};
		let api_events = self.0.state().api_events;
		let events = if api_events.is_null() {
			&empty
		} else {
			unsafe { &*api_events }
		};
		self.0.process(events, buffer);
		self.0.state_mut().api_events = null();
	}
}

impl<PID: Into<usize> + From<usize> + Copy, S: UserState<PID>, P: EasyVst<PID, S>> Plugin for EasyVstWrapper<PID, S, P> {
	fn new(host: HostCallback) -> Self {
		let params = P::params();
		let param_count = params.len();
		let mut p = P::new(PluginState::new(host, params));
		{
			let state = p.state_mut();
			for i in 0..param_count {
				let val = state.params[i].val;
				state.user_state.param_changed(&mut state.host, i.into(), val);
			}
		}
		EasyVstWrapper(p, PhantomData)
	}

	fn get_info(&self) -> Info { self.0.get_info() }

	fn init(&mut self) { self.0.init(); }

	fn change_preset(&mut self, preset: i32) { self.0.change_preset(preset); }

	fn get_preset_num(&self) -> i32 { self.0.get_preset_num() }

	fn set_preset_name(&mut self, name: String) { self.0.set_preset_name(name); }

	fn get_preset_name(&self, preset: i32) -> String { self.0.get_preset_name(preset) }

	fn can_be_automated(&self, index: i32) -> bool { self.0.can_be_automated(index) }

	fn string_to_parameter(&mut self, index: i32, text: String) -> bool { self.0.string_to_parameter(index, text) }

	fn set_sample_rate(&mut self, rate: f32) { self.0.set_sample_rate(rate); }

	fn set_block_size(&mut self, size: i64) { self.0.set_block_size(size); }

	fn resume(&mut self) { self.0.resume(); }

	fn suspend(&mut self) { self.0.suspend(); }

	fn vendor_specific(&mut self, index: i32, value: isize, ptr: *mut c_void, opt: f32) -> isize {
		self.0.vendor_specific(index, value, ptr, opt)
	}

	fn can_do(&self, can_do: CanDo) -> Supported { self.0.can_do(can_do) }

	fn get_tail_size(&self) -> isize { self.0.get_tail_size() }

	fn process_events(&mut self, events: &api::Events) {
		let state = self.0.state_mut();
		state.api_events = events as *const _;
	}

	fn get_editor(&mut self) -> Option<&mut Editor> { self.0.get_editor() }

	fn get_preset_data(&mut self) -> Vec<u8> { self.0.get_preset_data() }

	fn get_bank_data(&mut self) -> Vec<u8> { self.0.get_bank_data() }

	fn load_preset_data(&mut self, data: &[u8]) { self.0.load_preset_data(data); }

	fn load_bank_data(&mut self, data: &[u8]) { self.0.load_bank_data(data); }

	fn get_input_info(&self, input: i32) -> ChannelInfo { self.0.get_input_info(input) }

	fn get_output_info(&self, output: i32) -> ChannelInfo { self.0.get_output_info(output) }

	fn get_parameter(&self, index: i32) -> f32 {
		trace!("get_parameter {}", index);
		self.0.state().params[index as usize].norm()
	}

	fn set_parameter(&mut self, index: i32, val: f32) {
		trace!("set_parameter {} {:.2}", index, val);
		let i = index as usize;
		self.0.state_mut().user_sets_param_norm(i, val);
		let val = self.0.state().params[i].val;
		let state = self.0.state_mut();
		state.user_state.param_changed(&mut state.host, i.into(), val);
	}

	fn get_parameter_name(&self, index: i32) -> String {
		self.0.state().params[index as usize].def.name.to_string()
	}

	fn get_parameter_text(&self, _index: i32) -> String {
		"".to_string()
	}

	fn get_parameter_label(&self, index: i32) -> String {
		let i = index as usize;
		let val = self.0.state().params[i].val;
		self.0.state().user_state.format_param(i.into(), val)
	}

	fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
		self.process_f(buffer);
		// process_with_midi!(self, f32, buffer);
	}

	fn process_f64(&mut self, buffer: &mut AudioBuffer<f64>) {
		self.process_f(buffer);
		// process_with_midi!(self, f64, buffer);
	}
}