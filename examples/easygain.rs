#[macro_use] extern crate vst;
#[macro_use] extern crate easyvst;
#[macro_use] extern crate log;
extern crate log_panics;
extern crate simplelog;
extern crate num_traits;
extern crate asprim;

use simplelog::*;

use num_traits::Float;
use asprim::AsPrim;

use vst::buffer::{AudioBuffer, SendEventBuffer};
use vst::plugin::{Category, Info, HostCallback, CanDo};
use vst::host::Host;
use vst::api::Events;

use easyvst::*;

easyvst!(ParamId, MyState, MyPlugin);

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum ParamId {
	GainDb,
}

#[derive(Default)]
struct MyState {
	gain_amp: f32,
	send_buffer: SendEventBuffer,
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
		#[cfg(windows)]	  let my_folder = fs::get_folder_path().unwrap();
		#[cfg(not(windows))] let my_folder = ::std::path::PathBuf::from(".");
		let log_file = File::create(my_folder.join("easygain.log")).unwrap();
		use std::fs::File;
		let _ = CombinedLogger::init(vec![WriteLogger::new(LogLevelFilter::Info, Config::default(), log_file)]);
		info!("init in host {:?}", self.state.host.get_info());
		info!("my folder {:?}", my_folder);
	}

	fn process<T: Float + AsPrim>(&mut self, events: &Events, buffer: &mut AudioBuffer<T>) {
		// for each buffer, transform the samples
		for (input_buffer, output_buffer) in buffer.zip() {
			self.process_one_channel(input_buffer, output_buffer);
		}
		// forward all midi events
		use vst::event::Event;
		let state = &mut self.state.user_state;
		let events = events.events().filter_map(|e| {
			match e {
				Event::Midi(e) => Some(e),
				_ => None
			}
		});
		state.send_buffer.store_midi(events);
		self.state.host.process_events(state.send_buffer.events());
	}

	fn can_do(&self, can_do: CanDo) -> vst::api::Supported {
		use vst::api::Supported::*;
		use vst::plugin::CanDo::*;

		match can_do {
			SendEvents | SendMidiEvent | ReceiveEvents | ReceiveMidiEvent => Yes,
			_ => No,
		}
	}
}

#[inline]
pub fn amp_to_db<F: Float + AsPrim>(x: F) -> F {
	20.0.as_::<F>() * x.log10()
}

#[inline]
pub fn db_to_amp<F: Float + AsPrim>(x: F) -> F {
	10.0.as_::<F>().powf(x / 20.0.as_())
}