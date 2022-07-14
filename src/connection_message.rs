use leap_sys::LEAP_CONNECTION_MESSAGE;

use crate::event::Event;

pub struct ConnectionMessage {
    handle: LEAP_CONNECTION_MESSAGE,
}

impl From<LEAP_CONNECTION_MESSAGE> for ConnectionMessage {
    fn from(handle: LEAP_CONNECTION_MESSAGE) -> Self {
        Self { handle }
    }
}

impl ConnectionMessage {
    pub fn get_event(&self) -> Event {
        unsafe {
            return match self.handle.type_ {
                leap_sys::_eLeapEventType_eLeapEventType_None => Event::NoEvent,
                leap_sys::_eLeapEventType_eLeapEventType_Connection => {
                    Event::ConnectionEvent(&*self.handle.__bindgen_anon_1.connection_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_ConnectionLost => {
                    Event::ConnectionLostEvent(&*self.handle.__bindgen_anon_1.connection_lost_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_Device => {
                    Event::DeviceEvent(&*self.handle.__bindgen_anon_1.device_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_DeviceFailure => {
                    Event::DeviceFailureEvent(&*self.handle.__bindgen_anon_1.device_failure_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_Policy => {
                    Event::PolicyEvent(&*self.handle.__bindgen_anon_1.policy_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_Tracking => {
                    Event::TrakingEvent(&*self.handle.__bindgen_anon_1.tracking_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_ImageRequestError => {
                    Event::ImageRequestError
                }
                leap_sys::_eLeapEventType_eLeapEventType_ImageComplete => Event::ImageComplete,
                leap_sys::_eLeapEventType_eLeapEventType_LogEvent => {
                    Event::LogEvent(&*self.handle.__bindgen_anon_1.log_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_DeviceLost => Event::DeviceLost,
                leap_sys::_eLeapEventType_eLeapEventType_ConfigResponse => {
                    Event::ConfigResponseEvent(&*self.handle.__bindgen_anon_1.config_response_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_ConfigChange => {
                    Event::LeapConfigChangeEvent(&*self.handle.__bindgen_anon_1.config_change_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_DeviceStatusChange => {
                    Event::DeviceStatusChangeEvent(
                        &*self.handle.__bindgen_anon_1.device_status_change_event,
                    )
                }
                leap_sys::_eLeapEventType_eLeapEventType_DroppedFrame => {
                    Event::DroppedFrameEvent(&*self.handle.__bindgen_anon_1.dropped_frame_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_Image => {
                    Event::ImageEvent(&*self.handle.__bindgen_anon_1.image_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_PointMappingChange => {
                    Event::PointMappingChangeEvent(
                        &*self.handle.__bindgen_anon_1.point_mapping_change_event,
                    )
                }
                leap_sys::_eLeapEventType_eLeapEventType_TrackingMode => {
                    Event::TrackingModeEvent(&*self.handle.__bindgen_anon_1.tracking_mode_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_LogEvents => {
                    Event::LogEvents(&*self.handle.__bindgen_anon_1.log_events)
                }
                leap_sys::_eLeapEventType_eLeapEventType_HeadPose => {
                    Event::HeadPoseEvent(&*self.handle.__bindgen_anon_1.head_pose_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_Eyes => {
                    Event::EyeEvent(&*self.handle.__bindgen_anon_1.eye_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_IMU => {
                    Event::IMUEvent(&*self.handle.__bindgen_anon_1.imu_event)
                }
                event_code => Event::Unknown(event_code),
            };
        };
    }
}
