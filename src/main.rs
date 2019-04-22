use cpal;


fn list_devices() {
    println!("Default Input Device:\n  {:?}", cpal::default_input_device().map(|e| e.name()));
    println!("Default Output Device:\n  {:?}", cpal::default_output_device().map(|e| e.name()));

    let devices = cpal::devices();
    println!("Devices: ");
    for (device_index, device) in devices.enumerate() {
        println!("{}. \"{}\"",
                 device_index + 1,
                 device.name());

        // Input formats
        if let Ok(fmt) = device.default_input_format() {
            println!("  Default input stream format:\n    {:?}", fmt);
        }
        let mut input_formats = match device.supported_input_formats() {
            Ok(f) => f.peekable(),
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            },
        };
        if input_formats.peek().is_some() {
            println!("  All supported input stream formats:");
            for (format_index, format) in input_formats.enumerate() {
                println!("    {}.{}. {:?}", device_index + 1, format_index + 1, format);
            }
        }

        // Output formats
        if let Ok(fmt) = device.default_output_format() {
            println!("  Default output stream format:\n    {:?}", fmt);
        }
        let mut output_formats = match device.supported_output_formats() {
            Ok(f) => f.peekable(),
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            },
        };
        if output_formats.peek().is_some() {
            println!("  All supported output stream formats:");
            for (format_index, format) in output_formats.enumerate() {
                println!("    {}.{}. {:?}", device_index + 1, format_index + 1, format);
            }
        }
    }
}

fn beep() {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate = format.sample_rate.0 as f32;
    let mut sample_clock = 0f32;

    // Produce a sinusoid of maximum amplitude.
    let mut next_value = || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
    };

    event_loop.run(move |_, data| {
        match data {
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (next_value() * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = next_value();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            _ => (),
        }
    });
}


fn main() {
    list_devices();
//    beep();
}