#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

mod bindings;

use std::{
    collections::HashMap,
    ffi::{CStr, CString, c_char, c_int, c_void},
    ptr,
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
};

use example_effects::{
    i18n,
    settings::{
        EnumValue, ExTrKey, SettingDescriptor, SettingID, SettingKind, Settings, SettingsList,
    },
    ColorAdjustment, ColorAdjustmentFullSettings, SolidColorBlend, SolidColorBlendFullSettings,
};

use bindings::*;

// SAFETY: The host promises not to mess with the raw string pointers in this struct
unsafe impl Send for OfxPlugin {}
unsafe impl Sync for OfxPlugin {}

// ---------------------------------------------------------------------------
// EffectKind trait — allows generic dispatch for both effect types
// ---------------------------------------------------------------------------

trait EffectKind: 'static {
    type FullSettings: Settings<Key = ExTrKey> + Clone + Default + 'static;

    fn shared() -> &'static SharedData<Self::FullSettings>;
    fn label_key() -> ExTrKey;
    fn plugin_identifier() -> &'static CStr;
    fn apply_effect(
        settings: &Self::FullSettings,
        src: &[u8],
        dst: &mut [u8],
        width: usize,
        height: usize,
    );
}

struct ColorAdjustmentKind;
struct SolidBlendKind;

static SHARED_CA: OnceLock<SharedData<ColorAdjustmentFullSettings>> = OnceLock::new();
static SHARED_SB: OnceLock<SharedData<SolidColorBlendFullSettings>> = OnceLock::new();

impl EffectKind for ColorAdjustmentKind {
    type FullSettings = ColorAdjustmentFullSettings;

    fn shared() -> &'static SharedData<Self::FullSettings> {
        SHARED_CA.get().expect("SharedData not initialized")
    }

    fn label_key() -> ExTrKey {
        ExTrKey::ParamColorAdjustmentName
    }

    fn plugin_identifier() -> &'static CStr {
        c"com.example:VideoFXExampleColorAdjustment"
    }

    fn apply_effect(
        settings: &Self::FullSettings,
        src: &[u8],
        dst: &mut [u8],
        width: usize,
        height: usize,
    ) {
        let effect: ColorAdjustment = settings.into();
        effect.apply_effect(src, dst, width, height);
    }
}

impl EffectKind for SolidBlendKind {
    type FullSettings = SolidColorBlendFullSettings;

    fn shared() -> &'static SharedData<Self::FullSettings> {
        SHARED_SB.get().expect("SharedData not initialized")
    }

    fn label_key() -> ExTrKey {
        ExTrKey::ParamSolidBlendName
    }

    fn plugin_identifier() -> &'static CStr {
        c"com.example:VideoFXExampleSolidBlend"
    }

    fn apply_effect(
        settings: &Self::FullSettings,
        src: &[u8],
        dst: &mut [u8],
        width: usize,
        height: usize,
    ) {
        let effect: SolidColorBlend = settings.into();
        effect.apply_effect(src, dst, width, height);
    }
}

// ---------------------------------------------------------------------------
// SharedData — one instance per effect kind
// ---------------------------------------------------------------------------

struct HostInfo {
    host: &'static OfxPropertySetStruct,
    fetch_suite: unsafe extern "C" fn(
        host: OfxPropertySetHandle,
        suiteName: *const c_char,
        suiteVersion: c_int,
    ) -> *const c_void,
}

struct SharedData<T: Settings> {
    host_info: HostInfo,
    property_suite: &'static OfxPropertySuiteV1,
    image_effect_suite: &'static OfxImageEffectSuiteV1,
    parameter_suite: &'static OfxParameterSuiteV1,
    settings_list: SettingsList<T>,
    supports_multiple_clip_depths: AtomicBool,
    strings: HashMap<SettingID<T>, (CString, CString, Option<CString>, Option<CString>)>,
    menu_item_strings: HashMap<(SettingID<T>, u32), (CString, Option<CString>)>,
}

type OfxResult<T> = Result<T, OfxStatus>;

impl<T: Settings<Key = ExTrKey> + Clone + Default> SharedData<T> {
    pub unsafe fn new(host_info: HostInfo) -> OfxResult<Self> {
        let property_suite = (host_info.fetch_suite)(
            host_info.host as *const _ as _,
            kOfxPropertySuite.as_ptr(),
            1,
        ) as *const OfxPropertySuiteV1;
        let image_effect_suite = (host_info.fetch_suite)(
            host_info.host as *const _ as _,
            kOfxImageEffectSuite.as_ptr(),
            1,
        ) as *const OfxImageEffectSuiteV1;
        let parameter_suite = (host_info.fetch_suite)(
            host_info.host as *const _ as _,
            kOfxParameterSuite.as_ptr(),
            1,
        ) as *const OfxParameterSuiteV1;

        let settings_list = SettingsList::<T>::new();
        let mut strings = HashMap::new();
        let mut menu_item_strings = HashMap::new();
        for descriptor in settings_list.all_descriptors() {
            let id = &descriptor.id;
            let id_str = CString::new(descriptor.id.name).unwrap();
            let label = CString::new(i18n::tr(descriptor.label_key)).unwrap();
            let description = descriptor
                .description_key
                .map(|k| CString::new(i18n::tr(k)).unwrap());
            let group_name = if let SettingKind::Group { .. } = descriptor.kind {
                Some(CString::new(format!("{}_group", descriptor.id.name)).unwrap())
            } else {
                None
            };
            strings.insert(id.clone(), (id_str, label, description, group_name));

            if let SettingKind::Enumeration { options } = &descriptor.kind {
                for menu_item in options {
                    let item_label = CString::new(i18n::tr(menu_item.label_key)).unwrap();
                    menu_item_strings.insert(
                        (id.clone(), menu_item.index),
                        (
                            item_label,
                            menu_item
                                .description_key
                                .map(|k| CString::new(i18n::tr(k)).unwrap()),
                        ),
                    );
                }
            }
        }

        Ok(SharedData {
            host_info,
            property_suite: property_suite
                .as_ref()
                .ok_or(OfxStat::kOfxStatErrMissingHostFeature)?,
            image_effect_suite: image_effect_suite
                .as_ref()
                .ok_or(OfxStat::kOfxStatErrMissingHostFeature)?,
            parameter_suite: parameter_suite
                .as_ref()
                .ok_or(OfxStat::kOfxStatErrMissingHostFeature)?,
            settings_list,
            supports_multiple_clip_depths: AtomicBool::new(false),
            strings,
            menu_item_strings,
        })
    }
}

