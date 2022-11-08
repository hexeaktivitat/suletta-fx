use std::sync::Arc;

use nih_plug::prelude::*;

struct SulettaFXSat {
    params: Arc<SulettaFXSatParams>,
}

#[derive(Params)]
struct SulettaFXSatParams {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for SulettaFXSat {
    fn default() -> Self {
        Self {
            params: Arc::new(SulettaFXSatParams::default()),
        }
    }
}

impl Default for SulettaFXSatParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-127.0),
                    max: util::db_to_gain(0.0),
                    factor: FloatRange::gain_skew_factor(-127.0, 0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for SulettaFXSat {
    // TODO

    const NAME: &'static str = "Suletta FX Saturator";
    const VENDOR: &'static str = "hexeaktivitat";
    const URL: &'static str = "https://github.com/hexeaktivitat/suletta-fx";
    const EMAIL: &'static str = "hexeaktivitat@gmail.com";

    const VERSION: &'static str = "0.0.1";

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
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();

            for sample in channel_samples {
                *sample *= gain;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for SulettaFXSat {
    const CLAP_ID: &'static str = "Suletta FX Saturator";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Saturator FX for Suletta");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::AudioEffect, ClapFeature::MultiEffects];
}

impl Vst3Plugin for SulettaFXSat {
    const VST3_CLASS_ID: [u8; 16] = *b"SulettaFXSatxxxx";

    const VST3_CATEGORIES: &'static str = "Audio Effect|Multi Effects";
}

nih_export_clap!(SulettaFXSat);
nih_export_vst3!(SulettaFXSat);
