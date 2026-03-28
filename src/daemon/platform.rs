#[cfg(target_os = "windows")]
pub fn get_pid() -> u32 {
	use winapi::um::winuser;

	let mut pid: u32 = 0;

	unsafe {
		let hwnd = winuser::GetForegroundWindow();
		winuser::GetWindowThreadProcessId(hwnd, &mut pid);
	}

	return pid
}

#[cfg(target_os = "linux")]
pub fn get_pid() -> u32 {
	if std::env::var("WAYLAND_DISPLAY").is_ok() {
		return get_pid_wayland();
	}

	return get_pid_x11();
}

#[cfg(target_os = "linux")]
fn get_pid_wayland() -> u32 {
	// only works on KDE
    return kdotool::get_active_window_info().map(|i| i.pid).unwrap_or(0);
}

#[cfg(target_os = "linux")]
fn get_pid_x11() -> u32 {
    let (conn, _) = match xcb::Connection::connect(None) {
        Ok(c) => c,
        Err(_) => {
            crate::log!("Failed to connect to X server");
            return 0;
        }
    };

    let focus_cookie = conn.send_request(&xcb::x::GetInputFocus {});
    let focus_reply = match conn.wait_for_reply(focus_cookie) {
        Ok(r) => r,
        Err(_) => return 0,
    };
    let window = focus_reply.focus();

    let atom_cookie = conn.send_request(&xcb::x::InternAtom {
        only_if_exists: true,
        name: b"_NET_WM_PID",
    });
    let atom_reply = match conn.wait_for_reply(atom_cookie) {
        Ok(r) => r,
        Err(_) => return 0,
    };
    let pid_atom = atom_reply.atom();

    let prop_cookie = conn.send_request(&xcb::x::GetProperty {
        delete: false,
        window,
        property: pid_atom,
        r#type: xcb::x::ATOM_CARDINAL,
        long_offset: 0,
        long_length: 1,
    });

    match conn.wait_for_reply(prop_cookie) {
        Ok(reply) => {
            let value: &[u32] = reply.value();
            value.get(0).copied().unwrap_or(0)
        }
        Err(_) => 0,
    }
}