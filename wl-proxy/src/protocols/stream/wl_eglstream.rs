use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_eglstream object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlEglstream {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlEglstreamHandler>,
}

impl ConcreteObject for WlEglstream {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WlEglstream;
    const INTERFACE_NAME: &str = "wl_eglstream";
}

impl WlEglstream {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlEglstreamHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlEglstreamHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlEglstream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlEglstream")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}


/// A message handler for [`WlEglstream`] proxies.
pub trait WlEglstreamHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlEglstream>) {
        slf.core.delete_id();
    }
}

impl ObjectPrivate for WlEglstream {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlEglstream, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WlEglstream {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl WlEglstream {
    /// Since when the error.bad_alloc enum variant is available.
    pub const ENM__ERROR_BAD_ALLOC__SINCE: u32 = 1;
    /// Since when the error.bad_handle enum variant is available.
    pub const ENM__ERROR_BAD_HANDLE__SINCE: u32 = 1;
    /// Since when the error.bad_attribs enum variant is available.
    pub const ENM__ERROR_BAD_ATTRIBS__SINCE: u32 = 1;
    /// Since when the error.bad_address enum variant is available.
    pub const ENM__ERROR_BAD_ADDRESS__SINCE: u32 = 1;

    /// Since when the handle_type.fd enum variant is available.
    pub const ENM__HANDLE_TYPE_FD__SINCE: u32 = 1;
    /// Since when the handle_type.inet enum variant is available.
    pub const ENM__HANDLE_TYPE_INET__SINCE: u32 = 1;
    /// Since when the handle_type.socket enum variant is available.
    pub const ENM__HANDLE_TYPE_SOCKET__SINCE: u32 = 1;

    /// Since when the attrib.inet_addr enum variant is available.
    pub const ENM__ATTRIB_INET_ADDR__SINCE: u32 = 1;
    /// Since when the attrib.inet_port enum variant is available.
    pub const ENM__ATTRIB_INET_PORT__SINCE: u32 = 1;
    /// Since when the attrib.y_inverted enum variant is available.
    pub const ENM__ATTRIB_Y_INVERTED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlEglstreamError(pub u32);

impl WlEglstreamError {
    /// Bad allocation error
    pub const BAD_ALLOC: Self = Self(0);

    /// Bad handle error
    pub const BAD_HANDLE: Self = Self(1);

    /// Bad attributes error
    pub const BAD_ATTRIBS: Self = Self(2);

    /// Bad IP address error
    pub const BAD_ADDRESS: Self = Self(3);
}

impl Debug for WlEglstreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BAD_ALLOC => "BAD_ALLOC",
            Self::BAD_HANDLE => "BAD_HANDLE",
            Self::BAD_ATTRIBS => "BAD_ATTRIBS",
            Self::BAD_ADDRESS => "BAD_ADDRESS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Stream handle type
///
/// - fd:     The given handle represents a file descriptor, and the
///           EGLStream connection must be done as described in
///           EGL_KHR_stream_cross_process_fd
///
/// - inet:   The EGLStream connection must be done using an inet address
///           and port as described in EGL_NV_stream_socket. The given
///           handle can be ignored, but both inet address and port must
///           be given as attributes.
///
/// - socket: The given handle represents a unix socket, and the EGLStream
///           connection must be done as described in EGL_NV_stream_socket.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlEglstreamHandleType(pub u32);

impl WlEglstreamHandleType {
    /// File descriptor
    pub const FD: Self = Self(0);

    /// Inet connection
    pub const INET: Self = Self(1);

    /// Unix socket
    pub const SOCKET: Self = Self(2);
}

impl Debug for WlEglstreamHandleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::FD => "FD",
            Self::INET => "INET",
            Self::SOCKET => "SOCKET",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Stream creation attributes
///
/// - inet_addr:  The given attribute encodes an IPv4 address of a client
///               socket. Both IPv4 address and port must be set at the same
///               time.
///
/// - inet_port:  The given attribute encodes a port of a client socket.
///               Both IPv4 address and port must be set at the same time.
///
/// - y_inverted: The given attribute encodes the default value for a
///               stream's image inversion relative to wayland protocol
///               convention. Vulkan apps will be set to 'true', while
///               OpenGL apps will be set to 'false'.
///               NOTE: EGL_NV_stream_origin is the authorative source of
///               truth regarding a stream's frame orientation and should be
///               queried for an accurate value. The given attribute is a
///               'best guess' fallback mechanism which should only be used
///               when a query to EGL_NV_stream_origin fails.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlEglstreamAttrib(pub u32);

impl WlEglstreamAttrib {
    /// Inet IPv4 address
    pub const INET_ADDR: Self = Self(0);

    /// IP port
    pub const INET_PORT: Self = Self(1);

    /// Image Y-inversion bit
    pub const Y_INVERTED: Self = Self(2);
}

impl Debug for WlEglstreamAttrib {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INET_ADDR => "INET_ADDR",
            Self::INET_PORT => "INET_PORT",
            Self::Y_INVERTED => "Y_INVERTED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
