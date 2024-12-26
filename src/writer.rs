use crate::base::*;
use anyhow::Result;
use std::{f64::consts::PI, path::PathBuf};
/// Used to store patient information, record instrument information, etc.
#[derive(Debug, Clone)]
pub struct EDFPatientInfo {
    pub patient_name: String,
    pub patient_code: String,
    /// 0 表示 female, 1 表示 male
    pub sex: i32,
    pub admin_code: String,
    pub technician: String,
    pub equipment: String,
}

/// Used to store metadata information for a single channel
#[derive(Debug, Clone)]
pub struct EDFChannel {
    pub label: String,
    pub transducer: String,
    pub digital_max: i32,
    pub digital_min: i32,
    pub physical_max: f64,
    pub physical_min: f64,
    pub physical_dimension: String,
    pub sample_frequency: i32,
}

/// Used to store annotations for EDF/BDF files
#[derive(Debug, Clone)]
pub struct EDFAnnotation {
    pub onset: i32,    // in micosecs
    pub duration: i32, // in micosecs
    pub description: String,
}

/// Used to store header information, contains multiple channels
#[derive(Debug, Clone)]
pub struct EDFHeader {
    pub patient_info: EDFPatientInfo,
    pub channels: Vec<EDFChannel>,
}

pub struct EDFWriter {
    pub file_path: PathBuf,
    pub header: EDFHeader,
    edf: Option<Edf>,
}

impl EDFWriter {
    /// Creates a new `EDFWriter` instance.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A `PathBuf` that points to the file location where the EDF should be written.
    /// * `header` - An `EDFHeader` containing metadata such as channel information.
    pub fn new(file_path: PathBuf, header: EDFHeader) -> Self {
        Self {
            file_path,
            header,
            edf: None,
        }
    }

    /// Opens the EDF file for writing and initializes it with the header information.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or if there is a problem
    /// setting up the header in the file.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// # use your_crate::{EDFWriter, EDFHeader};
    /// # let mut writer = EDFWriter::new(PathBuf::from("path/to/your/file.edf"), EDFHeader::new());
    /// match writer.open() {
    ///     Ok(_) => println!("File opened successfully"),
    ///     Err(e) => println!("Failed to open file: {}", e),
    /// }
    /// ```
    pub fn open(&mut self) -> Result<()> {
        let channel_count = self.header.channels.len();
        let mut edf = Edf::new(self.file_path.clone(), channel_count as i32);

        edf.open_file_writeonly()?;
        // 设置通道及其他头信息
        self.setup_header(&mut edf)?;

        self.edf = Some(edf);
        Ok(())
    }

    /// Writes a single frame of multi-channel data to the EDF file.
    ///
    /// This function expects `channel_samples` where the length of the outer `Vec` matches
    /// the number of channels specified in the `header.channels`, and each inner `Vec<f64>`
    /// contains data points corresponding to the `sample_frequency` of that channel.
    ///
    /// # Arguments
    ///
    /// * `channel_samples` - A vector of vectors containing the sample data for each channel.
    ///   The length of the outer vector must match the number of channels in the header.
    ///   Each inner vector must contain a number of samples that matches the sample frequency
    ///   of the corresponding channel, unless handled otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the length of `channel_samples` does not match the number of channels,
    /// or if writing the data to the file fails.
    ///
    pub fn write_sample_stream(&mut self, channel_samples: &Vec<Vec<f64>>) -> Result<()> {
        if let Some(edf) = &mut self.edf {
            // 检查通道数量是否匹配
            if channel_samples.len() != self.header.channels.len() {
                return Err(anyhow::anyhow!(
                    "给定的通道数据数量({})与header.channels数量({})不一致！",
                    channel_samples.len(),
                    self.header.channels.len()
                ));
            }
            // 依次写入每个通道的数据
            for (ch_idx, ch_data) in channel_samples.iter().enumerate() {
                let channel_info = &self.header.channels[ch_idx];
                if ch_data.len() != channel_info.sample_frequency as usize {
                    eprintln!(
                        "警告: 通道{} 数据点数({})与声明的采样点数({})不一致",
                        ch_idx,
                        ch_data.len(),
                        channel_info.sample_frequency
                    );
                }
                edf.write_samples(&mut ch_data.clone(), channel_info.sample_frequency as usize)?;
            }
        } else {
            return Err(anyhow::anyhow!("EDFWriter 尚未打开文件，请先调用 open()。"));
        }
        Ok(())
    }

