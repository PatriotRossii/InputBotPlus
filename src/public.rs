use crate::common::*;
use std::{thread::sleep, time::Duration};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#include <cctype>

enum BlockInput {
    Block,
    DontBlock
};

struct KeybdKey {
    enum class Type {
        Backspace,
        Tab,
        Enter,
        Escape,
        Space,
        Home,
        Left,
        Up,
        Right,
        Down,
        Insert,
        Delete,
        Numrow0,
        Numrow1,
        Numrow2,
        Numrow3,
        Numrow4,
        Numrow5,
        Numrow6,
        Numrow7,
        Numrow8,
        Numrow9,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
        Numpad0,
        Numpad1,
        Numpad2,
        Numpad3,
        Numpad4,
        Numpad5,
        Numpad6,
        Numpad7,
        Numpad8,
        Numpad9,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        F13,
        F14,
        F15,
        F16,
        F17,
        F18,
        F19,
        F20,
        F21,
        F22,
        F23,
        F24,
        NumLock,
        ScrollLock,
        CapsLock,
        LShift,
        RShift,
        LControl,
        RControl,
        End_
    } type;

    KeybdKey(KeybdKey::Type type)
        : type{type} { }
    operator Type() {
        return type;
    }
};

struct MouseButton {
    enum class Type {
        Left,
        Middle,
        Right,
        X1,
        X2,
        End_
    } type;

    MouseButton(MouseButton::Type type)
        : type{type} { }
    operator Type() {
        return type;
    }
};

struct MouseCursor { };

struct MouseWheel { };

