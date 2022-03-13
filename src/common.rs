use crate::public::*;
pub use std::{
    collections::hash_map::HashMap,
    sync::atomic::{AtomicPtr, Ordering},
    sync::{Arc, Mutex},
    thread::spawn,
};
use once_cell::sync::Lazy;

struct NormalBind {
    BlockBindHandler value;
};

struct BlockBind {
    BlockBindHandler value;
};

struct BlockableBindHandler {
    BlockableBindHandler value;
};

using Bind = std::variant<NormalBind, BlockBind, BlockableBindHandler>;

using BindHandler = std::shared_ptr<std::function<void()>>;
using BlockBindHandler = std::shared_ptr<std::function<void()>>;
using BlockableBindHandler = std::shared_ptr<std::function<BlockInput()>>;
using KeybdBindMap = std::unordered_map<KeybdKey, Bind>;
using MouseBindMap = std::unordered_map<MouseButton, Bind>;

boost::synchronized_value<KeybdBindMap>& get_keybd_binds() {
    static boost::synchronized_value<KeybdBindMap> keybd_binds;
    return keybd_binds;
}

boost::synchronized_value<MouseBindMap>& get_mouse_binds() {
    static boost::synchronized_value<MouseBindMap> mouse_binds;
    return mouse_binds;
}