// ---------------------------------------------------------------------------
// Plugin info — two plugins in one bundle
// ---------------------------------------------------------------------------

static PLUGIN_INFO_CA: OnceLock<OfxPlugin> = OnceLock::new();
static PLUGIN_INFO_SB: OnceLock<OfxPlugin> = OnceLock::new();

// ---------------------------------------------------------------------------
// Entry points
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn OfxGetNumberOfPlugins() -> c_int {
    2
}

#[unsafe(no_mangle)]
pub extern "C" fn OfxGetPlugin(nth: c_int) -> *const OfxPlugin {
    std::panic::set_hook(Box::new(|info| {
        println!("{info:?}");
    }));

    match nth {
        0 => {
            let plugin_info = PLUGIN_INFO_CA.get_or_init(|| OfxPlugin {
                pluginApi: kOfxImageEffectPluginApi.as_ptr(),
                apiVersion: 1,
                pluginIdentifier: ColorAdjustmentKind::plugin_identifier().as_ptr(),
                pluginVersionMajor: 0,
                pluginVersionMinor: 1,
                setHost: Some(set_host_info_ca),
                mainEntry: Some(main_entry_ca),
            });
            plugin_info as *const _
        }
        1 => {
            let plugin_info = PLUGIN_INFO_SB.get_or_init(|| OfxPlugin {
                pluginApi: kOfxImageEffectPluginApi.as_ptr(),
                apiVersion: 1,
                pluginIdentifier: SolidBlendKind::plugin_identifier().as_ptr(),
                pluginVersionMajor: 0,
                pluginVersionMinor: 1,
                setHost: Some(set_host_info_sb),
                mainEntry: Some(main_entry_sb),
            });
            plugin_info as *const _
        }
        _ => ptr::null(),
    }
}

// ---------------------------------------------------------------------------
// set_host_info — one per effect kind
// ---------------------------------------------------------------------------

unsafe fn set_host_info_inner<T: Settings<Key = ExTrKey> + Clone + Default>(
    host: *mut OfxHost,
    cell: &OnceLock<SharedData<T>>,
) -> OfxResult<()> {
    example_effects::i18n::set_lang(example_effects::i18n::detect_system_lang());
    if let Some(host_struct) = host.as_ref() {
        let host = host_struct.host.as_ref().ok_or(OfxStat::kOfxStatFailed)?;
        let fetch_suite = host_struct.fetchSuite.ok_or(OfxStat::kOfxStatFailed)?;
        let new_shared_data = SharedData::<T>::new(HostInfo { host, fetch_suite })?;
        cell.get_or_init(|| new_shared_data);
        Ok(())
    } else {
        Err(OfxStat::kOfxStatFailed)
    }
}

unsafe extern "C" fn set_host_info_ca(host: *mut OfxHost) {
    let _ = set_host_info_inner::<ColorAdjustmentFullSettings>(host, &SHARED_CA);
}

unsafe extern "C" fn set_host_info_sb(host: *mut OfxHost) {
    let _ = set_host_info_inner::<SolidColorBlendFullSettings>(host, &SHARED_SB);
}

// ---------------------------------------------------------------------------
// main_entry — one per effect kind
// ---------------------------------------------------------------------------

unsafe extern "C" fn main_entry_ca(
    action: *const c_char,
    handle: *const c_void,
    inArgs: OfxPropertySetHandle,
    outArgs: OfxPropertySetHandle,
) -> OfxStatus {
    main_entry_generic::<ColorAdjustmentKind>(action, handle, inArgs, outArgs)
}

unsafe extern "C" fn main_entry_sb(
    action: *const c_char,
    handle: *const c_void,
    inArgs: OfxPropertySetHandle,
    outArgs: OfxPropertySetHandle,
) -> OfxStatus {
    main_entry_generic::<SolidBlendKind>(action, handle, inArgs, outArgs)
}