void bind(const KeybdKey& key, std::function<void()> callback) {
    get_keybd_binds()
        .synchronize()
        ->insert({
            key, Bind{NormalBind{std::make_shared_ptr(callback)}
        });
}

void block_bind(const KeybdKey& key, std::function<void()> callback) {
    get_keybd_binds()
        .synchronize()
        ->insert({
            key, Bind{BlockBind{std::make_shared_ptr(callback)}
        });
}

void blockable_bind(const KeybdKey& key, std::function<BlockInput()> callback) {
    get_keybd_binds()
        .synchronize()
        ->insert({
            key, Bind{BlockableBind{std::make_shared_ptr(callback)}}
        });
}

void bind_all(std::function<void(KeybdKey)> callback) {
    for(KeybdKey key = static_cast<KeybdKey>(0); key != KeybdKey::End_; key = static_cast<KeybdKey>(static_cast<int>(key) + 1)) {
        auto fire = std::function([callback, key]() {
            callback(key)
        });
        get_keybd_binds()
            .synchronize()
            ->insert({
                key, Bind{NormalBind{std::make_shared_ptr(callback)}
            });
    }
}

void unbind(const KeybdKey& key) {
    get_keybd_binds()
        .synchronize()
        ->erase(key);
}

void bind(const MouseButton& button, std::function<void()> callback) {
    get_mouse_binds()
        .synchronize()
        ->insert({
            key, Bind{NormalBind{std::make_shared_ptr(callback)}
        });
}

void block_bind(const MouseButton& button, std::function<void()> callback) {
    get_mouse_binds()
        .synchronize()
        ->insert({
            key, Bind{BlockBind{std::make_shared_ptr(callback)}
        });
}

void blockable_bind(const MouseButton& button, std::function<BlockInput()> callback) {
    get_mouse_binds()
        .synchronize()
        ->insert({
            key, Bind{BlockableBind{std::make_shared_ptr(callback)}}
        });
}

void unbind(const MouseButton& button) {
    get_mouse_binds()
        .synchronize()
        ->erase(key);
}

char from_keybd_key(KeybdKey key) {
    switch(static_cast<KeybdKey::Type>(key)) {
        case KeybdKey::Type::A:
            return 'a';
        case KeybdKey::Type::B:
			return 'b';
        case KeybdKey::Type::C:
			return 'c';
        case KeybdKey::Type::D:
			return 'd';
        case KeybdKey::Type::E:
			return 'e';
        case KeybdKey::Type::F:
			return 'f';
        case KeybdKey::Type::G:
			return 'g';
        case KeybdKey::Type::H:
			return 'h';
        case KeybdKey::Type::I:
			return 'i';
        case KeybdKey::Type::J:
			return 'j';
        case KeybdKey::Type::K:
			return 'k';
        case KeybdKey::Type::L:
			return 'l';
        case KeybdKey::Type::M:
			return 'm';
        case KeybdKey::Type::N:
			return 'n';
        case KeybdKey::Type::O:
			return 'o';
        case KeybdKey::Type::P:
			return 'p';
        case KeybdKey::Type::Q:
			return 'q';
        case KeybdKey::Type::R:
			return 'r';
        case KeybdKey::Type::S:
			return 's';
        case KeybdKey::Type::T:
			return 't';
        case KeybdKey::Type::U:
			return 'u';
        case KeybdKey::Type::V:
			return 'v';
        case KeybdKey::Type::W:
			return 'w';
        case KeybdKey::Type::X:
			return 'x';
        case KeybdKey::Type::Y:
			return 'y';
        case KeybdKey::Type::Z:
			return 'z';
        case KeybdKey::Type::Numpad0:
			return '0';
        case KeybdKey::Type::Numpad1:
			return '1';
        case KeybdKey::Type::Numpad2:
			return '2';
        case KeybdKey::Type::Numpad3:
			return '3';
        case KeybdKey::Type::Numpad4:
			return '4';
        case KeybdKey::Type::Numpad5:
			return '5';
        case KeybdKey::Type::Numpad6:
			return '6';
        case KeybdKey::Type::Numpad7:
			return '7';
        case KeybdKey::Type::Numpad8:
			return '8';
        case KeybdKey::Type::Numpad9:
			return '9';
        case KeybdKey::Type::Numrow0:
			return '0';
        case KeybdKey::Type::Numrow1:
			return '1';
        case KeybdKey::Type::Numrow2:
			return '2';
        case KeybdKey::Type::Numrow3:
			return '3';
        case KeybdKey::Type::Numrow4:
			return '4';
        case KeybdKey::Type::Numrow5:
			return '5';
        case KeybdKey::Type::Numrow6:
			return '6';
        case KeybdKey::Type::Numrow7:
			return '7';
        case KeybdKey::Type::Numrow8:
			return '8';
        case KeybdKey::Type::Numrow9:
			return '9';
        default:
            return '';
    }
}

KeybdKey get_keybd_key(char c) {
    switch(c) {
        case ' ':
            return KeybdKey::Type::Space;
        case 'A':
			return KeybdKey::Type::A;
		case 'a':
			return KeybdKey::Type::A;
        case 'B':
			return KeybdKey::Type::B;
		case 'b':
			return KeybdKey::Type::B;
        case 'C':
			return KeybdKey::Type::C;
		case 'c':
			return KeybdKey::Type::C;
        case 'D':
			return KeybdKey::Type::D;
		case 'd':
			return KeybdKey::Type::D;
        case 'E':
			return KeybdKey::Type::E;
		case 'e':
			return KeybdKey::Type::E;
        case 'F':
			return KeybdKey::Type::F;
		case 'f':
			return KeybdKey::Type::F;
        case 'G':
			return KeybdKey::Type::G;
		case 'g':
			return KeybdKey::Type::G;
        case 'H':
			return KeybdKey::Type::H;
		case 'h':
			return KeybdKey::Type::H;
        case 'I':
			return KeybdKey::Type::I;
		case 'i':
			return KeybdKey::Type::I;
        case 'J':
			return KeybdKey::Type::J;
		case 'j':
			return KeybdKey::Type::J;
        case 'K':
			return KeybdKey::Type::K;
		case 'k':
			return KeybdKey::Type::K;
        case 'L':
			return KeybdKey::Type::L;
		case 'l':
			return KeybdKey::Type::L;
        case 'M':
			return KeybdKey::Type::M;
		case 'm':
			return KeybdKey::Type::M;
        case 'N':
			return KeybdKey::Type::N;
		case 'n':
			return KeybdKey::Type::N;
        case 'O':
			return KeybdKey::Type::O;
		case 'o':
			return KeybdKey::Type::O;
        case 'P':
			return KeybdKey::Type::P;
		case 'p':
			return KeybdKey::Type::P;
        case 'Q':
			return KeybdKey::Type::Q;
		case 'q':
			return KeybdKey::Type::Q;
        case 'R':
			return KeybdKey::Type::R;
		case 'r':
			return KeybdKey::Type::R;
        case 'S':
			return KeybdKey::Type::S;
		case 's':
			return KeybdKey::Type::S;
        case 'T':
			return KeybdKey::Type::T;
		case 't':
			return KeybdKey::Type::T;
        case 'U':
			return KeybdKey::Type::U;
		case 'u':
			return KeybdKey::Type::U;
        case 'V':
			return KeybdKey::Type::V;
		case 'v':
			return KeybdKey::Type::V;
        case 'W':
			return KeybdKey::Type::W;
		case 'w':
			return KeybdKey::Type::W;
        case 'X':
			return KeybdKey::Type::X;
		case 'x':
			return KeybdKey::Type::X;
        case 'Y':
			return KeybdKey::Type::Y;
		case 'y':
			return KeybdKey::Type::Y;
        case 'Z':
			return KeybdKey::Type::Z;
		case 'z':
			return KeybdKey::Type::Z;
        default:
            throw std::runtime_error{
                "There are no such keyboard key"
            };
    }
}

struct KeySequence {
    std::string sequence;
    void send() const {
        for(const char& x: sequence) {
            bool uppercase = std::isupper(x);
            auto keybd_key = get_keybd_key(x);
            
            if(uppercase) {
                press(KeybdKey::Type::LShift);
            }

            press(keybd_key);
            sleep(20ms);
            release(Keybd_key);

            if(uppercase) {
                release(KeybdKey::Type::LShift);
            }
        }
    }
};