    /// Writes multiple frames of multi-channel data to the EDF file.
    ///
    /// This function takes a nested vector where each top-level vector represents a frame (e.g., one second of data),
    /// and each sub-vector within a frame contains data for a specific channel over that frame's duration.
    /// The data for all frames is written sequentially into the file.
    ///
    /// # Parameters
    ///
    /// * `frames_data` - A mutable reference to a vector of frames, where each frame is a vector of channels,
    ///   and each channel is a vector of `f64` representing the data points. The structure is as follows:
    ///   - `frames_data[frame_idx]` represents the data for a specific frame.
    ///   - `frames_data[frame_idx][ch_idx]` contains the data for channel `ch_idx` within that frame.
    ///
    /// # Returns
    ///
    /// A `Result<()>` indicating success or failure. Success returns `Ok(())`, and failure returns an `Err`
    /// with an error message detailing the cause of the failure.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - There is a mismatch in the expected number of channels per frame based on the file's header configuration.
    /// - Any frame contains a different number of data points per channel than expected by the channel's sample frequency.
    /// - The file has not been opened or is otherwise not ready for writing.
    ///
    pub fn write_multi_frames(&mut self, frames_data: &mut Vec<Vec<Vec<f64>>>) -> Result<()> {
        if frames_data.is_empty() {
            return Ok(());
        }

        let mut previous_frame = frames_data[0].clone();

        for (frame_idx, frame) in frames_data.iter_mut().enumerate() {
            if frame.len() != previous_frame.len() {
                eprintln!(
                    "警告: 第 {} 帧的通道数量与前一帧不一致，使用前一帧的数据进行替换。",
                    frame_idx
                );
                *frame = previous_frame.clone();
            } else {
                for (ch_idx, channel_data) in frame.iter_mut().enumerate() {
                    if channel_data.contains(&f64::NAN) {
                        eprintln!(
                            "警告: 第 {} 帧的第 {} 通道包含 NaN 值，使用前一帧的数据进行替换。",
                            frame_idx, ch_idx
                        );
                        *channel_data = previous_frame[ch_idx].clone();
                    }
                }
            }
            self.write_sample_stream(frame)?;
            previous_frame = frame.clone();
        }
        Ok(())
    }

    /// Writes an annotation to the EDF file.
    ///
    /// This function allows you to add annotations to an EDF file, specifying the onset time,
    /// duration, and a textual description of the event. It is important that the file must
    /// be opened with the `open()` method before calling this function.
    ///
    /// # Parameters
    ///
    /// * `onset` - The start time of the annotation in microseconds.
    /// * `duration` - The duration of the annotation in microseconds.
    /// * `description` - A `String` that describes the annotation.
    ///
    /// # Returns
    ///
    /// A `Result<()>` which is `Ok` if the annotation was successfully written, or an `Err`
    /// containing an error message if the operation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the file has not been opened before this function is called.
    ///
    pub fn write_annotation(
        &mut self,
        onset: i64,
        duration: i64,
        description: String,
    ) -> Result<()> {
        if let Some(edf) = &mut self.edf {
            edf.write_annotation(onset, duration, description)?;
        } else {
            return Err(anyhow::anyhow!("EDFWriter 尚未打开文件，请先调用 open()。"));
        }
        Ok(())
    }

    pub fn finish(&mut self) -> Result<()> {
        if let Some(edf) = self.edf.take() {
            edf.finish()?;
        }
        Ok(())
    }

    fn setup_header(&self, edf: &mut Edf) -> Result<()> {
        let patient = &self.header.patient_info;

        edf.set_equipment(patient.equipment.clone())?;
        edf.set_patientname(patient.patient_name.clone())?;
        edf.set_patientcode(patient.patient_code.clone())?;
        edf.set_sex(patient.sex)?;
        edf.set_admincode(patient.admin_code.clone())?;
        edf.set_technician(patient.technician.clone())?;

        for (i, ch) in self.header.channels.iter().enumerate() {
            edf.set_label(i as i32, ch.label.clone())?;
            edf.set_transducer(i as i32, ch.transducer.clone())?;
            edf.set_digital_maximum(i as i32, ch.digital_max)?;
            edf.set_digital_minimum(i as i32, ch.digital_min)?;
            edf.set_physical_maximum(i as i32, ch.physical_max)?;
            edf.set_physical_minimum(i as i32, ch.physical_min)?;
            edf.set_physical_dimension(i as i32, ch.physical_dimension.clone())?;
            edf.set_samplefrequency(i as i32, ch.sample_frequency)?;
        }

        Ok(())
    }
}
