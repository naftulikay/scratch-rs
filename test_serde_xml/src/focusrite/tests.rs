use crate::focusrite::{AnalogOutput, Device, LoopbackOutput, Output, SpdifRcaOutput};

use serde_xml::Error;

#[allow(unused)]
const FF_PROFILE_DEFAULT_XML: &'static str = include_str!("fixtures/default-18i8-profile.ff.xml");

#[test]
fn test_deserialize_profile() -> Result<(), Error> {
    let profile = serde_xml::from_str::<Device>(
        r#"
        <device timestamp="1688712542">
            <outputs>
                <analogue stereo="true" mute="false" source="Mix A" nickname="" gain="-16" hardware-control="true"/>
                <analogue stereo="true" mute="false" nickname="" gain="-16" hardware-control="true"/>
                <spdif-rca stereo="true" mute="false" source="Playback 1" nickname=""/>
                <spdif-rca stereo="true" mute="false" nickname=""/>
                <loopback stereo="true" mute="false" source="Playback 1" nickname=""/>
                <loopback stereo="true" mute="false" nickname=""/>
            </outputs>
            <record-outputs>
                <record hardware-input="0" hardware-input-m="0" hardware-input-h="0"/>
                <record/>                
            </record-outputs>
            <inputs>
                <analogue nickname="" air="false" mode="Line" pad="false"/>
                <analogue nickname=""/>
                <spdif-rca nickname=""/>
                <adat nickname=""/>
                <playback nickname=""/>
            </inputs>
            <settings spdif-mode="RCA" phantom-persistence="true"/>
            <clocking/>
            <monitoring preset="1-2"/>
        </device>
    "#,
    )?;

    assert_eq!(2, profile.record_outputs().len());

    Ok(())
}

/// Tests for output definitions.
mod outputs {
    use super::{AnalogOutput, Error, LoopbackOutput, Output, SpdifRcaOutput};

    #[test]
    fn test_deserialize_analog_output() -> Result<(), Error> {
        // with all optional fields
        serde_xml::from_str::<AnalogOutput>(
            r#"<analogue stereo="true" mute="false" source="Mix A" nickname="" gain="-16" hardware-control="true"/>"#,
        )?;

        // without source
        serde_xml::from_str::<AnalogOutput>(
            r#"<analogue stereo="true" mute="false" nickname="" gain="-16" hardware-control="true"/>"#,
        )?;

        Ok(())
    }

    #[test]
    fn test_deserialize_spdif_rca_output() -> Result<(), Error> {
        // with source
        serde_xml::from_str::<SpdifRcaOutput>(
            r#"<spdif-rca stereo="true" mute="false" source="Playback 1" nickname=""/>"#,
        )?;

        // without source
        serde_xml::from_str::<SpdifRcaOutput>(
            r#"<spdif-rca stereo="true" mute="false" nickname=""/>"#,
        )?;

        Ok(())
    }

    #[test]
    fn test_deserialize_loopback_output() -> Result<(), Error> {
        // with source
        serde_xml::from_str::<LoopbackOutput>(
            r#"<loopback stereo="true" mute="false" source="Playback 1" nickname=""/>"#,
        )?;

        // without source
        serde_xml::from_str::<LoopbackOutput>(
            r#"<loopback stereo="true" mute="false" nickname=""/>"#,
        )?;

        Ok(())
    }

    #[test]
    fn test_deserialize_outputs() -> Result<(), Error> {
        #[derive(Debug, serde::Deserialize)]
        struct ContainsOutputs {
            #[allow(unused)]
            #[serde(rename = "$value")]
            outputs: Vec<Output>,
        }

        serde_xml::from_str::<ContainsOutputs>(
            r#"
        <outputs>
            <analogue stereo="true" mute="false" source="Mix A" nickname="" gain="-16" hardware-control="true"/>
            <analogue stereo="true" mute="false" nickname="" gain="-16" hardware-control="true"/>
            <spdif-rca stereo="true" mute="false" source="Playback 1" nickname=""/>
            <spdif-rca stereo="true" mute="false" nickname=""/>
            <loopback stereo="true" mute="false" source="Playback 1" nickname=""/>
            <loopback stereo="true" mute="false" nickname=""/>
        </outputs>
        "#,
        )?;

        Ok(())
    }
}

/// Tests for record outputs.
mod record_outputs {
    use serde_xml::Error;

    use crate::focusrite::{RecordOutput, RecordOutputs};