unsafe fn main_entry_generic<K: EffectKind>(
    action: *const c_char,
    handle: *const c_void,
    inArgs: OfxPropertySetHandle,
    outArgs: OfxPropertySetHandle,
) -> OfxStatus {
    if action.is_null() {
        return OfxStat::kOfxStatFailed;
    }
    let effect = handle as OfxImageEffectHandle;
    let action = CStr::from_ptr(action);

    let return_status: OfxResult<()> = if action == kOfxActionLoad {
        action_load_generic::<K>()
    } else if action == kOfxActionDescribe {
        action_describe_generic::<K>(effect)
    } else if action == kOfxImageEffectActionDescribeInContext {
        action_describe_in_context_generic::<K>(effect)
    } else if action == kOfxImageEffectActionGetRegionsOfInterest {
        action_get_regions_of_interest_generic::<K>(effect, inArgs, outArgs)
    } else if action == kOfxImageEffectActionGetClipPreferences {
        action_get_clip_preferences_generic::<K>(outArgs)
    } else if action == kOfxActionCreateInstance || action == kOfxActionDestroyInstance {
        Ok(())
    } else if action == kOfxActionInstanceChanged {
        action_instance_changed_generic::<K>(effect, inArgs)
    } else if action == kOfxImageEffectActionRender {
        action_render_generic::<K>(effect, inArgs)
    } else {
        OfxResult::Err(OfxStat::kOfxStatReplyDefault)
    };

    match return_status {
        Ok(()) => OfxStat::kOfxStatOK,
        Err(e) => e,
    }
}

// ---------------------------------------------------------------------------
// Action handlers — generic over EffectKind
// ---------------------------------------------------------------------------

unsafe fn action_load_generic<K: EffectKind>() -> OfxResult<()> {
    let data = K::shared();
    let propGetInt = data
        .property_suite
        .propGetInt
        .ok_or(OfxStat::kOfxStatFailed)?;
    let mut supports_multiple_clip_depths: c_int = 0;
    propGetInt(
        data.host_info.host as *const _ as _,
        kOfxImageEffectPropSupportsMultipleClipDepths.as_ptr(),
        0,
        &mut supports_multiple_clip_depths,
    )
    .ofx_ok()?;
    data.supports_multiple_clip_depths
        .store(supports_multiple_clip_depths != 0, Ordering::Release);
    Ok(())
}

unsafe fn action_describe_generic<K: EffectKind>(
    descriptor: OfxImageEffectHandle,
) -> OfxResult<()> {
    let data = K::shared();
    let mut effectProps: OfxPropertySetHandle = ptr::null_mut();
    (data
        .image_effect_suite
        .getPropertySet
        .ok_or(OfxStat::kOfxStatFailed)?)(descriptor, &mut effectProps)
    .ofx_ok()?;

    let propSetString = data
        .property_suite
        .propSetString
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propSetInt = data
        .property_suite
        .propSetInt
        .ok_or(OfxStat::kOfxStatFailed)?;

    propSetString(
        effectProps,
        kOfxPropLabel.as_ptr(),
        0,
        i18n::tr_cstr(K::label_key()).as_ptr(),
    )
    .ofx_ok()?;

    propSetString(
        effectProps,
        kOfxImageEffectPluginPropGrouping.as_ptr(),
        0,
        c"VideoFX Example".as_ptr(),
    )
    .ofx_ok()?;

    propSetString(
        effectProps,
        kOfxImageEffectPropSupportedContexts.as_ptr(),
        0,
        kOfxImageEffectContextFilter.as_ptr(),
    )
    .ofx_ok()?;
    propSetString(
        effectProps,
        kOfxImageEffectPropSupportedContexts.as_ptr(),
        1,
        kOfxImageEffectContextGeneral.as_ptr(),
    )
    .ofx_ok()?;

    propSetString(
        effectProps,
        kOfxImageEffectPropSupportedPixelDepths.as_ptr(),
        0,
        kOfxBitDepthFloat.as_ptr(),
    )
    .ofx_ok()?;
    propSetString(
        effectProps,
        kOfxImageEffectPropSupportedPixelDepths.as_ptr(),
        1,
        kOfxBitDepthShort.as_ptr(),
    )
    .ofx_ok()?;
    propSetString(
        effectProps,
        kOfxImageEffectPropSupportedPixelDepths.as_ptr(),
        2,
        kOfxBitDepthByte.as_ptr(),
    )
    .ofx_ok()?;

    propSetString(
        effectProps,
        kOfxImageEffectPluginRenderThreadSafety.as_ptr(),
        0,
        kOfxImageEffectRenderFullySafe.as_ptr(),
    )
    .ofx_ok()?;
    propSetInt(
        effectProps,
        kOfxImageEffectPluginPropHostFrameThreading.as_ptr(),
        0,
        0,
    )
    .ofx_ok()?;
    propSetInt(effectProps, kOfxImageEffectPropSupportsTiles.as_ptr(), 0, 0).ofx_ok()?;

    Ok(())
}

unsafe fn action_describe_in_context_generic<K: EffectKind>(
    descriptor: OfxImageEffectHandle,
) -> OfxResult<()> {
    let data = K::shared();
    let clipDefine = data
        .image_effect_suite
        .clipDefine
        .ok_or(OfxStat::kOfxStatFailed)?;
    let getParamSet = data
        .image_effect_suite
        .getParamSet
        .ok_or(OfxStat::kOfxStatFailed)?;

    let propSetString = data
        .property_suite
        .propSetString
        .ok_or(OfxStat::kOfxStatFailed)?;

    // Output clip
    let mut props: OfxPropertySetHandle = ptr::null_mut();
    clipDefine(descriptor, c"Output".as_ptr(), &mut props).ofx_ok()?;
    if props.is_null() {
        return Err(OfxStat::kOfxStatFailed);
    }
    propSetString(
        props,
        kOfxImageEffectPropSupportedComponents.as_ptr(),
        0,
        kOfxImageComponentRGBA.as_ptr(),
    )
    .ofx_ok()?;
    propSetString(
        props,
        kOfxImageEffectPropSupportedComponents.as_ptr(),
        1,
        kOfxImageComponentRGB.as_ptr(),
    )
    .ofx_ok()?;

    // Source clip
    clipDefine(descriptor, c"Source".as_ptr(), &mut props).ofx_ok()?;
    if props.is_null() {
        return Err(OfxStat::kOfxStatFailed);
    }
    propSetString(
        props,
        kOfxImageEffectPropSupportedComponents.as_ptr(),
        0,
        kOfxImageComponentRGBA.as_ptr(),
    )
    .ofx_ok()?;
    propSetString(
        props,
        kOfxImageEffectPropSupportedComponents.as_ptr(),
        1,
        kOfxImageComponentRGB.as_ptr(),
    )
    .ofx_ok()?;

    // Parameter set — use generic map_params
    let mut param_set: OfxParamSetHandle = ptr::null_mut();
    getParamSet(descriptor, &mut param_set).ofx_ok()?;

    let defaults = <K::FullSettings>::default();
    map_params_generic::<K>(data, param_set, &K::shared().settings_list.setting_descriptors, &defaults, c"")?;

    Ok(())
}

