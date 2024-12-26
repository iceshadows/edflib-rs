use anyhow::Result;
use edflib::{EDFChannel, EDFHeader, EDFPatientInfo, EDFWriter};
use std::f64::consts::PI;
pub fn main() -> Result<()> {
    // 1. 构建通道信息
    let sample_rate = 256;
    let channel_0 = EDFChannel {
        label: "Sine20Hz".to_string(),
        transducer: "AgAgCl cup electrodes".to_string(),
        digital_max: 32767,
        digital_min: -32768,
        physical_max: 2000.0,
        physical_min: -2000.0,
        physical_dimension: "mV".to_string(),
        sample_frequency: sample_rate,
    };
    let channel_1 = EDFChannel {
        label: "Sine50Hz".to_string(),
        transducer: "AgAgCl cup electrodes".to_string(),
        digital_max: 32767,
        digital_min: -32768,
        physical_max: 2000.0,
        physical_min: -2000.0,
        physical_dimension: "mV".to_string(),
        sample_frequency: sample_rate,
    };

    // 2. 构建患者及头信息
    let patient_info = EDFPatientInfo {
        patient_name: "Demo".to_string(),
        patient_code: "0001".to_string(),
        sex: 0, // 0:female, 1:male
        admin_code: "0001".to_string(),
        technician: "DYZS".to_string(),
        equipment: "DYZS".to_string(),
    };

    let header = EDFHeader {
        patient_info,
        channels: vec![channel_0, channel_1],
    };

    // 3. 构建 EDFWriter
    let mut writer = EDFWriter::new("generator.bdf".into(), header);

    // 4. 打开文件进行写入
    writer.open()?;

    // 5. 构造多帧数据：
    //    frames_data[帧索引][通道索引] = Vec<f64>（该帧的采样点）
    let duration_in_seconds = 10; // 10 秒
    let mut frames_data: Vec<Vec<Vec<f64>>> = Vec::with_capacity(duration_in_seconds);

    for _second in 0..duration_in_seconds {
        let mut ch0_data = Vec::with_capacity(sample_rate as usize);
        let mut ch1_data = Vec::with_capacity(sample_rate as usize);

        for i in 0..sample_rate {
            let t = i as f64 / sample_rate as f64;
            // 通道 0：20Hz 正弦波 * 1000.0 (mV)
            let value_0 = (2.0 * PI * 20.0 * t).sin() * 1000.0;
            // 通道 1：50Hz 正弦波 * 1000.0 (mV)
            let value_1 = (2.0 * PI * 50.0 * t).sin() * 1000.0;
            ch0_data.push(value_0);
            ch1_data.push(value_1);
        }

        // 这一帧对应两个通道的数据 (顺序必须与 header.channels 中的通道顺序保持一致)
        frames_data.push(vec![ch0_data, ch1_data]);
    }

    // 6. 使用 "按帧写入" 的方式，一次性写入所有帧
    writer.write_multi_frames(&mut frames_data)?;

    // 7. 写注释
    writer.write_annotation(0, 0, "Start of recording".to_string())?;
    writer.write_annotation(
        (duration_in_seconds * 1_000_000) as i64,
        0,
        "End of recording".to_string(),
    )?;

    // 8. 完成写入并关闭文件
    writer.finish()?;

    println!("BDF 文件生成成功: generator.bdf");
    Ok(())
}
