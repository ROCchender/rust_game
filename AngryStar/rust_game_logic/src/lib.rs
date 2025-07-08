use serde::{Serialize, Deserialize};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

// 对应 Unity 的 Vector2
#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

// 弹簧配置（对应 Unity Inspector 面板参数）
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct SpringConfig {
    pub anchor: Vector2,
    pub connected_anchor: Vector2,
    pub distance: f32,
    pub damping_ratio: f32,
    pub frequency: f32,
    pub break_force: f32,
}

// 刚体物理状态
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct RigidbodyState {
    pub position: Vector2,
    pub velocity: Vector2,
    pub mass: f32,
}

// 计算结果
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct SpringResult {
    pub force: Vector2,
    pub should_break: bool,
}

/// 核心计算函数（完全匹配Unity的SpringJoint2D物理）
#[no_mangle]
pub extern "C" fn calculate_spring_force(
    self_state_json: *const c_char,
    connected_state_json: *const c_char,
    config_json: *const c_char,
) -> *mut c_char {
    // 反序列化输入数据
    let self_state: RigidbodyState = unsafe {
        serde_json::from_str(
            CStr::from_ptr(self_state_json).to_str().unwrap()
        ).unwrap()
    };

    let connected_state: RigidbodyState = unsafe {
        serde_json::from_str(
            CStr::from_ptr(connected_state_json).to_str().unwrap()
        ).unwrap()
    };

    let config: SpringConfig = unsafe {
        serde_json::from_str(
            CStr::from_ptr(config_json).to_str().unwrap()
        ).unwrap()
    };

    // 1. 计算世界空间锚点位置
    let self_anchor = Vector2 {
        x: self_state.position.x + config.anchor.x,
        y: self_state.position.y + config.anchor.y,
    };
    let connected_anchor = Vector2 {
        x: connected_state.position.x + config.connected_anchor.x,
        y: connected_state.position.y + config.connected_anchor.y,
    };

    // 2. 计算方向向量和距离
    let delta = Vector2 {
        x: connected_anchor.x - self_anchor.x,
        y: connected_anchor.y - self_anchor.y,
    };
    let distance = (delta.x.powi(2) + delta.y.powi(2)).sqrt();

    // 3. 计算弹簧参数（完全匹配Unity算法）
    let stiffness = (2.0 * std::f32::consts::PI * config.frequency).powi(2) * self_state.mass;
    let damping = 2.0 * (stiffness * self_state.mass).sqrt() * config.damping_ratio;

    // 4. 计算弹性力和阻尼力
    let mut force = Vector2 { x: 0.0, y: 0.0 };
    if distance > 0.0 {
        let direction = Vector2 {
            x: delta.x / distance,
            y: delta.y / distance,
        };
        let displacement = distance - config.distance;
        let relative_vel = Vector2 {
            x: connected_state.velocity.x - self_state.velocity.x,
            y: connected_state.velocity.y - self_state.velocity.y,
        };

        force.x = direction.x * (stiffness * displacement + damping * relative_vel.x);
        force.y = direction.y * (stiffness * displacement + damping * relative_vel.y);
    }

    // 5. 断裂检测
    let should_break = (stiffness * (distance - config.distance)).abs() > config.break_force 
        && config.break_force.is_finite();

    // 返回结果
    let result = SpringResult { force, should_break };
    CString::new(serde_json::to_string(&result).unwrap()).unwrap().into_raw()
}

/// 内存释放函数
#[no_mangle]
pub extern "C" fn free_cstring(ptr: *mut c_char) {
    unsafe { let _ = CString::from_raw(ptr); };
}