unsafe fn action_get_regions_of_interest_generic<K: EffectKind>(
    effect: OfxImageEffectHandle,
    inArgs: OfxPropertySetHandle,
    outArgs: OfxPropertySetHandle,
) -> OfxResult<()> {
    let data = K::shared();
    let propGetDouble = data
        .property_suite
        .propGetDouble
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propSetDoubleN = data
        .property_suite
        .propSetDoubleN
        .ok_or(OfxStat::kOfxStatFailed)?;
    let clipGetHandle = data
        .image_effect_suite
        .clipGetHandle
        .ok_or(OfxStat::kOfxStatFailed)?;
    let clipGetRegionOfDefinition = data
        .image_effect_suite
        .clipGetRegionOfDefinition
        .ok_or(OfxStat::kOfxStatFailed)?;

    let mut sourceClip: OfxImageClipHandle = ptr::null_mut();
    clipGetHandle(effect, c"Source".as_ptr(), &mut sourceClip, ptr::null_mut()).ofx_ok()?;
    let mut sourceRoD = OfxRectD {
        x1: 0.0,
        x2: 0.0,
        y1: 0.0,
        y2: 0.0,
    };
    let mut time: OfxTime = 0.0;
    propGetDouble(inArgs, kOfxPropTime.as_ptr(), 0, &mut time).ofx_ok()?;
    clipGetRegionOfDefinition(sourceClip, time, &mut sourceRoD).ofx_ok()?;

    propSetDoubleN(
        outArgs,
        c"OfxImageClipPropRoI_Source".as_ptr(),
        4,
        ptr::addr_of_mut!(sourceRoD) as *mut _,
    )
    .ofx_ok()?;

    Ok(())
}

unsafe fn action_get_clip_preferences_generic<K: EffectKind>(
    outArgs: OfxPropertySetHandle,
) -> OfxResult<()> {
    let data = K::shared();
    let propSetInt = data
        .property_suite
        .propSetInt
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propSetString = data
        .property_suite
        .propSetString
        .ok_or(OfxStat::kOfxStatFailed)?;

    propSetInt(outArgs, kOfxImageEffectFrameVarying.as_ptr(), 0, 1).ofx_ok()?;
    propSetString(
        outArgs,
        kOfxImageEffectPropPreMultiplication.as_ptr(),
        0,
        kOfxImageOpaque.as_ptr(),
    )
    .ofx_ok()?;

    Ok(())
}

