# edflib-rs

A Rust library for creating and writing EDF (European Data Format) files, commonly used for storing bio-signal data such as EEG, EMG, and other physiological recordings.

## Table of Contents

* [Features](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#features)
* [Installation](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#installation)
* [Usage](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#usage)
  * [Creating an EDF File](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#creating-an-edf-file)
  * [Writing Sample Streams](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#writing-sample-streams)
  * [Writing Multiple Frames](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#writing-multiple-frames)
  * [Adding Annotations](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#adding-annotations)
  * [Finalizing the EDF File](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#finalizing-the-edf-file)
* [API Documentation](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#api-documentation)
* [Examples](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#examples)
* [Contributing](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#contributing)
* [License](https://chatgpt.com/c/676d7593-83cc-800a-a97a-49d2ed517a50#license)

## Features

* **Comprehensive Metadata Handling** : Easily store patient information, instrument details, and channel-specific metadata.
* **Multi-Channel Support** : Manage multiple channels with individual sample frequencies and physical dimensions.
* **Annotations** : Add annotations with precise onset times, durations, and descriptions.
* **Error Handling** : Robust error handling using the `anyhow` crate.
* **Flexible Data Writing** : Write single frames or multiple frames of multi-channel data efficiently.

## Installation

Add `edf_writer` to your `Cargo.toml`:

```toml
[dependencies]
edf_writer = "0.1.0"
anyhow = "1.0"
```

> **Note** : Replace `"0.1.0"` with the latest version of the crate.

## Usage

### Creating an EDF File

First, set up the necessary metadata, including patient information and channel details.

```rust
use edf_writer::{EDFWriter, EDFHeader, EDFPatientInfo, EDFChannel};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // Define patient information
    let patient_info = EDFPatientInfo {
        patient_name: "John Doe".to_string(),
        patient_code: "JD123".to_string(),
        sex: 1, // 0 for female, 1 for male
        admin_code: "ADM456".to_string(),
        technician: "Tech Name".to_string(),
        equipment: "EEG Device Model X".to_string(),
    };

    // Define channel information
    let channels = vec![
        EDFChannel {
            label: "EEG Fp1".to_string(),
            transducer: "AgAgCl".to_string(),
            digital_max: 32767,
            digital_min: -32768,
            physical_max: 10.0,
            physical_min: -10.0,
            physical_dimension: "uV".to_string(),
            sample_frequency: 256,
        },
        // Add more channels as needed
    ];

    let header = EDFHeader {
        patient_info,
        channels,
    };

    // Initialize the EDFWriter
    let file_path = PathBuf::from("output.edf");
    let mut writer = EDFWriter::new(file_path, header);

    // Open the EDF file for writing
    writer.open()?;

    // ... proceed to write data ...

    writer.finish()?;
    Ok(())
}
```

### Writing Sample Streams

Write a single frame of multi-channel data to the EDF file.

```rust
use anyhow::Result;

// Assuming `writer` is an instance of EDFWriter and has been opened
fn write_single_frame(writer: &mut EDFWriter) -> Result<()> {
    let channel_samples = vec![
        vec![0.1, 0.2, 0.3, /* ... */], // Samples for channel 1
        // Add samples for additional channels
    ];

    writer.write_sample_stream(&channel_samples)?;
    Ok(())
}
```

### Writing Multiple Frames

Write multiple frames of multi-channel data efficiently.

```rust
use anyhow::Result;

// Assuming `writer` is an instance of EDFWriter and has been opened
fn write_multiple_frames(writer: &mut EDFWriter) -> Result<()> {
    let mut frames_data = vec![
        vec![
            vec![0.1, 0.2, 0.3], // Frame 1, Channel 1
            // Add data for additional channels
        ],
        vec![
            vec![0.4, 0.5, 0.6], // Frame 2, Channel 1
            // Add data for additional channels
        ],
        // Add more frames as needed
    ];

    writer.write_multi_frames(&mut frames_data)?;
    Ok(())
}
```

### Adding Annotations

Add annotations to the EDF file to mark specific events or periods.

```rust
use anyhow::Result;

// Assuming `writer` is an instance of EDFWriter and has been opened
fn add_annotation(writer: &mut EDFWriter) -> Result<()> {
    let onset = 1000000; // in microseconds
    let duration = 500000; // in microseconds
    let description = "Event Description".to_string();

    writer.write_annotation(onset, duration, description)?;
    Ok(())
}
```

### Finalizing the EDF File

Once all data has been written, finalize and close the EDF file.

```rust
use anyhow::Result;

// Assuming `writer` is an instance of EDFWriter and has been opened
fn finalize(writer: &mut EDFWriter) -> Result<()> {
    writer.finish()?;
    Ok(())
}
```

## API Documentation

Comprehensive API documentation will be available on ...

## Examples

Check out the [`examples/`](https://chatgpt.com/c/examples/) directory in the repository for more detailed usage examples.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the [MIT License](https://chatgpt.com/c/LICENSE).
