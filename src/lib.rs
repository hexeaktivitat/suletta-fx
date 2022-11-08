// boilerplate template for additional plugin defs

use std::sync::Arc;

use nih_plug::prelude::*;

struct SulettaFX {
    params: Arc<SulettaFXParams>,
}

#[derive(Params)]
struct SulettaFXParams {

}

impl Default for SulettaFX {
    fn default() -> Self {
        Self {
            params: Arc::new(SulettaFXParams::default()),
        }
    }
}

impl Default for SulettaFXParams {
    fn default() -> Self {
        Self {

        }
    }
}

impl Plugin for SulettaFX {
    // TODO

    const NAME: &'static str = "Suletta FX - Gain";
    const VENDOR: &'static str = "hexeaktivitat";
    const URL: &'static str = "https://github.com/hexeaktivitat/suletta-fx";
    const EMAIL: &'static str = "hexeaktivitat@gmail.com";

    const VERSION: &'static str = "0.1.0";

    const DEFAULT_INPUT_CHANNELS: u32 = 2;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;

    const DEFAULT_AUX_INPUTS: Option<AuxiliaryIOConfig> = None;
    const DEFAULT_AUX_OUTPUTS: Option<AuxiliaryIOConfig> = None;

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        config.num_input_channels == config.num_output_channels && config.num_input_channels > 0
    }

    fn initialize(
        &mut self,
        _bus_config: &BusConfig,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // TODO

        true
    }

    fn reset(&mut self) {
        // TODO
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // TODO

        ProcessStatus::Normal
    }
}

impl ClapPlugin for SulettaFX {
    const CLAP_ID: &'static str = "Suletta FX - ";
    const CLAP_DESCRIPTION: Option<&'static str> = Some(" FX for Suletta");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::AudioEffect, ClapFeature::MultiEffects];
}

impl Vst3Plugin for SulettaFX {
    const VST3_CLASS_ID: [u8; 16] = *b"SulettaFXxxxxxxx";

    const VST3_CATEGORIES: &'static str = "Audio Effect|Multi Effects";
}

nih_export_clap!(SulettaFX);
nih_export_vst3!(SulettaFX);
