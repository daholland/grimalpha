pub struct InputState {
    pub mouse: MouseState,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MouseState {
    pub pos: (i32, i32),
    pub buttons: (bool, bool, bool, bool, bool),
}

#[cfg(feature = "xinput")]
mod xinput {
    extern crate winapi;
    use super::winapi::*;


    #[link(name = "xinput")]
    extern "system" {
        pub fn XInputEnable(enable: BOOL);

        pub fn XInputGetAudioDeviceIds(dwUserIndex: DWORD,
                                       pRenderDeviceId: LPWSTR,
                                       pRenderCount: *mut UINT,
                                       pCaptureDeviceId: LPWSTR,
                                       pCaptureCount: *mut UINT)
                                       -> DWORD;

        pub fn XInputGetBatteryInformation(dwUserIndex: DWORD,
                                           devType: BYTE,
                                           pBatteryInformation: *mut XINPUT_BATTERY_INFORMATION)
                                           -> DWORD;

        pub fn XInputGetCapabilities(dwUserIndex: DWORD,
                                     dwFlags: DWORD,
                                     pCapabilities: *mut XINPUT_CAPABILITIES)
                                     -> DWORD;

        pub fn XInputGetDSoundAudioDeviceGuids(dwUserIndex: DWORD,
                                               pDSoundRenderGuid: *mut GUID,
                                               pDSoundCaptureGuid: *mut GUID)
                                               -> DWORD;

        pub fn XInputGetKeystroke(dwUserIndex: DWORD,
                                  dwReserved: DWORD,
                                  pKeystroke: PXINPUT_KEYSTROKE)
                                  -> DWORD;

        pub fn XInputGetState(dwUserIndex: DWORD, pState: *mut XINPUT_STATE) -> DWORD;

        pub fn XInputSetState(dwUserIndex: DWORD, pVibration: *mut XINPUT_VIBRATION) -> DWORD;
    }

    pub struct JoyPadState(XINPUT_STATE);

    impl JoyPadState {
        pub fn new() -> XINPUT_STATE {
            XINPUT_STATE {
                dwPacketNumber: 0,
                Gamepad: XINPUT_GAMEPAD {
                    wButtons: 0, // WORD,
                    bLeftTrigger: 0, // BYTE,
                    bRightTrigger: 0, // BYTE,
                    sThumbLX: 0, // SHORT,
                    sThumbLY: 0, // SHORT,
                    sThumbRX: 0, // SHORT,
                    sThumbRY: 0, // SHORT,
                },
            }
        }
    }
}
