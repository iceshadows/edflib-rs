use std::{
    ops::Deref,
    os::raw::c_int,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::utils::*;
use anyhow::{anyhow, Result};
use derive_new::new;
use edflib_sys::*;

pub enum Filetype {
    EDF,
    BDF,
}

impl Filetype {
    fn from(ext: &str) -> Self {
        match ext {
            "edf" => Filetype::EDF,
            "bdf" => Filetype::BDF,
            _ => Filetype::EDF,
        }
    }
    fn as_str(&self) -> &str {
        match self {
            Filetype::EDF => "edf",
            Filetype::BDF => "bdf",
        }
    }
}

pub enum AnnotationPosition {
    Start,
    Middle,
    End,
}

impl AnnotationPosition {
    fn to_raw(&self) -> i32 {
        (match self {
            AnnotationPosition::Start => EDF_ANNOT_IDX_POS_START,
            AnnotationPosition::Middle => EDF_ANNOT_IDX_POS_MIDDLE,
            AnnotationPosition::End => EDF_ANNOT_IDX_POS_END,
        }) as i32
    }
}

#[derive(new)]
struct Inner {
    #[new(value = "0")]
    hdl: i32,
    #[new(value = "Filetype::EDF")]
    filetype: Filetype,
}

#[derive(new)]
pub struct Edf {
    path: PathBuf,
    #[new(value = "Arc::new(Mutex::new(Inner::new()))")]
    inner: Arc<Mutex<Inner>>,
    pub number_of_signals: i32,
}

impl Edf {
    fn get_hdl(&self) -> i32 {
        let inner = self.inner.lock().unwrap();
        inner.hdl
    }
    pub fn get_edflib_version() -> String {
        let version = unsafe { edflib_version() };
        version.to_string()
    }
    pub fn open_file_writeonly(&self) -> Result<()> {
        let path = PathBuf::from(self.path.to_str().unwrap());
        let ext = path.extension().unwrap().to_str().unwrap();
        let filetype = Filetype::from(ext);

        let path = str_to_char(path.to_str().unwrap());
        let mut inner = self.inner.lock().unwrap();

        let filetype = match filetype {
            Filetype::EDF => EDFLIB_FILETYPE_EDFPLUS as c_int,
            Filetype::BDF => EDFLIB_FILETYPE_BDFPLUS as c_int,
        };
        let hdl = unsafe { edfopen_file_writeonly(path, filetype, self.number_of_signals) };
        inner.hdl = hdl;

        if hdl < 0 {
            let msg = format!(
                "Can not open file \"{}\"for writing",
                self.path.to_str().unwrap()
            );
            Err(anyhow!(msg))
        } else {
            Ok(())
        }
    }

    pub fn finish(&self) -> Result<()> {
        let result = unsafe { edfclose_file(self.get_hdl()) };

        if result < 0 {
            Err(anyhow!("Error finishing and closing the file"))
        } else {
            Ok(())
        }
    }

    pub fn set_patientname(&self, patientname: String) -> Result<()> {
        let patientname = str_to_char(patientname.as_str());
        let result = unsafe { edf_set_patientname(self.get_hdl(), patientname) };

        if result < 0 {
            Err(anyhow!("Error setting set_patientname"))
        } else {
            Ok(())
        }
    }

    pub fn set_patientcode(&self, patientcode: String) -> Result<()> {
        let patientcode = str_to_char(patientcode.as_str());
        let result = unsafe { edf_set_patientcode(self.get_hdl(), patientcode) };

        if result < 0 {
            Err(anyhow!("Error setting set_patientcode"))
        } else {
            Ok(())
        }
    }

    pub fn set_admincode(&self, admincode: String) -> Result<()> {
        let admincode = str_to_char(admincode.as_str());
        let result = unsafe { edf_set_admincode(self.get_hdl(), admincode) };
        if result < 0 {
            Err(anyhow!("Error setting set_admincode"))
        } else {
            Ok(())
        }
    }

    pub fn set_technician(&self, technician: String) -> Result<()> {
        let technician = str_to_char(technician.as_str());
        let result = unsafe { edf_set_technician(self.get_hdl(), technician) };

        if result < 0 {
            Err(anyhow!("Error setting set_technician"))
        } else {
            Ok(())
        }
    }

    pub fn set_sex(&self, sex: i32) -> Result<()> {
        let result = unsafe { edf_set_sex(self.get_hdl(), sex) };
        if result < 0 {
            Err(anyhow!("Error setting set_sex"))
        } else {
            Ok(())
        }
    }
    pub fn set_birthdate(&self, birthdate: i32) -> Result<()> {
        //TODO: check if this is correct
        panic!("Not implemented")
    }

    pub fn set_startdatetime(&self, startdatetime: i32) -> Result<()> {
        //TODO: check if this is correct
        panic!("Not implemented")
    }

    pub fn set_transducer(&self, edfsignal: i32, transducer: String) -> Result<()> {
        let transducer = str_to_char(transducer.as_str());
        let result = unsafe { edf_set_transducer(self.get_hdl(), edfsignal, transducer) };

        if result < 0 {
            Err(anyhow!("Error setting set_transducer"))
        } else {
            Ok(())
        }
    }

    pub fn set_samplefrequency(&self, edfsignal: i32, samplefrequency: i32) -> Result<()> {
        let result = unsafe { edf_set_samplefrequency(self.get_hdl(), edfsignal, samplefrequency) };
        if result < 0 {
            Err(anyhow!("Error setting set_samplefrequency"))
        } else {
            Ok(())
        }
    }

    pub fn set_digital_maximum(&self, edfsignal: i32, dig_max: i32) -> Result<()> {
        let result = unsafe { edf_set_digital_maximum(self.get_hdl(), edfsignal, dig_max) };

        if result < 0 {
            Err(anyhow!("Error setting set_digital_maximum"))
        } else {
            Ok(())
        }
    }
    pub fn set_physical_maximum(&self, edfsignal: i32, dig_max: f64) -> Result<()> {
        let result = unsafe { edf_set_physical_maximum(self.get_hdl(), edfsignal, dig_max) };

        if result < 0 {
            Err(anyhow!("Error setting set_physical_maximum"))
        } else {
            Ok(())
        }
    }

    pub fn set_physical_minimum(&self, edfsignal: i32, dig_max: f64) -> Result<()> {
        let result = unsafe { edf_set_physical_minimum(self.get_hdl(), edfsignal, dig_max) };

        if result < 0 {
            Err(anyhow!("Error setting set_physical_minimum"))
        } else {
            Ok(())
        }
    }

    pub fn set_digital_minimum(&self, edfsignal: i32, dig_min: i32) -> Result<()> {
        let result = unsafe { edf_set_digital_minimum(self.get_hdl(), edfsignal, dig_min) };

        if result < 0 {
            Err(anyhow!("Error setting set_digital_minimum"))
        } else {
            Ok(())
        }
    }

    pub fn set_physical_dimension(&self, edfsignal: i32, phys_dim: String) -> Result<()> {
        let phys_dim = str_to_char(phys_dim.as_str());
        let result = unsafe { edf_set_physical_dimension(self.get_hdl(), edfsignal, phys_dim) };

        if result < 0 {
            Err(anyhow!("Error setting set_physical_dimension"))
        } else {
            Ok(())
        }
    }

    pub fn set_label(&self, edfsignal: i32, label: String) -> Result<()> {
        let label = str_to_char(label.as_str());
        let result = unsafe { edf_set_label(self.get_hdl(), edfsignal, label) };

        if result < 0 {
            Err(anyhow!("Error setting set_label"))
        } else {
            Ok(())
        }
    }

    pub fn set_equipment(&self, equipment: String) -> Result<()> {
        let equipment = str_to_char(equipment.as_str());
        let result = unsafe { edf_set_equipment(self.get_hdl(), equipment) };

        if result < 0 {
            Err(anyhow!("Error setting set_equipment"))
        } else {
            Ok(())
        }
    }

    pub fn set_recording_additional(&self, recording_additional: String) -> Result<()> {
        let recording_additional = str_to_char(recording_additional.as_str());
        let result = unsafe { edf_set_recording_additional(self.get_hdl(), recording_additional) };

        if result < 0 {
            Err(anyhow!("Error setting set_recording_additional"))
        } else {
            Ok(())
        }
    }
    pub fn set_recordingduration(&self, duration: Duration) -> Result<()> {
        // Convert duration from seconds to 10 microseconds
        let duration_in_10_microseconds =
            (duration.as_secs() as i64 * 100000 + duration.subsec_micros() as i64 / 100) as i32;

        // Ensure the duration is within the valid range
        if duration_in_10_microseconds < 100 || duration_in_10_microseconds > 6000000 {
            return Err(anyhow!(
                "Datarecord duration must be in the range 0.001 to 60 seconds"
            ));
        }

        let result =
            unsafe { edf_set_datarecord_duration(self.get_hdl(), duration_in_10_microseconds) };
        if result < 0 {
            Err(anyhow!("Error setting datarecord duration"))
        } else {
            Ok(())
        }
    }

    pub fn set_annot_chan_idx_pos(&self, position: AnnotationPosition) -> Result<()> {
        let result = unsafe { edf_set_annot_chan_idx_pos(self.get_hdl(), position.to_raw()) };

        if result < 0 {
            Err(anyhow!("Error setting set_annot_chan_idx_pos"))
        } else {
            Ok(())
        }
    }

    pub fn set_number_of_annotation_signals(&self, annot_signals: usize) -> Result<()> {
        let result =
            unsafe { edf_set_number_of_annotation_signals(self.get_hdl(), annot_signals as i32) };

        if result < 0 {
            Err(anyhow!("Error setting set_number_of_annotation_signals"))
        } else {
            Ok(())
        }
    }

    pub fn write_samples(&self, samples: &mut Vec<f64>, samplefrequency: usize) -> Result<()> {
        if (samples.len() / samplefrequency != 1) {
            return Err(anyhow!(
                "samples length must be a full sample of samplefrequency"
            ));
        }
        // 分批写入样本
        for chunk in samples.chunks_mut(samplefrequency) {
            let buf: *mut f64 = chunk.as_mut_ptr().cast::<f64>();
            let result = unsafe { edfwrite_physical_samples(self.get_hdl(), buf) };
            if result < 0 {
                return Err(anyhow!("Error writing samples"));
            }
        }
        Ok(())
    }

    pub fn write_annotation(&self, onset: i64, duration: i64, description: String) -> Result<()> {
        let description = str_to_char(description.as_str());
        let result =
            unsafe { edfwrite_annotation_latin1_hr(self.get_hdl(), onset, duration, description) };

        if result < 0 {
            Err(anyhow!("Error write_annotation"))
        } else {
            Ok(())
        }
    }
}
