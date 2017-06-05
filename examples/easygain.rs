#[macro_use] extern crate log;
extern crate log_panics;
extern crate simplelog;
extern crate num_traits;
extern crate asprim;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate vst2;
#[macro_use] extern crate easyvst;

use simplelog::*;

use num_traits::{FromPrimitive, Float};
use asprim::AsPrim;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Category, Info};
use vst2::host::Host;

use easyvst::*;
use easyvst::util::*;

easyvst!(ParamId, MyState, MyPlugin);

enum_from_primitive! {
	#[repr(usize)]
	#[derive(Debug, Copy, Clone)]
	pub enum ParamId {
		GainDb,
	}
}

#[derive(Default)]
struct MyState {
	gain_amp: f32,
}

impl UserState<ParamId> for MyState {
	fn param_changed(&mut self, param_id: ParamId, val: f32) {
		info!("param_changed {:?} {:.3}", param_id, val.as_f32());
		use ParamId::*;
		match param_id {
			GainDb => self.gain_amp = db_to_amp(val),
		}
	}
}

type MyPluginState = PluginState<ParamId, MyState>;

#[derive(Default)]
struct MyPlugin {
	state: MyPluginState,
}

impl MyPlugin {
	fn process_one_channel<T: Float + AsPrim>(&mut self, input: &[T], output: &mut [T]) {
		for (input_sample, output_sample) in input.iter().zip(output) {
			*output_sample = *input_sample * self.state.user_state.gain_amp.as_();
		}
	}
}

impl EasyVst<ParamId, MyState> for MyPlugin {
	fn state(&self) -> &MyPluginState {
		&self.state
	}

	fn state_mut(&mut self) -> &mut MyPluginState {
		&mut self.state
	}

	fn get_info(&self) -> Info {
		Info {
			name: "MyPlugin".to_string(),
			vendor: "MyVendor".to_string(),
			unique_id: 0x3456DCBA,
			category: Category::Effect,

			inputs: 2,
			outputs: 2,
			parameters: 1,

			..Info::default()
		}
	}

	fn params() -> Vec<ParamDef> {
		vec![
			ParamDef::new("Gain", (-48.0).as_(), 12.0.as_(), 0.0.as_()),
		]
	}

	fn format_param(param_id: ParamId, val: f32) -> String {
		info!("format_param {:?} {:.3}", param_id, val.as_f32());
		use ParamId::*;
		match param_id {
			GainDb => format!("{:.3} dB", val.as_f32()),
		}
	}

	fn new(state: MyPluginState) -> Self {
		let mut p: MyPlugin = Default::default();
		p.state = state;
		p
	}

	fn init(&mut self) {
		use std::fs::File;
		let _ = CombinedLogger::init(
			vec![
				WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("vst.log").unwrap()),
			]
		);
		info!("init in host {:?}", self.state.host.get_info());
	}

	fn process_f<T: Float + AsPrim>(&mut self, buffer: AudioBuffer<T>) {
		// split out the input and output buffers into two vectors
		let (inputs, outputs) = buffer.split();

		// for each buffer, transform the samples
		for (input_buffer, output_buffer) in inputs.iter().zip(outputs) {
			self.process_one_channel(input_buffer, output_buffer);
		}
	}
}