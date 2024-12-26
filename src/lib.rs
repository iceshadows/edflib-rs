mod base;
mod utils;
mod writer;
use crate::base::*;

pub use writer::*;
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_version() {
        let version = Edf::get_edflib_version();
        println!("{}", version);
        assert_eq!(version, "127");
    }

    #[test]
    fn test_open_file_writeonly() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path.clone(), 1);
        assert!(edf.open_file_writeonly().is_ok());

        // 确保文件已创建
        assert!(path.exists());
    }

    #[test]
    fn test_set_samplefrequency() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let mut edf = Edf::new(path, 1);
        edf.open_file_writeonly().unwrap();

        assert!(edf.set_samplefrequency(0, 256).is_ok());
    }

    #[test]
    fn test_set_digital_maximum() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path, 1);
        edf.open_file_writeonly().unwrap();

        assert!(edf.set_digital_maximum(0, 32767).is_ok());
    }

    #[test]
    fn test_set_digital_minimum() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path, 1);
        edf.open_file_writeonly().unwrap();

        assert!(edf.set_digital_minimum(0, -32768).is_ok());
    }

    #[test]
    fn test_set_physical_dimension() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path, 1);
        edf.open_file_writeonly().unwrap();

        assert!(edf.set_physical_dimension(0, "uV".to_string()).is_ok());
    }

    #[test]
    fn test_set_label() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path, 1);
        edf.open_file_writeonly().unwrap();

        assert!(edf.set_label(0, "EEG Fp1".to_string()).is_ok());
    }

    #[test]
    fn test_set_equipment() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path, 1);
        edf.open_file_writeonly().unwrap();

        assert!(edf.set_equipment("Neuroscan".to_string()).is_ok());
    }

    #[test]
    fn test_set_annot_chan_idx_pos() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path.clone(), 1);
        edf.open_file_writeonly().unwrap();

        assert!(edf
            .set_annot_chan_idx_pos(AnnotationPosition::Start)
            .is_ok());
        assert!(edf.finish().is_ok());

        // 确保文件已正确关闭
        assert!(path.exists());
    }

    #[test]
    fn test_set_number_of_annotation_signals() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path, 1);
        edf.open_file_writeonly().unwrap();

        assert!(edf.set_number_of_annotation_signals(1).is_ok());
    }

    #[test]
    fn test_write_samples() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path, 1);
        edf.open_file_writeonly().unwrap();

        // 设置采样频率为256 Hz
        let sample_rate = 256.0;

        // 生成20Hz正弦波数据
        let duration = 10.0; // 10秒
        let num_samples = (sample_rate * duration) as usize;
        let mut data: Vec<f64> = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = i as f64 / sample_rate;
            let value = (2.0 * std::f64::consts::PI * 20.0 * t).sin();
            data.push(value);
        }

        // 写入数据到EDF文件
        // edf.write_samples(&mut data, 256)?;
        assert!(edf.write_samples(&mut data, 256).is_ok());
    }

    #[test]
    fn test_write_annotation() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path, 1);
        edf.open_file_writeonly().unwrap();

        assert!(edf
            .write_annotation(0, 100, "Test Annotation".to_string())
            .is_ok());
    }

    #[test]
    fn test_finish() {
        let temp_file = NamedTempFile::with_suffix(".edf").unwrap();
        let path = temp_file.path().to_path_buf();

        let edf = Edf::new(path.clone(), 1);
        edf.open_file_writeonly().unwrap();

        assert!(edf.finish().is_ok());

        // 确保文件已正确关闭
        assert!(path.exists());
    }
}
