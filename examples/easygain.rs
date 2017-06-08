#[macro_use] extern crate vst2;
#[macro_use] extern crate easyvst;
#[macro_use] extern crate log;
extern crate log_panics;
extern crate simplelog;
extern crate num_traits;
extern crate asprim;

use simplelog::*;

use num_traits::Float;
use asprim::AsPrim;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Category, Info, HostCallback};
use vst2::host::Host;

use easyvst::*;
use easyvst::util::*;

easyvst!(ParamId, MyState, MyPlugin);

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum ParamId {
	GainDb,
}

#[derive(Default)]
struct MyState {
	gain_amp: f32,
}

impl UserState<ParamId> for MyState {
	fn param_changed(&mut self, _host: &mut HostCallback, param_id: ParamId, val: f32) {
		info!("param_changed {:?} {:.2}", param_id, val);
		use ParamId::*;
		match param_id {
			GainDb => self.gain_amp = db_to_amp(val),
		}
	}

	fn format_param(&self, param_id: ParamId, val: f32) -> String {
		info!("format_param {:?} {:.2}", param_id, val);
		use ParamId::*;
		match param_id {
			GainDb => format!("{:.2} dB", val),
		}
	}
}

type MyPluginState = PluginState<ParamId, MyState>;

#[derive(Default)]
struct MyPlugin {
	state: MyPluginState,
}

impl MyPlugin {
	fn process_one_channel<F: Float + AsPrim>(&mut self, input: &[F], output: &mut [F]) {
		for (input_sample, output_sample) in input.iter().zip(output) {
			*output_sample = *input_sample * self.state.user_state.gain_amp.as_();
		}
	}
}

impl EasyVst<ParamId, MyState> for MyPlugin {
	fn params() -> Vec<ParamDef> {
		vec![
			ParamDef::new("Gain", -48., 12., 0.),
		]
	}

	fn state(&self) -> &MyPluginState { &self.state }

	fn state_mut(&mut self) -> &mut MyPluginState { &mut self.state }

	fn get_info(&self) -> Info {
		Info {
			name: "easygain".to_string(),
			vendor: "easyvst".to_string(),
			unique_id: 0x3456DCBA,
			category: Category::Effect,

			inputs: 2,
			outputs: 2,
			parameters: 1,

			..Info::default()
		}
	}

	fn new(state: MyPluginState) -> Self {
		let mut p: MyPlugin = Default::default();
		p.state = state;
		p
	}

	fn init(&mut self) {
		#[cfg(windows)]      let my_folder = fs::get_folder_path().unwrap();
		#[cfg(not(windows))] let my_folder = ::std::path::PathBuf::from(".");
		let log_file = File::create(my_folder.join("easygain.log")).unwrap();
		use std::fs::File;
		let _ = CombinedLogger::init(vec![WriteLogger::new(LogLevelFilter::Info, Config::default(), log_file)]);
		info!("init in host {:?}", self.state.host.get_info());
		info!("my folder {:?}", my_folder);
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