    #[test]
    fn test_deserialize_record_outputs() -> Result<(), Error> {
        let outputs = serde_xml::from_str::<RecordOutputs>(
            r#"
                <record-outputs>
                    <record hardware-input="0" hardware-input-m="0" hardware-input-h="0"/>
                    <record/>
                </record-outputs>
            "#,
        )?;

        assert_eq!(
            2,
            outputs.len(),
            "Failed to deserialize two child record output items."
        );

        Ok(())
    }

    #[test]
    fn test_deserialize_record_output() -> Result<(), Error> {
        let output = serde_xml::from_str::<RecordOutput>(
            r#"<record hardware-input="0" hardware-input-m="0" hardware-input-h="0"/>"#,
        )?;

        assert_eq!(
            Some(0),
            output.hardware_input,
            "Failed to deserialize hardware-input field."
        );

        assert_eq!(
            Some(0),
            output.hardware_input_m,
            "Failed to deserialize hardware-input-m field"
        );

        assert_eq!(
            Some(0),
            output.hardware_input_h,
            "Failed to deserialize hardware-input-h field"
        );

        Ok(())
    }

    #[test]
    fn test_deserialize_record_output_empty() -> Result<(), Error> {
        serde_xml::from_str::<RecordOutput>(r#"<record/>"#)?;

        Ok(())
    }
}

/// Tests for inputs.
mod inputs {
    use crate::focusrite::{AdatInput, AnalogInput, DeviceInputs, PlaybackInput, SpdifRcaInput};
    use serde_xml::Error;

    #[test]
    fn test_deserialize_device_inputs() -> Result<(), Error> {
        serde_xml::from_str::<DeviceInputs>(
            r#"
            <inputs>
                <analogue nickname="" air="false" mode="Line" pad="false"/>
                <analogue nickname=""/>
                <spdif-rca nickname=""/>
                <adat nickname=""/>
                <playback nickname=""/>
            </inputs>
            "#,
        )?;

        Ok(())
    }

    #[test]
    fn deserialize_analog_input() -> Result<(), Error> {
        // input with air/pad/mode config
        serde_xml::from_str::<AnalogInput>(
            r#"<analogue nickname="" air="false" mode="Line" pad="false"/>"#,
        )?;

        // alt input with inst mode
        serde_xml::from_str::<AnalogInput>(
            r#"<analogue nickname="" air="false" mode="Inst" pad="false"/>"#,
        )?;

        // empty input
        serde_xml::from_str::<AnalogInput>(r#"<analogue nickname=""/>"#)?;

        Ok(())
    }

    #[test]
    fn deserialize_spdif_rca_input() -> Result<(), Error> {
        serde_xml::from_str::<SpdifRcaInput>(r#"<spdif-rca nickname=""/>"#)?;

        Ok(())
    }

    #[test]
    fn deserialize_adat_input() -> Result<(), Error> {
        serde_xml::from_str::<AdatInput>(r#"<adat nickname=""/>"#)?;

        Ok(())
    }

    #[test]
    fn deserialize_playback_input() -> Result<(), Error> {
        serde_xml::from_str::<PlaybackInput>(r#"<playback nickname=""/>"#)?;

        Ok(())
    }
}

/// Tests for mixes.
mod mixes {}

/// Tests for mix inputs.
mod mixer_inputs {}

/// Tests for device settings.
mod device_settings {
    use crate::focusrite::DeviceSettings;
    use serde_xml::Error;

    #[test]
    fn test_deserialize_device_settings() -> Result<(), Error> {
        serde_xml::from_str::<DeviceSettings>(
            r#"<settings spdif-mode="RCA" phantom-persistence="true"/>"#,
        )?;

        // try empty
        serde_xml::from_str::<DeviceSettings>(r#"<settings/>"#)?;

        Ok(())
    }
}

/// Tests for clocking.
mod clocking {
    use crate::focusrite::DeviceClocking;
    use serde_xml::Error;

    #[test]
    fn test_deserialize_device_clocking() -> Result<(), Error> {
        serde_xml::from_str::<DeviceClocking>(r#"<clocking/>"#)?;

        Ok(())
    }
}

/// Tests for monitoring.
mod monitoring {
    use crate::focusrite::DeviceMonitoring;
    use serde_xml::Error;

    #[test]
    fn test_deserialize_device_monitoring() -> Result<(), Error> {
        serde_xml::from_str::<DeviceMonitoring>(r#"<monitoring preset="1-2"/>"#)?;

        Ok(())
    }
}