unsafe fn action_instance_changed_generic<K: EffectKind>(
    _effect: OfxImageEffectHandle,
    inArgs: OfxPropertySetHandle,
) -> OfxResult<()> {
    let data = K::shared();
    let propGetInt = data
        .property_suite
        .propGetInt
        .ok_or(OfxStat::kOfxStatFailed)?;

    let mut reason: c_int = 0;
    propGetInt(inArgs, kOfxPropChangeReason.as_ptr(), 0, &mut reason).ofx_ok()?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Render — generic over EffectKind
// ---------------------------------------------------------------------------

unsafe fn action_render_generic<K: EffectKind>(
    effect: OfxImageEffectHandle,
    inArgs: OfxPropertySetHandle,
) -> OfxResult<()> {
    let data = K::shared();

    let clipGetHandle = data
        .image_effect_suite
        .clipGetHandle
        .ok_or(OfxStat::kOfxStatFailed)?;
    let clipGetImage = data
        .image_effect_suite
        .clipGetImage
        .ok_or(OfxStat::kOfxStatFailed)?;
    let clipReleaseImage = data
        .image_effect_suite
        .clipReleaseImage
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propGetPointer = data
        .property_suite
        .propGetPointer
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propGetInt = data
        .property_suite
        .propGetInt
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propGetDouble = data
        .property_suite
        .propGetDouble
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propGetString = data
        .property_suite
        .propGetString
        .ok_or(OfxStat::kOfxStatFailed)?;

    let getParamSet = data
        .image_effect_suite
        .getParamSet
        .ok_or(OfxStat::kOfxStatFailed)?;

    // Read time
    let mut time: OfxTime = 0.0;
    propGetDouble(inArgs, kOfxPropTime.as_ptr(), 0, &mut time).ofx_ok()?;

    // Read effect parameters
    let mut param_set: OfxParamSetHandle = ptr::null_mut();
    getParamSet(effect, &mut param_set).ofx_ok()?;

    let mut settings = <K::FullSettings>::default();
    apply_params_generic::<K>(data, param_set, time, &data.settings_list.setting_descriptors, &mut settings)?;

    // Get clip handles
    let mut srcClip: OfxImageClipHandle = ptr::null_mut();
    clipGetHandle(effect, c"Source".as_ptr(), &mut srcClip, ptr::null_mut()).ofx_ok()?;
    let mut dstClip: OfxImageClipHandle = ptr::null_mut();
    clipGetHandle(effect, c"Output".as_ptr(), &mut dstClip, ptr::null_mut()).ofx_ok()?;

    // Get images
    let mut srcImg: OfxPropertySetHandle = ptr::null_mut();
    clipGetImage(srcClip, time, ptr::null(), &mut srcImg).ofx_ok()?;
    let mut dstImg: OfxPropertySetHandle = ptr::null_mut();
    clipGetImage(dstClip, time, ptr::null(), &mut dstImg).ofx_ok()?;

    // Get image data pointers
    let mut srcPtr: *mut c_void = ptr::null_mut();
    propGetPointer(srcImg, kOfxImagePropData.as_ptr(), 0, &mut srcPtr).ofx_ok()?;
    let mut dstPtr: *mut c_void = ptr::null_mut();
    propGetPointer(dstImg, kOfxImagePropData.as_ptr(), 0, &mut dstPtr).ofx_ok()?;

    // Get row bytes and bounds
    let mut srcRowBytes: c_int = 0;
    let mut dstRowBytes: c_int = 0;
    propGetInt(srcImg, kOfxImagePropRowBytes.as_ptr(), 0, &mut srcRowBytes).ofx_ok()?;
    propGetInt(dstImg, kOfxImagePropRowBytes.as_ptr(), 0, &mut dstRowBytes).ofx_ok()?;

    let mut left: c_int = 0; let mut bottom: c_int = 0;
    let mut right: c_int = 0; let mut top: c_int = 0;
    propGetInt(srcImg, kOfxImagePropBounds.as_ptr(), 0, &mut left).ofx_ok()?;
    propGetInt(srcImg, kOfxImagePropBounds.as_ptr(), 1, &mut bottom).ofx_ok()?;
    propGetInt(srcImg, kOfxImagePropBounds.as_ptr(), 2, &mut right).ofx_ok()?;
    propGetInt(srcImg, kOfxImagePropBounds.as_ptr(), 3, &mut top).ofx_ok()?;

    let width = (right - left) as usize;
    let height = (top - bottom) as usize;
    let src_stride = srcRowBytes.max(0) as usize;
    let dst_stride = dstRowBytes.max(0) as usize;

    let mut depth_ptr: *mut c_char = ptr::null_mut();
    let depth = (|| {
        propGetString(srcImg, kOfxImageEffectPropPixelDepth.as_ptr(), 0, &mut depth_ptr)
            .ofx_ok()
            .ok()?;
        let s = CStr::from_ptr(depth_ptr);
        if s == kOfxBitDepthFloat {
            Some(16usize)
        } else if s == kOfxBitDepthShort {
            Some(8usize)
        } else if s == kOfxBitDepthByte {
            Some(4usize)
        } else {
            None
        }
    })()
    .unwrap_or(4);

    let row_bytes_u8 = width * 4;
    let total_u8 = row_bytes_u8 * height;
    let mut src_buf = vec![0u8; total_u8];
    let mut dst_buf = vec![0u8; total_u8];

    match depth {
        4 => {
            for y in 0..height {
                ptr::copy_nonoverlapping(
                    (srcPtr as *const u8).add(y * src_stride),
                    src_buf.as_mut_ptr().add(y * row_bytes_u8),
                    row_bytes_u8,
                );
            }
        }
        8 => {
            for y in 0..height {
                let host_row = (srcPtr as *const u8).add(y * src_stride) as *const u16;
                let u8_row = src_buf.as_mut_ptr().add(y * row_bytes_u8);
                for x in 0..(width * 4) {
                    let v = *host_row.add(x) as u32;
                    *u8_row.add(x) = ((v * 255 + 32767) / 65535) as u8;
                }
            }
        }
        _ => {
            for y in 0..height {
                let host_row = (srcPtr as *const u8).add(y * src_stride) as *const f32;
                let u8_row = src_buf.as_mut_ptr().add(y * row_bytes_u8);
                for x in 0..(width * 4) {
                    let v = *host_row.add(x);
                    *u8_row.add(x) = (v.clamp(0.0, 1.0) * 255.0).round() as u8;
                }
            }
        }
    }

    // OFX hosts (e.g. VEGAS) use premultiplied alpha; effect works in straight.
    // Convert premultiplied → straight before, and straight → premultiplied after.
    {
        for px in src_buf.chunks_exact_mut(4) {
            let a_byte = px[3] as f32;
            if a_byte > 0.0 {
                let recip = 255.0 / a_byte;
                px[0] = (px[0] as f32 * recip).clamp(0.0, 255.0).round() as u8;
                px[1] = (px[1] as f32 * recip).clamp(0.0, 255.0).round() as u8;
                px[2] = (px[2] as f32 * recip).clamp(0.0, 255.0).round() as u8;
            }
        }
    }

    K::apply_effect(&settings, &src_buf, &mut dst_buf, width, height);

    {
        for px in dst_buf.chunks_exact_mut(4) {
            let a = px[3] as f32 * (1.0 / 255.0);
            px[0] = (px[0] as f32 * a).clamp(0.0, 255.0).round() as u8;
            px[1] = (px[1] as f32 * a).clamp(0.0, 255.0).round() as u8;
            px[2] = (px[2] as f32 * a).clamp(0.0, 255.0).round() as u8;
        }
    }

    match depth {
        4 => {
            for y in 0..height {
                ptr::copy_nonoverlapping(
                    dst_buf.as_ptr().add(y * row_bytes_u8),
                    (dstPtr as *mut u8).add(y * dst_stride),
                    row_bytes_u8,
                );
            }
        }
        8 => {
            for y in 0..height {
                let u8_row = dst_buf.as_ptr().add(y * row_bytes_u8);
                let host_row = (dstPtr as *mut u8).add(y * dst_stride) as *mut u16;
                for x in 0..(width * 4) {
                    let v = *u8_row.add(x) as u16;
                    *host_row.add(x) = (v << 8) | v;
                }
            }
        }
        _ => {
            for y in 0..height {
                let u8_row = dst_buf.as_ptr().add(y * row_bytes_u8);
                let host_row = (dstPtr as *mut u8).add(y * dst_stride) as *mut f32;
                for x in 0..(width * 4) {
                    *host_row.add(x) = *u8_row.add(x) as f32 / 255.0;
                }
            }
        }
    }

    clipReleaseImage(srcImg).ofx_ok()?;
    clipReleaseImage(dstImg).ofx_ok()?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Parameter mapping helpers — generic
// ---------------------------------------------------------------------------

unsafe fn map_params_generic<K: EffectKind>(
    data: &SharedData<<K as EffectKind>::FullSettings>,
    param_set: OfxParamSetHandle,
    setting_descriptors: &[SettingDescriptor<<K as EffectKind>::FullSettings>],
    default_settings: &<K as EffectKind>::FullSettings,
    parent: &CStr,
) -> OfxResult<()> {
    let paramDefine = data
        .parameter_suite
        .paramDefine
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propSetDouble = data
        .property_suite
        .propSetDouble
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propSetInt = data
        .property_suite
        .propSetInt
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propSetString = data
        .property_suite
        .propSetString
        .ok_or(OfxStat::kOfxStatFailed)?;

    for descriptor in setting_descriptors {
        let mut paramProps: OfxPropertySetHandle = ptr::null_mut();
        let descriptor_strings = data.strings.get(&descriptor.id).unwrap();
        let descriptor_id_cstr = descriptor_strings.0.as_c_str();

        match &descriptor.kind {
            SettingKind::Enumeration { options } => {
                paramDefine(
                    param_set,
                    kOfxParamTypeChoice.as_ptr(),
                    descriptor_id_cstr.as_ptr(),
                    &mut paramProps,
                )
                .ofx_ok()?;
                let default_value = default_settings
                    .get_field::<EnumValue>(&descriptor.id)
                    .map_err(|_| OfxStat::kOfxStatFailed)?
                    .0;
                let mut default_idx: usize = 0;
                for (i, menu_item) in options.iter().enumerate() {
                    let item_strings = data
                        .menu_item_strings
                        .get(&(descriptor.id.clone(), menu_item.index))
                        .unwrap();
                    let item_label_cstr = item_strings.0.as_c_str();
                    propSetString(
                        paramProps,
                        kOfxParamPropChoiceOption.as_ptr(),
                        i as i32,
                        item_label_cstr.as_ptr(),
                    )
                    .ofx_ok()?;
                    if menu_item.index == default_value {
                        default_idx = i;
                    }
                }
                propSetInt(
                    paramProps,
                    kOfxParamPropDefault.as_ptr(),
                    0,
                    default_idx as i32,
                )
                .ofx_ok()?;
            }
            SettingKind::Percentage { .. } => {
                let default_value = default_settings
                    .get_field::<f32>(&descriptor.id)
                    .map_err(|_| OfxStat::kOfxStatFailed)?;
                paramDefine(
                    param_set,
                    kOfxParamTypeDouble.as_ptr(),
                    descriptor_id_cstr.as_ptr(),
                    &mut paramProps,
                )
                .ofx_ok()?;
                propSetString(
                    paramProps,
                    kOfxParamPropDoubleType.as_ptr(),
                    0,
                    kOfxParamDoubleTypeScale.as_ptr(),
                )
                .ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDefault.as_ptr(), 0, default_value as f64)
                    .ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropMin.as_ptr(), 0, 0.0).ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDisplayMin.as_ptr(), 0, 0.0).ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropMax.as_ptr(), 0, 1.0).ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDisplayMax.as_ptr(), 0, 1.0).ofx_ok()?;
            }
            SettingKind::IntRange { range } => {
                let default_value = default_settings
                    .get_field::<i32>(&descriptor.id)
                    .map_err(|_| OfxStat::kOfxStatFailed)?;
                paramDefine(
                    param_set,
                    kOfxParamTypeInteger.as_ptr(),
                    descriptor_id_cstr.as_ptr(),
                    &mut paramProps,
                )
                .ofx_ok()?;
                propSetInt(paramProps, kOfxParamPropDefault.as_ptr(), 0, default_value).ofx_ok()?;
                propSetInt(paramProps, kOfxParamPropMin.as_ptr(), 0, *range.start()).ofx_ok()?;
                propSetInt(paramProps, kOfxParamPropDisplayMin.as_ptr(), 0, *range.start())
                    .ofx_ok()?;
                propSetInt(paramProps, kOfxParamPropMax.as_ptr(), 0, *range.end()).ofx_ok()?;
                propSetInt(paramProps, kOfxParamPropDisplayMax.as_ptr(), 0, *range.end())
                    .ofx_ok()?;
            }
            SettingKind::FloatRange { range, .. } => {
                let default_value = default_settings
                    .get_field::<f32>(&descriptor.id)
                    .map_err(|_| OfxStat::kOfxStatFailed)?;
                paramDefine(
                    param_set,
                    kOfxParamTypeDouble.as_ptr(),
                    descriptor_id_cstr.as_ptr(),
                    &mut paramProps,
                )
                .ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDefault.as_ptr(), 0, default_value as f64)
                    .ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropMin.as_ptr(), 0, *range.start() as f64)
                    .ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDisplayMin.as_ptr(), 0, *range.start() as f64)
                    .ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropMax.as_ptr(), 0, *range.end() as f64)
                    .ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDisplayMax.as_ptr(), 0, *range.end() as f64)
                    .ofx_ok()?;
            }
            SettingKind::Boolean => {
                let default_value = default_settings
                    .get_field::<bool>(&descriptor.id)
                    .map_err(|_| OfxStat::kOfxStatFailed)?;
                paramDefine(
                    param_set,
                    kOfxParamTypeBoolean.as_ptr(),
                    descriptor_id_cstr.as_ptr(),
                    &mut paramProps,
                )
                .ofx_ok()?;
                propSetInt(paramProps, kOfxParamPropDefault.as_ptr(), 0, default_value as i32)
                    .ofx_ok()?;
            }
            SettingKind::Group { children } => {
                let default_value = default_settings
                    .get_field::<bool>(&descriptor.id)
                    .map_err(|_| OfxStat::kOfxStatFailed)?;
                let group_name_cstr = descriptor_strings
                    .3
                    .as_ref()
                    .expect("Group name is None")
                    .as_c_str();
                paramDefine(
                    param_set,
                    kOfxParamTypeGroup.as_ptr(),
                    group_name_cstr.as_ptr(),
                    &mut paramProps,
                )
                .ofx_ok()?;

                let mut checkboxProps: OfxPropertySetHandle = ptr::null_mut();
                paramDefine(
                    param_set,
                    kOfxParamTypeBoolean.as_ptr(),
                    descriptor_id_cstr.as_ptr(),
                    &mut checkboxProps,
                )
                .ofx_ok()?;
                propSetString(
                    checkboxProps,
                    kOfxPropLabel.as_ptr(),
                    0,
                    c"Enabled".as_ptr(),
                )
                .ofx_ok()?;
                propSetInt(
                    checkboxProps,
                    kOfxParamPropDefault.as_ptr(),
                    0,
                    default_value as i32,
                )
                .ofx_ok()?;
                propSetString(
                    checkboxProps,
                    kOfxParamPropParent.as_ptr(),
                    0,
                    group_name_cstr.as_ptr(),
                )
                .ofx_ok()?;
                propSetInt(checkboxProps, kOfxParamPropAnimates.as_ptr(), 0, 0).ofx_ok()?;

                map_params_generic::<K>(
                    data,
                    param_set,
                    children,
                    default_settings,
                    group_name_cstr,
                )?;
            }
            SettingKind::ColorRGBA { r_id, g_id, b_id, a_id } => {
                paramDefine(
                    param_set,
                    kOfxParamTypeRGBA.as_ptr(),
                    descriptor_id_cstr.as_ptr(),
                    &mut paramProps,
                )
                .ofx_ok()?;
                let dr = default_settings.get_field::<f32>(r_id).map_err(|_| OfxStat::kOfxStatFailed)? as f64;
                let dg = default_settings.get_field::<f32>(g_id).map_err(|_| OfxStat::kOfxStatFailed)? as f64;
                let db = default_settings.get_field::<f32>(b_id).map_err(|_| OfxStat::kOfxStatFailed)? as f64;
                let da = default_settings.get_field::<f32>(a_id).map_err(|_| OfxStat::kOfxStatFailed)? as f64;
                propSetDouble(paramProps, kOfxParamPropDefault.as_ptr(), 0, dr).ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDefault.as_ptr(), 1, dg).ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDefault.as_ptr(), 2, db).ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDefault.as_ptr(), 3, da).ofx_ok()?;
            }
            SettingKind::ColorRGB { r_id, g_id, b_id } => {
                paramDefine(
                    param_set,
                    kOfxParamTypeRGB.as_ptr(),
                    descriptor_id_cstr.as_ptr(),
                    &mut paramProps,
                )
                .ofx_ok()?;
                let dr = default_settings.get_field::<f32>(r_id).map_err(|_| OfxStat::kOfxStatFailed)? as f64;
                let dg = default_settings.get_field::<f32>(g_id).map_err(|_| OfxStat::kOfxStatFailed)? as f64;
                let db = default_settings.get_field::<f32>(b_id).map_err(|_| OfxStat::kOfxStatFailed)? as f64;
                propSetDouble(paramProps, kOfxParamPropDefault.as_ptr(), 0, dr).ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDefault.as_ptr(), 1, dg).ofx_ok()?;
                propSetDouble(paramProps, kOfxParamPropDefault.as_ptr(), 2, db).ofx_ok()?;
            }
        }

        if !paramProps.is_null() {
            let descriptor_strings = data.strings.get(&descriptor.id).unwrap();
            let descriptor_label_cstr = descriptor_strings.1.as_c_str();
            propSetString(
                paramProps,
                kOfxPropLabel.as_ptr(),
                0,
                descriptor_label_cstr.as_ptr(),
            )
            .ofx_ok()?;
            if let Some(description) = descriptor_strings.2.as_deref() {
                propSetString(
                    paramProps,
                    kOfxParamPropHint.as_ptr(),
                    0,
                    description.as_ptr(),
                )
                .ofx_ok()?;
            }
            propSetString(paramProps, kOfxParamPropParent.as_ptr(), 0, parent.as_ptr()).ofx_ok()?;
        }
    }

    Ok(())
}

unsafe fn apply_params_generic<K: EffectKind>(
    data: &SharedData<<K as EffectKind>::FullSettings>,
    param_set: OfxParamSetHandle,
    time: f64,
    setting_descriptors: &[SettingDescriptor<<K as EffectKind>::FullSettings>],
    dst: &mut <K as EffectKind>::FullSettings,
) -> OfxResult<()> {
    let paramGetHandle = data
        .parameter_suite
        .paramGetHandle
        .ok_or(OfxStat::kOfxStatFailed)?;
    let paramGetValueAtTime = data
        .parameter_suite
        .paramGetValueAtTime
        .ok_or(OfxStat::kOfxStatFailed)?;
    let propGetDouble = data
        .property_suite
        .propGetDouble
        .ok_or(OfxStat::kOfxStatFailed)?;

    for descriptor in setting_descriptors {
        let descriptor_strings = data.strings.get(&descriptor.id).unwrap();
        let descriptor_id_cstr = descriptor_strings.0.as_c_str();

        match &descriptor.kind {
            SettingKind::Enumeration { options } => {
                let mut param: OfxParamHandle = ptr::null_mut();
                paramGetHandle(param_set, descriptor_id_cstr.as_ptr(), &mut param, ptr::null_mut())
                    .ofx_ok()?;
                let mut selected_idx: c_int = 0;
                paramGetValueAtTime(param, time, &mut selected_idx).ofx_ok()?;
                dst.set_field::<EnumValue>(
                    &descriptor.id,
                    EnumValue(options[selected_idx as usize].index),
                )
                .unwrap();
            }
            SettingKind::Percentage { .. } => {
                let mut param: OfxParamHandle = ptr::null_mut();
                paramGetHandle(param_set, descriptor_id_cstr.as_ptr(), &mut param, ptr::null_mut())
                    .ofx_ok()?;
                let mut value: f64 = 0.0;
                paramGetValueAtTime(param, time, &mut value).ofx_ok()?;
                dst.set_field::<f32>(&descriptor.id, value as f32).unwrap();
            }
            SettingKind::IntRange { .. } => {
                let mut param: OfxParamHandle = ptr::null_mut();
                paramGetHandle(param_set, descriptor_id_cstr.as_ptr(), &mut param, ptr::null_mut())
                    .ofx_ok()?;
                let mut value: c_int = 0;
                paramGetValueAtTime(param, time, &mut value).ofx_ok()?;
                dst.set_field::<i32>(&descriptor.id, value).unwrap();
            }
            SettingKind::FloatRange { .. } => {
                let mut param: OfxParamHandle = ptr::null_mut();
                paramGetHandle(param_set, descriptor_id_cstr.as_ptr(), &mut param, ptr::null_mut())
                    .ofx_ok()?;
                let mut value: f64 = 0.0;
                paramGetValueAtTime(param, time, &mut value).ofx_ok()?;
                dst.set_field::<f32>(&descriptor.id, value as f32).unwrap();
            }
            SettingKind::Boolean => {
                let mut param: OfxParamHandle = ptr::null_mut();
                paramGetHandle(param_set, descriptor_id_cstr.as_ptr(), &mut param, ptr::null_mut())
                    .ofx_ok()?;
                let mut value: c_int = 0;
                paramGetValueAtTime(param, time, &mut value).ofx_ok()?;
                dst.set_field::<bool>(&descriptor.id, value != 0).unwrap();
            }
            SettingKind::Group { children } => {
                let mut param: OfxParamHandle = ptr::null_mut();
                paramGetHandle(param_set, descriptor_id_cstr.as_ptr(), &mut param, ptr::null_mut())
                    .ofx_ok()?;
                let mut value: c_int = 0;
                paramGetValueAtTime(param, time, &mut value).ofx_ok()?;
                dst.set_field::<bool>(&descriptor.id, value != 0).unwrap();

                apply_params_generic::<K>(data, param_set, time, children, dst)?;
            }
            SettingKind::ColorRGBA { r_id, g_id, b_id, a_id } => {
                let mut param: OfxParamHandle = ptr::null_mut();
                paramGetHandle(param_set, descriptor_id_cstr.as_ptr(), &mut param, ptr::null_mut())
                    .ofx_ok()?;
                let mut r: f64 = 0.0; let mut g: f64 = 0.0;
                let mut b: f64 = 0.0; let mut a: f64 = 0.0;
                paramGetValueAtTime(param, time, &mut r, &mut g, &mut b, &mut a).ofx_ok()?;
                dst.set_field::<f32>(r_id, r as f32).unwrap();
                dst.set_field::<f32>(g_id, g as f32).unwrap();
                dst.set_field::<f32>(b_id, b as f32).unwrap();
                dst.set_field::<f32>(a_id, a as f32).unwrap();
            }
            SettingKind::ColorRGB { r_id, g_id, b_id } => {
                let mut param: OfxParamHandle = ptr::null_mut();
                paramGetHandle(param_set, descriptor_id_cstr.as_ptr(), &mut param, ptr::null_mut())
                    .ofx_ok()?;
                let mut r: f64 = 0.0; let mut g: f64 = 0.0; let mut b: f64 = 0.0;
                paramGetValueAtTime(param, time, &mut r, &mut g, &mut b).ofx_ok()?;
                dst.set_field::<f32>(r_id, r as f32).unwrap();
                dst.set_field::<f32>(g_id, g as f32).unwrap();
                dst.set_field::<f32>(b_id, b as f32).unwrap();
            }
        }
    }

    let _ = propGetDouble;
    Ok(())
}
